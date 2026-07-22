use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, fs, path::PathBuf, time::{Instant, SystemTime, UNIX_EPOCH}};
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
struct Metrics {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    available_memory: u64,
    platform: String,
    architecture: String,
    cpu_brand: String,
    logical_cores: usize,
    gpu_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Connection {
    id: String,
    name: String,
    kind: String,
    base_url: String,
    #[serde(default)]
    api_key: String,
    #[serde(default)]
    default_model: String,
    enabled: bool,
    #[serde(default)]
    input_cost_per_million: f64,
    #[serde(default)]
    output_cost_per_million: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatResult {
    connection_id: String,
    connection_name: String,
    model: String,
    response: String,
    input_tokens: u64,
    output_tokens: u64,
    total_tokens: u64,
    estimated_cost: f64,
    latency_ms: f64,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize)]
struct UsageConnection {
    connection_id: String,
    connection_name: String,
    requests: u64,
    input_tokens: u64,
    output_tokens: u64,
    total_tokens: u64,
    estimated_cost: f64,
}

#[derive(Debug, Clone, Serialize)]
struct UsageSummary {
    total_requests: u64,
    input_tokens: u64,
    output_tokens: u64,
    total_tokens: u64,
    estimated_cost: f64,
    by_connection: Vec<UsageConnection>,
    recent: Vec<ChatResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ChatMessage { role: String, content: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppSettings {
    theme: String,
    automatic_fallback: bool,
    context_token_budget: usize,
    history_limit: usize,
}
impl Default for AppSettings {
    fn default() -> Self { Self { theme: "dark".into(), automatic_fallback: true, context_token_budget: 12000, history_limit: 1000 } }
}

fn app_dir() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(std::env::temp_dir);
    let path = base.join("Forge AI");
    let _ = fs::create_dir_all(&path);
    path
}

fn connections_path() -> PathBuf { app_dir().join("connections.json") }
fn usage_path() -> PathBuf { app_dir().join("usage.json") }
fn conversation_path() -> PathBuf { app_dir().join("conversation.json") }
fn settings_path() -> PathBuf { app_dir().join("settings.json") }
fn credential(id: &str) -> Result<keyring::Entry, String> { keyring::Entry::new("Forge AI", id).map_err(|e| e.to_string()) }
fn hydrate_key(connection: &mut Connection) { connection.api_key = credential(&connection.id).and_then(|e| e.get_password().map_err(|x| x.to_string())).unwrap_or_default(); }

fn read_json<T: for<'de> Deserialize<'de> + Default>(path: PathBuf) -> T {
    fs::read_to_string(path).ok().and_then(|v| serde_json::from_str(&v).ok()).unwrap_or_default()
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) -> Result<(), String> {
    let body = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(path, body).map_err(|e| e.to_string())
}

fn epoch() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

#[tauri::command]
fn get_system_metrics() -> Metrics {
    let mut system = System::new_all();
    system.refresh_all();
    let cpu_brand = system.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_default();
    Metrics {
        cpu_usage: system.global_cpu_usage(),
        total_memory: system.total_memory(),
        used_memory: system.used_memory(),
        available_memory: system.available_memory(),
        platform: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        cpu_brand,
        logical_cores: system.cpus().len(),
        gpu_name: if cfg!(target_os = "macos") { "Apple Metal compatible".into() } else { "System GPU".into() },
    }
}

#[tauri::command]
fn list_connections() -> Vec<Connection> { let mut items: Vec<Connection> = read_json(connections_path()); for item in &mut items { item.api_key.clear(); } items }

#[tauri::command]
fn save_connection(connection: Connection) -> Result<(), String> {
    let mut items: Vec<Connection> = read_json(connections_path());
    let mut connection = connection;
    if !connection.api_key.is_empty() { credential(&connection.id)?.set_password(&connection.api_key).map_err(|e| e.to_string())?; }
    connection.api_key.clear();
    if let Some(existing) = items.iter_mut().find(|item| item.id == connection.id) {
        *existing = connection;
    } else {
        items.push(connection);
    }
    write_json(connections_path(), &items)
}

#[tauri::command]
fn delete_connection(id: String) -> Result<(), String> {
    let mut items: Vec<Connection> = read_json(connections_path());
    items.retain(|item| item.id != id);
    let _ = credential(&id).and_then(|e| e.delete_credential().map_err(|x| x.to_string()));
    write_json(connections_path(), &items)
}

fn normalized(url: &str) -> String { url.trim_end_matches('/').to_string() }

async fn provider_request(connection: &Connection, model: &str, prompt: &str) -> Result<(String, u64, u64), String> {
    let client = reqwest::Client::new();
    match connection.kind.as_str() {
        "local_ollama" | "remote_ollama" => {
            let response = client.post(format!("{}/api/generate", normalized(&connection.base_url)))
                .json(&json!({"model": model, "prompt": prompt, "stream": false}))
                .send().await.map_err(|e| e.to_string())?;
            let status = response.status();
            let value: Value = response.json().await.map_err(|e| e.to_string())?;
            if !status.is_success() { return Err(value.to_string()); }
            Ok((value["response"].as_str().unwrap_or_default().to_string(), value["prompt_eval_count"].as_u64().unwrap_or(0), value["eval_count"].as_u64().unwrap_or(0)))
        }
        "anthropic" => {
            let response = client.post(format!("{}/messages", normalized(&connection.base_url)))
                .header("x-api-key", &connection.api_key)
                .header("anthropic-version", "2023-06-01")
                .json(&json!({"model": model, "max_tokens": 2048, "messages": [{"role": "user", "content": prompt}]}))
                .send().await.map_err(|e| e.to_string())?;
            let status = response.status();
            let value: Value = response.json().await.map_err(|e| e.to_string())?;
            if !status.is_success() { return Err(value.to_string()); }
            Ok((value["content"][0]["text"].as_str().unwrap_or_default().to_string(), value["usage"]["input_tokens"].as_u64().unwrap_or(0), value["usage"]["output_tokens"].as_u64().unwrap_or(0)))
        }
        "gemini" => {
            let url = format!("{}/models/{}:generateContent?key={}", normalized(&connection.base_url), model, connection.api_key);
            let response = client.post(url).json(&json!({"contents": [{"parts": [{"text": prompt}]}]})).send().await.map_err(|e| e.to_string())?;
            let status = response.status();
            let value: Value = response.json().await.map_err(|e| e.to_string())?;
            if !status.is_success() { return Err(value.to_string()); }
            Ok((value["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or_default().to_string(), value["usageMetadata"]["promptTokenCount"].as_u64().unwrap_or(0), value["usageMetadata"]["candidatesTokenCount"].as_u64().unwrap_or(0)))
        }
        _ => {
            let mut request = client.post(format!("{}/chat/completions", normalized(&connection.base_url)))
                .json(&json!({"model": model, "messages": [{"role": "user", "content": prompt}]}));
            if !connection.api_key.is_empty() { request = request.bearer_auth(&connection.api_key); }
            let response = request.send().await.map_err(|e| e.to_string())?;
            let status = response.status();
            let value: Value = response.json().await.map_err(|e| e.to_string())?;
            if !status.is_success() { return Err(value.to_string()); }
            Ok((value["choices"][0]["message"]["content"].as_str().unwrap_or_default().to_string(), value["usage"]["prompt_tokens"].as_u64().unwrap_or(0), value["usage"]["completion_tokens"].as_u64().unwrap_or(0)))
        }
    }
}

#[tauri::command]
async fn test_connection(connection: Connection) -> Result<String, String> {
    let mut connection = connection;
    if connection.api_key.is_empty() { hydrate_key(&mut connection); }
    let client = reqwest::Client::new();
    let url = if connection.kind.contains("ollama") { format!("{}/api/tags", normalized(&connection.base_url)) } else { connection.base_url.clone() };
    let mut request = client.get(url);
    if !connection.api_key.is_empty() && !connection.kind.contains("ollama") { request = request.bearer_auth(&connection.api_key); }
    let response = request.send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() { Ok("Connected".into()) } else { Err(format!("Connection returned {}", response.status())) }
}

#[tauri::command]
async fn discover_models(connection: Connection) -> Result<Vec<String>, String> {
    let mut connection = connection; if connection.api_key.is_empty() { hydrate_key(&mut connection); }
    let client = reqwest::Client::new();
    let url = match connection.kind.as_str() {
        "local_ollama" | "remote_ollama" => format!("{}/api/tags", normalized(&connection.base_url)),
        "gemini" => format!("{}/models?key={}", normalized(&connection.base_url), connection.api_key),
        _ => format!("{}/models", normalized(&connection.base_url)),
    };
    let mut request = client.get(url); if !connection.api_key.is_empty() && connection.kind != "gemini" { request = request.bearer_auth(&connection.api_key); }
    let response = request.send().await.map_err(|e| e.to_string())?; let status = response.status();
    let value: Value = response.json().await.map_err(|e| e.to_string())?; if !status.is_success() { return Err(value.to_string()); }
    let source = if connection.kind.contains("ollama") { value["models"].as_array() } else { value["data"].as_array().or_else(|| value["models"].as_array()) };
    let mut models: Vec<String> = source.into_iter().flatten().filter_map(|v| v["id"].as_str().or_else(|| v["name"].as_str()).map(|s| s.trim_start_matches("models/").to_string())).collect();
    models.sort(); models.dedup(); Ok(models)
}

#[tauri::command]
async fn send_chat(connection_id: String, model: String, prompt: String) -> Result<ChatResult, String> {
    let connections: Vec<Connection> = read_json(connections_path());
    let mut connection = connections.into_iter().find(|item| item.id == connection_id).ok_or("Connection not found")?; hydrate_key(&mut connection);
    let started = Instant::now();
    let (response, input_tokens, output_tokens) = provider_request(&connection, &model, &prompt).await?;
    let estimated_cost = input_tokens as f64 / 1_000_000.0 * connection.input_cost_per_million + output_tokens as f64 / 1_000_000.0 * connection.output_cost_per_million;
    let result = ChatResult { connection_id: connection.id, connection_name: connection.name, model, response, input_tokens, output_tokens, total_tokens: input_tokens + output_tokens, estimated_cost, latency_ms: started.elapsed().as_secs_f64() * 1000.0, created_at: epoch() };
    let mut history: Vec<ChatResult> = read_json(usage_path());
    history.push(result.clone());
    if history.len() > 1000 { history.drain(..history.len() - 1000); }
    write_json(usage_path(), &history)?;
    Ok(result)
}

#[tauri::command]
async fn send_chat_with_fallback(connection_ids: Vec<String>, model: String, prompt: String) -> Result<ChatResult, String> {
    let settings: AppSettings = read_json(settings_path()); let all: Vec<Connection> = read_json(connections_path()); let mut errors = Vec::new();
    let primary_id = connection_ids.first().cloned().unwrap_or_default();
    let ids = if settings.automatic_fallback { connection_ids } else { connection_ids.into_iter().take(1).collect() };
    for id in ids { if let Some(mut c) = all.iter().find(|x| x.id == id && x.enabled).cloned() { hydrate_key(&mut c); let selected_model = if id == primary_id && !model.is_empty() { model.clone() } else { c.default_model.clone() }; let started=Instant::now(); match provider_request(&c,&selected_model,&prompt).await { Ok((response,input_tokens,output_tokens)) => { let estimated_cost=input_tokens as f64/1_000_000.0*c.input_cost_per_million+output_tokens as f64/1_000_000.0*c.output_cost_per_million; let result=ChatResult{connection_id:c.id,connection_name:c.name,model:selected_model,response,input_tokens,output_tokens,total_tokens:input_tokens+output_tokens,estimated_cost,latency_ms:started.elapsed().as_secs_f64()*1000.0,created_at:epoch()}; let mut history:Vec<ChatResult>=read_json(usage_path()); history.push(result.clone()); if history.len()>settings.history_limit { history.drain(..history.len()-settings.history_limit); } write_json(usage_path(),&history)?; return Ok(result) }, Err(e)=>errors.push(format!("{}: {}",c.name,e)) } } }
    Err(format!("All providers failed: {}", errors.join(" | ")))
}

#[tauri::command] fn get_conversation() -> Vec<ChatMessage> { read_json(conversation_path()) }
#[tauri::command] fn save_conversation(messages: Vec<ChatMessage>) -> Result<(),String> { write_json(conversation_path(),&messages) }
#[tauri::command] fn clear_conversation() -> Result<(),String> { write_json(conversation_path(),&Vec::<ChatMessage>::new()) }
#[tauri::command] fn get_settings() -> AppSettings { read_json(settings_path()) }
#[tauri::command] fn save_settings(settings: AppSettings) -> Result<(),String> { write_json(settings_path(),&settings) }

#[tauri::command]
fn get_usage_summary() -> UsageSummary {
    let history: Vec<ChatResult> = read_json(usage_path());
    let mut grouped: HashMap<String, UsageConnection> = HashMap::new();
    for item in &history {
        let row = grouped.entry(item.connection_id.clone()).or_insert(UsageConnection { connection_id: item.connection_id.clone(), connection_name: item.connection_name.clone(), requests: 0, input_tokens: 0, output_tokens: 0, total_tokens: 0, estimated_cost: 0.0 });
        row.requests += 1; row.input_tokens += item.input_tokens; row.output_tokens += item.output_tokens; row.total_tokens += item.total_tokens; row.estimated_cost += item.estimated_cost;
    }
    UsageSummary { total_requests: history.len() as u64, input_tokens: history.iter().map(|x| x.input_tokens).sum(), output_tokens: history.iter().map(|x| x.output_tokens).sum(), total_tokens: history.iter().map(|x| x.total_tokens).sum(), estimated_cost: history.iter().map(|x| x.estimated_cost).sum(), by_connection: grouped.into_values().collect(), recent: history.into_iter().rev().take(50).collect() }
}

#[tauri::command]
fn clear_usage_history() -> Result<(), String> { write_json(usage_path(), &Vec::<ChatResult>::new()) }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_system_metrics, list_connections, save_connection, delete_connection, test_connection, discover_models, send_chat, send_chat_with_fallback, get_conversation, save_conversation, clear_conversation, get_settings, save_settings, get_usage_summary, clear_usage_history])
        .run(tauri::generate_context!())
        .expect("error while running Forge AI");
}

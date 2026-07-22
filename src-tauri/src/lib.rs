use keyring::Entry;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, fs, path::PathBuf, time::{Instant, SystemTime, UNIX_EPOCH}};
use sysinfo::System;

const KEYRING_SERVICE: &str = "com.forge-ai.desktop";

#[derive(Debug, Clone, Serialize)]
struct Metrics { cpu_usage: f32, total_memory: u64, used_memory: u64, available_memory: u64, platform: String, architecture: String, cpu_brand: String, logical_cores: usize, gpu_name: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Connection {
    id: String, name: String, kind: String, base_url: String,
    #[serde(default, skip_serializing)] api_key: String,
    #[serde(default)] has_api_key: bool,
    #[serde(default)] default_model: String,
    #[serde(default = "enabled_default")] enabled: bool,
    #[serde(default)] input_cost_per_million: f64,
    #[serde(default)] output_cost_per_million: f64,
}
fn enabled_default() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage { role: String, content: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Conversation { id: String, title: String, connection_id: String, model: String, messages: Vec<ChatMessage>, updated_at: u64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    theme: String, automatic_fallback: bool, context_token_budget: u64,
    context_message_limit: usize, fallback_connection_ids: Vec<String>,
}
impl Default for Settings { fn default() -> Self { Self { theme: "dark".into(), automatic_fallback: true, context_token_budget: 12_000, context_message_limit: 24, fallback_connection_ids: vec![] } } }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatResult { connection_id: String, connection_name: String, model: String, response: String, input_tokens: u64, output_tokens: u64, total_tokens: u64, estimated_cost: f64, latency_ms: f64, created_at: u64, #[serde(default)] fallback_attempts: Vec<String> }

#[derive(Debug, Clone, Serialize)]
struct UsageConnection { connection_id: String, connection_name: String, requests: u64, input_tokens: u64, output_tokens: u64, total_tokens: u64, estimated_cost: f64 }
#[derive(Debug, Clone, Serialize)]
struct UsageModel { connection_id: String, provider_name: String, model: String, requests: u64, input_tokens: u64, output_tokens: u64, total_tokens: u64, estimated_cost: f64, remaining_tokens: Option<u64> }
#[derive(Debug, Clone, Serialize)]
struct UsageSummary { total_requests: u64, input_tokens: u64, output_tokens: u64, total_tokens: u64, estimated_cost: f64, by_connection: Vec<UsageConnection>, by_model: Vec<UsageModel>, recent: Vec<ChatResult> }

fn app_dir() -> PathBuf { let base = dirs::data_local_dir().unwrap_or_else(std::env::temp_dir); let path = base.join("Forge AI"); let _ = fs::create_dir_all(&path); path }
fn path(name: &str) -> PathBuf { app_dir().join(name) }
fn read_json<T: for<'de> Deserialize<'de> + Default>(path: PathBuf) -> T { fs::read_to_string(path).ok().and_then(|v| serde_json::from_str(&v).ok()).unwrap_or_default() }
fn write_json<T: Serialize>(path: PathBuf, value: &T) -> Result<(), String> { fs::write(path, serde_json::to_string_pretty(value).map_err(|e| e.to_string())?).map_err(|e| e.to_string()) }
fn epoch() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }
fn normalized(url: &str) -> String { url.trim().trim_end_matches('/').to_string() }
fn credential(id: &str) -> Result<Entry, String> { Entry::new(KEYRING_SERVICE, id).map_err(|e| format!("Credential storage unavailable: {e}")) }
fn hydrate_key(connection: &mut Connection) { connection.api_key = credential(&connection.id).ok().and_then(|e| e.get_password().ok()).unwrap_or_default(); connection.has_api_key = !connection.api_key.is_empty(); }
fn public_connection(mut connection: Connection) -> Connection { connection.api_key.clear(); connection }

#[tauri::command]
fn get_system_metrics() -> Metrics { let mut s = System::new_all(); s.refresh_all(); Metrics { cpu_usage: s.global_cpu_usage(), total_memory: s.total_memory(), used_memory: s.used_memory(), available_memory: s.available_memory(), platform: std::env::consts::OS.into(), architecture: std::env::consts::ARCH.into(), cpu_brand: s.cpus().first().map(|c| c.brand().into()).unwrap_or_default(), logical_cores: s.cpus().len(), gpu_name: if cfg!(target_os="macos") { "Apple Metal compatible".into() } else { "System GPU".into() } } }

#[tauri::command]
fn list_connections() -> Vec<Connection> { read_json::<Vec<Connection>>(path("connections.json")).into_iter().map(public_connection).collect() }

#[tauri::command]
fn save_connection(mut connection: Connection) -> Result<(), String> {
    let name = connection.name.trim().to_string(); let base = connection.base_url.trim().to_string(); let model = connection.default_model.trim().to_string();
    if name.is_empty() { return Err("Connection name is required".into()); }
    let parsed = reqwest::Url::parse(&base).map_err(|_| "Enter a valid http:// or https:// base URL")?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" { return Err("Base URL must use http:// or https://".into()); }
    if model.is_empty() { return Err("A default model is required".into()); }
    if !connection.input_cost_per_million.is_finite() || connection.input_cost_per_million < 0.0 || !connection.output_cost_per_million.is_finite() || connection.output_cost_per_million < 0.0 { return Err("Token pricing must be zero or a positive number".into()); }
    let mut items: Vec<Connection> = read_json(path("connections.json"));
    let old_key_in_json = items.iter().find(|x| x.id == connection.id).map(|x| x.api_key.clone()).unwrap_or_default();
    if !connection.api_key.is_empty() { credential(&connection.id)?.set_password(&connection.api_key).map_err(|e| format!("Could not save API key securely: {e}"))?; connection.has_api_key = true; }
    else if !old_key_in_json.is_empty() { credential(&connection.id)?.set_password(&old_key_in_json).map_err(|e| format!("Could not migrate API key securely: {e}"))?; connection.has_api_key = true; }
    connection.name = name; connection.base_url = normalized(&base); connection.default_model = model; connection.api_key.clear();
    if let Some(existing) = items.iter_mut().find(|x| x.id == connection.id) { *existing = connection; } else { items.push(connection); }
    for item in &mut items { item.api_key.clear(); }
    write_json(path("connections.json"), &items)
}

#[tauri::command]
fn delete_connection(id: String) -> Result<(), String> { let mut items: Vec<Connection> = read_json(path("connections.json")); items.retain(|x| x.id != id); if let Ok(e)=credential(&id) { let _=e.delete_credential(); } write_json(path("connections.json"), &items) }

fn auth_request(client: &reqwest::Client, connection: &Connection, url: String) -> reqwest::RequestBuilder { let request=client.get(url); if connection.api_key.is_empty() || connection.kind.contains("ollama") || connection.kind=="gemini" { request } else if connection.kind=="anthropic" { request.header("x-api-key", &connection.api_key).header("anthropic-version", "2023-06-01") } else { request.bearer_auth(&connection.api_key) } }

#[tauri::command]
async fn discover_models(mut connection: Connection) -> Result<Vec<String>, String> {
    hydrate_key(&mut connection); let base=normalized(&connection.base_url); let client=reqwest::Client::new();
    let url=match connection.kind.as_str() { "local_ollama"|"remote_ollama"=>format!("{base}/api/tags"), "gemini"=>format!("{base}/models?key={}", connection.api_key), _=>format!("{base}/models") };
    let response=auth_request(&client,&connection,url).send().await.map_err(|e|e.to_string())?; let status=response.status(); let value:Value=response.json().await.map_err(|e|e.to_string())?;
    if !status.is_success(){return Err(format!("Model discovery returned {status}: {value}"));}
    let list=if connection.kind.contains("ollama") { value["models"].as_array().cloned().unwrap_or_default().iter().filter_map(|x|x["name"].as_str().map(String::from)).collect() } else { value["data"].as_array().or_else(||value["models"].as_array()).cloned().unwrap_or_default().iter().filter_map(|x|x["id"].as_str().or_else(||x["name"].as_str()).map(|s|s.trim_start_matches("models/").to_string())).collect() };
    Ok(list)
}

#[tauri::command]
async fn test_connection(mut connection: Connection) -> Result<String,String>{ hydrate_key(&mut connection); let models=discover_models(connection).await?; Ok(format!("Connected — {} model{} available",models.len(),if models.len()==1{""}else{"s"})) }

fn budget_messages(messages:&[ChatMessage], settings:&Settings)->Vec<ChatMessage>{
    let mut selected=Vec::new(); let mut estimated=0_u64;
    for message in messages.iter().rev().take(settings.context_message_limit) { let tokens=(message.content.chars().count() as u64/4).max(1); if estimated+tokens>settings.context_token_budget && !selected.is_empty(){break;} estimated+=tokens; selected.push(message.clone()); }
    selected.reverse(); selected
}

async fn provider_request(connection:&Connection,model:&str,messages:&[ChatMessage])->Result<(String,u64,u64),String>{
    let client=reqwest::Client::new(); let base=normalized(&connection.base_url); let prompt=messages.iter().map(|m|format!("{}: {}",m.role,m.content)).collect::<Vec<_>>().join("\n\n");
    let (response,kind)=match connection.kind.as_str(){
        "local_ollama"|"remote_ollama"=>(client.post(format!("{base}/api/generate")).json(&json!({"model":model,"prompt":prompt,"stream":false})),"ollama"),
        "anthropic"=>(client.post(format!("{base}/messages")).header("x-api-key",&connection.api_key).header("anthropic-version","2023-06-01").json(&json!({"model":model,"max_tokens":2048,"messages":messages})),"anthropic"),
        "gemini"=>(client.post(format!("{base}/models/{model}:generateContent?key={}",connection.api_key)).json(&json!({"contents":messages.iter().map(|m|json!({"role":if m.role=="assistant"{"model"}else{"user"},"parts":[{"text":m.content}]})).collect::<Vec<_>>()})),"gemini"),
        _=>{let mut r=client.post(format!("{base}/chat/completions")).json(&json!({"model":model,"messages":messages}));if !connection.api_key.is_empty(){r=r.bearer_auth(&connection.api_key)};(r,"openai")}
    };
    let response=response.send().await.map_err(|e|e.to_string())?;let status=response.status();let value:Value=response.json().await.map_err(|e|e.to_string())?;if !status.is_success(){return Err(format!("{status}: {value}"));}
    match kind {"ollama"=>Ok((value["response"].as_str().unwrap_or_default().into(),value["prompt_eval_count"].as_u64().unwrap_or(0),value["eval_count"].as_u64().unwrap_or(0))),"anthropic"=>Ok((value["content"][0]["text"].as_str().unwrap_or_default().into(),value["usage"]["input_tokens"].as_u64().unwrap_or(0),value["usage"]["output_tokens"].as_u64().unwrap_or(0))),"gemini"=>Ok((value["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or_default().into(),value["usageMetadata"]["promptTokenCount"].as_u64().unwrap_or(0),value["usageMetadata"]["candidatesTokenCount"].as_u64().unwrap_or(0))),_=>Ok((value["choices"][0]["message"]["content"].as_str().unwrap_or_default().into(),value["usage"]["prompt_tokens"].as_u64().unwrap_or(0),value["usage"]["completion_tokens"].as_u64().unwrap_or(0)))}
}

#[tauri::command]
async fn send_chat(connection_id:String,model:String,messages:Vec<ChatMessage>)->Result<ChatResult,String>{
    let settings:Settings=read_json(path("settings.json"));let mut connections:Vec<Connection>=read_json(path("connections.json"));for c in &mut connections{hydrate_key(c)}
    let mut ordered=Vec::new();if let Some(c)=connections.iter().find(|x|x.id==connection_id&&x.enabled){ordered.push(c.clone())}for id in &settings.fallback_connection_ids{if let Some(c)=connections.iter().find(|x|&x.id==id&&x.enabled&&x.id!=connection_id){ordered.push(c.clone())}}for c in connections{if c.enabled&&!ordered.iter().any(|x|x.id==c.id){ordered.push(c)}}if !settings.automatic_fallback{ordered.truncate(1)}
    let context=budget_messages(&messages,&settings);let started=Instant::now();let mut failures=Vec::new();
    for connection in ordered { let request_model=if connection.id==connection_id&&!model.trim().is_empty(){model.clone()}else{connection.default_model.clone()};match provider_request(&connection,&request_model,&context).await{Ok((response,input_tokens,output_tokens))=>{let cost=input_tokens as f64/1_000_000.0*connection.input_cost_per_million+output_tokens as f64/1_000_000.0*connection.output_cost_per_million;let result=ChatResult{connection_id:connection.id,connection_name:connection.name,model:request_model,response,input_tokens,output_tokens,total_tokens:input_tokens+output_tokens,estimated_cost:cost,latency_ms:started.elapsed().as_secs_f64()*1000.0,created_at:epoch(),fallback_attempts:failures};let mut history:Vec<ChatResult>=read_json(path("usage.json"));history.push(result.clone());if history.len()>1000{let remove=history.len()-1000;history.drain(..remove);}write_json(path("usage.json"),&history)?;return Ok(result)},Err(e)=>failures.push(format!("{}: {}",connection.name,e))} }
    Err(format!("All available providers failed: {}",failures.join(" | ")))
}

#[tauri::command] fn list_conversations()->Vec<Conversation>{let mut x:Vec<Conversation>=read_json(path("conversations.json"));x.sort_by_key(|c|std::cmp::Reverse(c.updated_at));x}
#[tauri::command] fn save_conversation(conversation:Conversation)->Result<(),String>{let mut items:Vec<Conversation>=read_json(path("conversations.json"));if let Some(x)=items.iter_mut().find(|x|x.id==conversation.id){*x=conversation}else{items.push(conversation)};write_json(path("conversations.json"),&items)}
#[tauri::command] fn delete_conversation(id:String)->Result<(),String>{let mut items:Vec<Conversation>=read_json(path("conversations.json"));items.retain(|x|x.id!=id);write_json(path("conversations.json"),&items)}
#[tauri::command] fn get_settings()->Settings{read_json(path("settings.json"))}
#[tauri::command] fn save_settings(settings:Settings)->Result<(),String>{if settings.theme!="dark"&&settings.theme!="light"&&settings.theme!="system"{return Err("Invalid theme".into())}if settings.context_token_budget<256||settings.context_message_limit==0{return Err("Context limits must be positive".into())}write_json(path("settings.json"),&settings)}
#[tauri::command] fn get_usage_summary()->UsageSummary{let history:Vec<ChatResult>=read_json(path("usage.json"));let mut grouped:HashMap<String,UsageConnection>=HashMap::new();let mut models:HashMap<String,UsageModel>=HashMap::new();for x in &history{let row=grouped.entry(x.connection_id.clone()).or_insert(UsageConnection{connection_id:x.connection_id.clone(),connection_name:x.connection_name.clone(),requests:0,input_tokens:0,output_tokens:0,total_tokens:0,estimated_cost:0.0});row.requests+=1;row.input_tokens+=x.input_tokens;row.output_tokens+=x.output_tokens;row.total_tokens+=x.total_tokens;row.estimated_cost+=x.estimated_cost;let model_key=format!("{}\u{0}{}",x.connection_id,x.model);let model=models.entry(model_key).or_insert(UsageModel{connection_id:x.connection_id.clone(),provider_name:x.connection_name.clone(),model:x.model.clone(),requests:0,input_tokens:0,output_tokens:0,total_tokens:0,estimated_cost:0.0,remaining_tokens:None});model.requests+=1;model.input_tokens+=x.input_tokens;model.output_tokens+=x.output_tokens;model.total_tokens+=x.total_tokens;model.estimated_cost+=x.estimated_cost}let mut by_model:Vec<UsageModel>=models.into_values().collect();by_model.sort_by(|a,b|a.provider_name.cmp(&b.provider_name).then(a.model.cmp(&b.model)));UsageSummary{total_requests:history.len()as u64,input_tokens:history.iter().map(|x|x.input_tokens).sum(),output_tokens:history.iter().map(|x|x.output_tokens).sum(),total_tokens:history.iter().map(|x|x.total_tokens).sum(),estimated_cost:history.iter().map(|x|x.estimated_cost).sum(),by_connection:grouped.into_values().collect(),by_model,recent:history.into_iter().rev().take(100).collect()}}
#[tauri::command] fn clear_usage_history()->Result<(),String>{write_json(path("usage.json"),&Vec::<ChatResult>::new())}

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){tauri::Builder::default().plugin(tauri_plugin_shell::init()).invoke_handler(tauri::generate_handler![get_system_metrics,list_connections,save_connection,delete_connection,test_connection,discover_models,send_chat,list_conversations,save_conversation,delete_conversation,get_settings,save_settings,get_usage_summary,clear_usage_history]).run(tauri::generate_context!()).expect("error while running Forge AI");}

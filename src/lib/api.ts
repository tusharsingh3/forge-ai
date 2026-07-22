import { invoke } from '@tauri-apps/api/core';

export type ProviderKind = 'local_ollama' | 'remote_ollama' | 'openai' | 'anthropic' | 'gemini' | 'openai_compatible';
export type ThemeMode = 'dark' | 'light' | 'system';
export interface Metrics { cpu_usage:number; total_memory:number; used_memory:number; available_memory:number; platform:string; architecture:string; cpu_brand:string; logical_cores:number; gpu_name:string; }
export interface Connection { id:string; name:string; kind:ProviderKind; base_url:string; api_key:string; has_api_key:boolean; default_model:string; enabled:boolean; input_cost_per_million:number; output_cost_per_million:number; }
export interface ChatMessage { role:'user'|'assistant'; content:string; meta?:ChatResult; }
export interface Conversation { id:string; title:string; connection_id:string; model:string; messages:ChatMessage[]; updated_at:number; }
export interface Settings { theme:ThemeMode; automatic_fallback:boolean; context_token_budget:number; context_message_limit:number; fallback_connection_ids:string[]; }
export interface ChatResult { connection_id:string; connection_name:string; model:string; response:string; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; latency_ms:number; created_at:number; fallback_attempts:string[]; }
export interface UsageConnection { connection_id:string; connection_name:string; requests:number; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; }
export interface UsageModel { connection_id:string; provider_name:string; model:string; requests:number; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; remaining_tokens:number|null; }
export interface UsageSummary { total_requests:number; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; by_connection:UsageConnection[]; by_model:UsageModel[]; recent:ChatResult[]; }

export const api = {
  metrics:()=>invoke<Metrics>('get_system_metrics'),
  connections:()=>invoke<Connection[]>('list_connections'),
  saveConnection:(connection:Connection)=>invoke<void>('save_connection',{connection}),
  deleteConnection:(id:string)=>invoke<void>('delete_connection',{id}),
  testConnection:(connection:Connection)=>invoke<string>('test_connection',{connection}),
  discoverModels:(connection:Connection)=>invoke<string[]>('discover_models',{connection}),
  chat:(connectionId:string,model:string,messages:ChatMessage[])=>invoke<ChatResult>('send_chat',{connectionId,model,messages:messages.map(({role,content})=>({role,content}))}),
  conversations:()=>invoke<Conversation[]>('list_conversations'),
  saveConversation:(conversation:Conversation)=>invoke<void>('save_conversation',{conversation}),
  deleteConversation:(id:string)=>invoke<void>('delete_conversation',{id}),
  settings:()=>invoke<Settings>('get_settings'),
  saveSettings:(settings:Settings)=>invoke<void>('save_settings',{settings}),
  usage:()=>invoke<UsageSummary>('get_usage_summary'),
  clearUsage:()=>invoke<void>('clear_usage_history'),
};

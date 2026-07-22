import { invoke } from '@tauri-apps/api/core';
export type ProviderKind = 'local_ollama' | 'remote_ollama' | 'openai' | 'anthropic' | 'gemini' | 'openai_compatible';
export interface Metrics { cpu_usage:number; total_memory:number; used_memory:number; available_memory:number; platform:string; architecture:string; cpu_brand:string; logical_cores:number; gpu_name:string; }
export interface Connection { id:string; name:string; kind:ProviderKind; base_url:string; api_key:string; default_model:string; enabled:boolean; input_cost_per_million:number; output_cost_per_million:number; }
export interface ChatResult { connection_id:string; connection_name:string; model:string; response:string; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; latency_ms:number; created_at:number; }
export interface UsageConnection { connection_id:string; connection_name:string; requests:number; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; }
export interface UsageSummary { total_requests:number; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; by_connection:UsageConnection[]; recent:ChatResult[]; }
export interface ChatMessage { role:'user'|'assistant'; content:string; meta?:ChatResult; }
export interface AppSettings { theme:'dark'|'light'|'system'; automatic_fallback:boolean; context_token_budget:number; history_limit:number; }
export const api = {
 metrics:()=>invoke<Metrics>('get_system_metrics'),
 connections:()=>invoke<Connection[]>('list_connections'),
 saveConnection:(connection:Connection)=>invoke<void>('save_connection',{connection}),
 deleteConnection:(id:string)=>invoke<void>('delete_connection',{id}),
 testConnection:(connection:Connection)=>invoke<string>('test_connection',{connection}),
 discoverModels:(connection:Connection)=>invoke<string[]>('discover_models',{connection}),
 chat:(connectionId:string,model:string,prompt:string)=>invoke<ChatResult>('send_chat',{connectionId,model,prompt}),
 chatWithFallback:(connectionIds:string[],model:string,prompt:string)=>invoke<ChatResult>('send_chat_with_fallback',{connectionIds,model,prompt}),
 conversation:()=>invoke<ChatMessage[]>('get_conversation'),
 saveConversation:(messages:ChatMessage[])=>invoke<void>('save_conversation',{messages}),
 clearConversation:()=>invoke<void>('clear_conversation'),
 settings:()=>invoke<AppSettings>('get_settings'),
 saveSettings:(settings:AppSettings)=>invoke<void>('save_settings',{settings}),
 usage:()=>invoke<UsageSummary>('get_usage_summary'),
 clearUsage:()=>invoke<void>('clear_usage_history')
};

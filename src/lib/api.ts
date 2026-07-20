import { invoke } from '@tauri-apps/api/core';
export type ProviderKind = 'local_ollama' | 'remote_ollama' | 'openai' | 'anthropic' | 'gemini' | 'openai_compatible';
export interface Metrics { cpu_usage:number; total_memory:number; used_memory:number; available_memory:number; platform:string; architecture:string; cpu_brand:string; logical_cores:number; gpu_name:string; }
export interface Connection { id:string; name:string; kind:ProviderKind; base_url:string; api_key:string; default_model:string; enabled:boolean; input_cost_per_million:number; output_cost_per_million:number; }
export interface ChatResult { connection_id:string; connection_name:string; model:string; response:string; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; latency_ms:number; created_at:number; }
export interface UsageConnection { connection_id:string; connection_name:string; requests:number; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; }
export interface UsageSummary { total_requests:number; input_tokens:number; output_tokens:number; total_tokens:number; estimated_cost:number; by_connection:UsageConnection[]; recent:ChatResult[]; }
export const api = {
 metrics:()=>invoke<Metrics>('get_system_metrics'),
 connections:()=>invoke<Connection[]>('list_connections'),
 saveConnection:(connection:Connection)=>invoke<void>('save_connection',{connection}),
 deleteConnection:(id:string)=>invoke<void>('delete_connection',{id}),
 testConnection:(connection:Connection)=>invoke<string>('test_connection',{connection}),
 chat:(connectionId:string,model:string,prompt:string)=>invoke<ChatResult>('send_chat',{connectionId,model,prompt}),
 usage:()=>invoke<UsageSummary>('get_usage_summary'),
 clearUsage:()=>invoke<void>('clear_usage_history')
};

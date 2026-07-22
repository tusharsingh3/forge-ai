import { api, type AccountUser, type StoredAccountSession } from './api';

export interface AccountDevice { id:string; name:string; platform:string; last_seen_at:string; current:boolean; }
interface AuthResponse { session:StoredAccountSession; }
interface OAuthStart { device_code:string; user_code:string; verification_uri:string; interval_seconds:number; }

const baseUrl=(import.meta.env.VITE_FORGE_API_URL as string|undefined)?.replace(/\/$/,'')||'';
export const sessionNeedsRefresh=(expiresAt:number,now=Date.now())=>expiresAt<=now+30_000;

async function request<T>(path:string,init:RequestInit={},authenticated=false):Promise<T>{
  if(!baseUrl)throw new Error('Forge account services are not configured in this build. Local Mode remains fully available.');
  let session=authenticated?await validSession():null;
  const headers=new Headers(init.headers);headers.set('content-type','application/json');if(session)headers.set('authorization',`Bearer ${session.access_token}`);
  const response=await fetch(`${baseUrl}${path}`,{...init,headers});
  const body=await response.json().catch(()=>({}));
  if(!response.ok)throw new Error(typeof body.message==='string'?body.message:'The Forge account service could not complete this request.');
  return body as T;
}

async function validSession(){
  const session=await api.accountSession();
  if(!session)return null;
  if(!sessionNeedsRefresh(session.expires_at))return session;
  const refreshed=await request<AuthResponse>('/v1/auth/refresh',{method:'POST',body:JSON.stringify({refresh_token:session.refresh_token})});
  await api.saveAccountSession(refreshed.session);
  return refreshed.session;
}

async function saveAuth(path:string,payload:object){const result=await request<AuthResponse>(path,{method:'POST',body:JSON.stringify(payload)});await api.saveAccountSession(result.session);return result.session.user;}

export const auth={
  configured:Boolean(baseUrl),
  current:async():Promise<AccountUser|null>=>{const session=await validSession();return session?.user??null},
  signIn:async(email:string,password:string)=>saveAuth('/v1/auth/login',{email,password,device_id:await api.deviceId()}),
  signUp:async(display_name:string,email:string,password:string)=>saveAuth('/v1/auth/register',{display_name,email,password,device_id:await api.deviceId()}),
  forgotPassword:(email:string)=>request<void>('/v1/auth/password/forgot',{method:'POST',body:JSON.stringify({email})}),
  resendVerification:()=>request<void>('/v1/auth/email/resend',{method:'POST'},true),
  oauth:async(provider:'google'|'github',onStarted:(value:OAuthStart)=>void)=>{
    const started=await request<OAuthStart>(`/v1/auth/oauth/${provider}/start`,{method:'POST',body:JSON.stringify({device_id:await api.deviceId()})});
    onStarted(started);
    const deadline=Date.now()+5*60_000;
    while(Date.now()<deadline){
      await new Promise(resolve=>setTimeout(resolve,Math.max(2,started.interval_seconds)*1000));
      try{const result=await request<AuthResponse>('/v1/auth/oauth/poll',{method:'POST',body:JSON.stringify({device_code:started.device_code})});await api.saveAccountSession(result.session);return result.session.user}catch(error){if(!String(error).includes('authorization_pending'))throw error}
    }
    throw new Error('Browser sign-in expired. Please try again.');
  },
  devices:()=>request<AccountDevice[]>('/v1/account/devices',{},true),
  revokeDevice:(id:string)=>request<void>(`/v1/account/devices/${encodeURIComponent(id)}`,{method:'DELETE'},true),
  signOut:async()=>{try{await request<void>('/v1/auth/logout',{method:'POST'},true)}finally{await api.clearAccountSession()}},
  signOutAll:async()=>{await request<void>('/v1/auth/logout-all',{method:'POST'},true);await api.clearAccountSession()},
  deleteAccount:async()=>{await request<void>('/v1/account',{method:'DELETE'},true);await api.clearAccountSession()},
};

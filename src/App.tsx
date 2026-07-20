import { useEffect, useMemo, useState } from 'react';
import {
  Activity,
  AlertTriangle,
  Bot,
  Cloud,
  Gauge,
  Plus,
  Send,
  Server,
  Settings,
  Sparkles,
} from 'lucide-react';
import { api, type ChatResult, type Connection, type Metrics, type UsageSummary } from './lib/api';

type Tab = 'dashboard' | 'connections' | 'playground' | 'usage' | 'settings';
type ChatMessage = { role: 'user' | 'assistant'; content: string; meta?: ChatResult };

const labels: Record<Connection['kind'], string> = {
  local_ollama: 'Local Ollama',
  remote_ollama: 'Remote Ollama / Linux server',
  openai: 'OpenAI API',
  anthropic: 'Anthropic API',
  gemini: 'Google Gemini API',
  openai_compatible: 'OpenAI-compatible API',
};

const emptyUsage: UsageSummary = {
  total_requests: 0,
  input_tokens: 0,
  output_tokens: 0,
  total_tokens: 0,
  estimated_cost: 0,
  by_connection: [],
  recent: [],
};

const emptyMetrics: Metrics = {
  cpu_usage: 0,
  total_memory: 0,
  used_memory: 0,
  available_memory: 0,
  platform: '',
  architecture: '',
  cpu_brand: '',
  logical_cores: 0,
  gpu_name: '',
};

const icons = {
  dashboard: <Gauge />,
  connections: <Cloud />,
  playground: <Send />,
  usage: <Activity />,
  settings: <Settings />,
};

const titles = {
  dashboard: 'AI control center',
  connections: 'Provider connections',
  playground: 'Unified AI chat',
  usage: 'Tokens and cost',
  settings: 'Application settings',
};

export default function App() {
  const [tab, setTab] = useState<Tab>('dashboard');
  const [metrics, setMetrics] = useState(emptyMetrics);
  const [connections, setConnections] = useState<Connection[]>([]);
  const [usage, setUsage] = useState(emptyUsage);
  const [error, setError] = useState('');

  const refresh = async () => {
    try {
      const [m, c, u] = await Promise.all([api.metrics(), api.connections(), api.usage()]);
      setMetrics(m);
      setConnections(c);
      setUsage(u);
      setError('');
    } catch (e) {
      setError(String(e));
    }
  };

  useEffect(() => {
    refresh();
    const t = setInterval(refresh, 4000);
    return () => clearInterval(t);
  }, []);

  return (
    <div className="shell">
      <aside>
        <div className="brand">
          <b>F</b>
          <div>
            <strong>Forge AI</strong>
            <small>Version 1.0.0</small>
          </div>
        </div>

        <nav>
          {(['dashboard', 'connections', 'playground', 'usage', 'settings'] as Tab[]).map((x) => (
            <button className={tab === x ? 'active' : ''} onClick={() => setTab(x)} key={x}>
              {icons[x]}
              {x}
            </button>
          ))}
        </nav>

        <div className="device">
          <small>This device</small>
          <strong>{metrics.cpu_brand || 'Detecting hardware...'}</strong>
          <span>
            {metrics.logical_cores} cores - {(metrics.total_memory / 1024 ** 3).toFixed(1)} GB RAM
            <br />
            {metrics.gpu_name}
          </span>
        </div>
      </aside>

      <main>
        <header>
          <small>FORGE AI 1.0.0</small>
          <h1>{titles[tab]}</h1>
          <p>One desktop control center for local, remote, and cloud AI.</p>
        </header>

        {error && <div className="error">{error}</div>}
        {tab === 'dashboard' && <Dashboard metrics={metrics} connections={connections} usage={usage} />}
        {tab === 'connections' && <Connections items={connections} refresh={refresh} />}
        {tab === 'playground' && <Playground items={connections} usage={usage} refresh={refresh} />}
        {tab === 'usage' && <Usage usage={usage} />}
        {tab === 'settings' && (
          <section className="panel">
            <h2>Settings</h2>
            <p>
              Remote Linux servers are supported through Ollama or OpenAI-compatible connection URLs.
              Secure OS credential storage is the next hardening task.
            </p>
          </section>
        )}
      </main>
    </div>
  );
}

function Dashboard({
  metrics,
  connections,
  usage,
}: {
  metrics: Metrics;
  connections: Connection[];
  usage: UsageSummary;
}) {
  return (
    <>
      <div className="cards">
        <Card label="Connections" value={String(connections.length)} />
        <Card label="Requests" value={String(usage.total_requests)} />
        <Card label="Tokens" value={usage.total_tokens.toLocaleString()} />
        <Card label="CPU" value={`${metrics.cpu_usage.toFixed(0)}%`} />
      </div>

      <section className="panel">
        <h2>Execution targets</h2>
        {connections.length ? (
          connections.map((c) => (
            <div className="row" key={c.id}>
              <span className="provider">{c.kind.includes('ollama') ? <Server /> : <Cloud />}</span>
              <div>
                <strong>{c.name}</strong>
                <small>
                  {labels[c.kind]} - {c.default_model || 'Select model per request'}
                </small>
              </div>
            </div>
          ))
        ) : (
          <p>No connections yet. Add a local runtime, remote Linux server, or cloud API.</p>
        )}
      </section>
    </>
  );
}

function Connections({ items, refresh }: { items: Connection[]; refresh: () => Promise<void> }) {
  const [form, setForm] = useState<Connection | null>(null);

  const save = async () => {
    if (form) {
      await api.saveConnection(form);
      setForm(null);
      await refresh();
    }
  };

  return (
    <>
      <div className="toolbar">
        <div>
          <h2>Connections</h2>
          <p>Cloud providers require API access; consumer subscriptions are not reusable API credentials.</p>
        </div>
        <button
          onClick={() =>
            setForm({
              id: crypto.randomUUID(),
              name: '',
              kind: 'remote_ollama',
              base_url: 'http://',
              api_key: '',
              default_model: '',
              enabled: true,
              input_cost_per_million: 0,
              output_cost_per_million: 0,
            })
          }
        >
          <Plus />
          Add
        </button>
      </div>

      <section className="panel">
        {items.map((c) => (
          <div className="row" key={c.id}>
            <span className="provider">
              <Cloud />
            </span>
            <div>
              <strong>{c.name}</strong>
              <small>
                {labels[c.kind]} - {c.base_url}
              </small>
            </div>
            <button onClick={() => setForm(c)}>Edit</button>
          </div>
        ))}
      </section>

      {form && (
        <div className="modal">
          <div className="dialog">
            <h2>Connection</h2>
            <input placeholder="Name" value={form.name} onChange={(e) => setForm({ ...form, name: e.target.value })} />
            <select
              value={form.kind}
              onChange={(e) => setForm({ ...form, kind: e.target.value as Connection['kind'] })}
            >
              {Object.entries(labels).map(([k, v]) => (
                <option value={k} key={k}>
                  {v}
                </option>
              ))}
            </select>
            <input
              placeholder="Base URL"
              value={form.base_url}
              onChange={(e) => setForm({ ...form, base_url: e.target.value })}
            />
            <input
              placeholder="API key"
              type="password"
              value={form.api_key}
              onChange={(e) => setForm({ ...form, api_key: e.target.value })}
            />
            <input
              placeholder="Default model"
              value={form.default_model}
              onChange={(e) => setForm({ ...form, default_model: e.target.value })}
            />
            <div>
              <button onClick={() => setForm(null)}>Cancel</button>
              <button onClick={save}>Save</button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

function Playground({
  items,
  usage,
  refresh,
}: {
  items: Connection[];
  usage: UsageSummary;
  refresh: () => Promise<void>;
}) {
  const enabledItems = items.filter((item) => item.enabled);
  const [id, setId] = useState('');
  const [model, setModel] = useState('');
  const [prompt, setPrompt] = useState('Help me design the Forge AI model switching experience.');
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      role: 'assistant',
      content:
        'Start a unified chat here, then switch providers from the model control panel without losing the current conversation context.',
    },
  ]);
  const [isSending, setIsSending] = useState(false);

  useEffect(() => {
    if (!id && enabledItems[0]) {
      setId(enabledItems[0].id);
      setModel(enabledItems[0].default_model);
    }
  }, [enabledItems, id]);

  const active = useMemo(() => items.find((item) => item.id === id), [items, id]);
  const activeUsage = useMemo(() => usage.by_connection.find((item) => item.connection_id === id), [usage, id]);
  const recent = useMemo(() => usage.recent.find((item) => item.connection_id === id), [usage, id]);
  const fallbackItems = enabledItems.filter((item) => item.id !== id).slice(0, 3);
  const contextPercent = Math.min(94, Math.max(12, Math.round((messages.length / 18) * 100)));
  const quotaWarning = contextPercent > 70 || (activeUsage?.total_tokens ?? 0) > 50000;

  const switchProvider = (connection: Connection) => {
    setId(connection.id);
    setModel(connection.default_model);
  };

  const buildContextPrompt = (nextPrompt: string) => {
    const transcript = messages
      .slice(-8)
      .map((message) => `${message.role === 'user' ? 'User' : 'Assistant'}: ${message.content}`)
      .join('\n\n');

    return [
      'Continue this Forge AI conversation without losing context.',
      'Use the transcript below as the shared context across providers.',
      transcript ? `Transcript:\n${transcript}` : '',
      `User: ${nextPrompt}`,
    ]
      .filter(Boolean)
      .join('\n\n');
  };

  const send = async () => {
    if (!id || !model || !prompt.trim()) return;

    const nextPrompt = prompt.trim();
    setPrompt('');
    setIsSending(true);
    setMessages((current) => [...current, { role: 'user', content: nextPrompt }]);

    try {
      const r = await api.chat(id, model, buildContextPrompt(nextPrompt));
      setMessages((current) => [...current, { role: 'assistant', content: r.response, meta: r }]);
      await refresh();
    } catch (e) {
      setMessages((current) => [
        ...current,
        { role: 'assistant', content: `Unable to complete the request: ${String(e)}` },
      ]);
    } finally {
      setIsSending(false);
    }
  };

  return (
    <div className="workspace">
      <section className="chat-panel">
        <div className="chat-header">
          <div>
            <span>Conversation</span>
            <h2>Build Forge AI Desktop App</h2>
          </div>
          <div className="chat-actions">
            <select
              value={id}
              onChange={(e) => {
                const next = items.find((item) => item.id === e.target.value);
                if (next) switchProvider(next);
              }}
            >
              <option value="">Select provider</option>
              {enabledItems.map((item) => (
                <option value={item.id} key={item.id}>
                  {item.name}
                </option>
              ))}
            </select>
            <input value={model} onChange={(e) => setModel(e.target.value)} placeholder="Model ID" />
          </div>
        </div>

        <div className="messages">
          {messages.map((message, index) => (
            <div className={`message ${message.role}`} key={`${message.role}-${index}`}>
              <span className="avatar">{message.role === 'user' ? 'You' : <Bot />}</span>
              <div>
                <strong>{message.role === 'user' ? 'You' : active?.name || 'Forge AI'}</strong>
                <p>{message.content}</p>
                {message.meta && (
                  <small>
                    {message.meta.total_tokens.toLocaleString()} tokens - {(message.meta.latency_ms / 1000).toFixed(1)}s
                  </small>
                )}
              </div>
            </div>
          ))}
        </div>

        <div className="composer">
          <textarea value={prompt} onChange={(e) => setPrompt(e.target.value)} />
          <button onClick={send} disabled={!id || !model || isSending}>
            <Send />
            {isSending ? 'Sending' : 'Send'}
          </button>
        </div>
      </section>

      <aside className="model-panel">
        <div className="panel-heading">
          <span>Active Model</span>
          <strong>{active?.name || 'No provider selected'}</strong>
          <small>{active ? `${labels[active.kind]} - Online` : 'Add a provider to begin'}</small>
        </div>

        {quotaWarning && (
          <div className="quota-warning">
            <AlertTriangle />
            <div>
              <strong>Quota or context nearly exhausted</strong>
              <small>Switch providers and Forge AI will carry the recent context forward.</small>
            </div>
          </div>
        )}

        <div className="meter-card">
          <MetricRow label="Context Usage" value={`${contextPercent}%`} />
          <div className="meter">
            <span style={{ width: `${contextPercent}%` }} />
          </div>
          <MetricRow label="Input Tokens" value={(activeUsage?.input_tokens ?? 0).toLocaleString()} />
          <MetricRow label="Output Tokens" value={(activeUsage?.output_tokens ?? 0).toLocaleString()} />
          <MetricRow label="Estimated Cost" value={`$${(activeUsage?.estimated_cost ?? 0).toFixed(4)}`} />
          <MetricRow label="Latency" value={recent ? `${(recent.latency_ms / 1000).toFixed(1)}s` : '-'} />
        </div>

        <div className="side-section">
          <h3>Switch Model</h3>
          <div className="model-grid">
            {enabledItems.map((item) => (
              <button className={item.id === id ? 'selected' : ''} onClick={() => switchProvider(item)} key={item.id}>
                <span>{item.kind.includes('ollama') ? <Server /> : <Sparkles />}</span>
                <strong>{item.name}</strong>
                <small>{item.default_model || labels[item.kind]}</small>
              </button>
            ))}
          </div>
          {!enabledItems.length && <p>Add a provider connection to unlock model switching.</p>}
        </div>

        <div className="side-section">
          <h3>Fallback Order</h3>
          <div className="fallback">
            {[active, ...fallbackItems].filter(Boolean).map((item, index) => (
              <span key={item!.id}>
                {index + 1}. {item!.name}
              </span>
            ))}
          </div>
          <p>Requests can fail over when a model is unavailable, out of quota, or too expensive.</p>
        </div>
      </aside>
    </div>
  );
}

function MetricRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="metric-row">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function Usage({ usage }: { usage: UsageSummary }) {
  return (
    <div className="cards">
      <Card label="Requests" value={String(usage.total_requests)} />
      <Card label="Input tokens" value={usage.input_tokens.toLocaleString()} />
      <Card label="Output tokens" value={usage.output_tokens.toLocaleString()} />
      <Card label="Estimated cost" value={`$${usage.estimated_cost.toFixed(4)}`} />
    </div>
  );
}

function Card({ label, value }: { label: string; value: string }) {
  return (
    <div className="card">
      <small>{label}</small>
      <strong>{value}</strong>
    </div>
  );
}

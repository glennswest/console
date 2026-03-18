use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Terminal</div>
<div class="filter-bar">
  <label>Node:</label>
  <select id="node-select" onchange="loadProcesses()"><option value="">Select node...</option></select>
  <label>Process:</label>
  <select id="proc-select" onchange="connectWS()"><option value="">Select process...</option></select>
  <span id="ws-status" class="badge badge-gray">disconnected</span>
</div>
<div class="term-output" id="terminal" style="max-height:calc(100vh - 200px);min-height:500px"><div class="empty">Select a node and process</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
let ws = null;
async function init() {{
    const nodes = await fetch(MKUBE + '/api/v1/nodes').then(r => r.json()).catch(() => null);
    const nodeItems = (nodes && nodes.items) || [];
    const sel = document.getElementById('node-select');
    for (const n of nodeItems) {{
        const name = n.metadata?.name || '';
        const ip = (n.status?.addresses || []).find(a => a.type === 'InternalIP')?.address || '';
        sel.innerHTML += `<option value="${{ip}}">${{name}} (${{ip}})</option>`;
    }}
}}
async function loadProcesses() {{
    const ip = document.getElementById('node-select').value;
    const psel = document.getElementById('proc-select');
    psel.innerHTML = '<option value="">Select process...</option>';
    if (!ip) return;
    try {{
        const procs = await fetch(`http://${{ip}}:9080/api/v1/processes`).then(r => r.json());
        for (const p of procs) {{
            psel.innerHTML += `<option value="${{p.name}}">${{p.name}} (${{p.state||'?'}})</option>`;
        }}
    }} catch(e) {{
        psel.innerHTML = '<option value="">stormd unreachable</option>';
    }}
}}
function connectWS() {{
    const ip = document.getElementById('node-select').value;
    const proc = document.getElementById('proc-select').value;
    if (!ip || !proc) return;
    if (ws) {{ ws.close(); ws = null; }}
    const term = document.getElementById('terminal');
    term.innerHTML = '';
    const statusEl = document.getElementById('ws-status');
    statusEl.className = 'badge badge-yellow';
    statusEl.textContent = 'connecting';
    const proto = 'ws:';
    ws = new WebSocket(`${{proto}}//${{ip}}:9080/ws/console/${{encodeURIComponent(proc)}}`);
    ws.onopen = () => {{
        statusEl.className = 'badge badge-green';
        statusEl.textContent = 'connected';
    }};
    ws.onclose = () => {{
        statusEl.className = 'badge badge-gray';
        statusEl.textContent = 'disconnected';
    }};
    ws.onerror = () => {{
        statusEl.className = 'badge badge-red';
        statusEl.textContent = 'error';
    }};
    ws.onmessage = (e) => {{
        const msg = JSON.parse(e.data);
        if (msg.type === 'snapshot') {{
            term.innerHTML = '<div style="color:#666">--- terminal snapshot ---</div>' +
                ansiToHtml(msg.data?.contents || msg.data || '') +
                '<div style="color:#666">--- live output ---</div>\n';
        }} else if (msg.type === 'entry') {{
            const cls = msg.data?.stream || 'stdout';
            const ts = msg.data?.timestamp ? new Date(msg.data.timestamp).toLocaleTimeString() : '';
            const color = cls === 'stderr' ? 'color:#e94560' : '';
            const line = `<div class="log-entry" style="${{color}}"><span style="color:#666">${{escapeHtml(ts)}}</span> ${{ansiToHtml(msg.data?.line || '')}}</div>`;
            term.insertAdjacentHTML('beforeend', line);
            term.scrollTop = term.scrollHeight;
        }} else if (msg.type === 'lagged') {{
            term.insertAdjacentHTML('beforeend', `<div style="color:#f0a030">--- skipped ${{msg.skipped}} entries ---</div>`);
        }}
    }};
}}
init();
"#
    );
    Html(layout::page_with_js("Terminal", "Logs", body, &js))
}

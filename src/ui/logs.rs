use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Logs</div>
<div class="filter-bar">
  <label>Node:</label>
  <select id="node-filter"><option value="">All</option></select>
  <label>Pod:</label>
  <select id="pod-filter"><option value="">All</option></select>
  <input type="text" id="search" placeholder="Search...">
  <label><input type="checkbox" id="follow" checked> Follow</label>
  <button class="btn btn-sm" onclick="loadLogs()">Refresh</button>
</div>
<div class="term-output" id="logs" style="max-height:calc(100vh - 200px)"><div class="loading">Loading pod list...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
let allPods = [];
async function init() {{
    // Load nodes and pods for filters
    const [nodes, pods] = await Promise.all([
        fetch(MKUBE + '/api/v1/nodes').then(r => r.json()).catch(() => null),
        fetch(MKUBE + '/api/v1/pods').then(r => r.json()).catch(() => null),
    ]);
    allPods = (pods && pods.items) || [];
    const nodeItems = (nodes && nodes.items) || [];
    const nf = document.getElementById('node-filter');
    for (const n of nodeItems) {{
        const name = n.metadata?.name || '';
        nf.innerHTML += `<option value="${{name}}">${{name}}</option>`;
    }}
    const pf = document.getElementById('pod-filter');
    for (const p of allPods) {{
        const name = p.metadata?.name || '';
        const ns = p.metadata?.namespace || 'default';
        pf.innerHTML += `<option value="${{ns}}/${{name}}">${{ns}}/${{name}}</option>`;
    }}
    loadLogs();
}}
async function loadLogs() {{
    const podVal = document.getElementById('pod-filter').value;
    const search = document.getElementById('search').value.toLowerCase();
    const el = document.getElementById('logs');
    if (!podVal) {{
        el.innerHTML = '<div class="empty">Select a pod to view logs</div>';
        return;
    }}
    const [ns, name] = podVal.split('/');
    el.innerHTML = '<div class="loading">Loading...</div>';
    const logs = await fetch(MKUBE + `/api/v1/namespaces/${{ns}}/pods/${{name}}/log`).then(r => r.text()).catch(() => '');
    if (!logs) {{ el.innerHTML = '<div class="empty">No logs</div>'; return; }}
    let lines = logs.split('\n');
    if (search) lines = lines.filter(l => l.toLowerCase().includes(search));
    el.innerHTML = ansiToHtml(lines.join('\n'));
    if (document.getElementById('follow').checked) el.scrollTop = el.scrollHeight;
}}
init();
"#
    );
    Html(layout::page_with_js("Logs", "Logs", body, &js))
}

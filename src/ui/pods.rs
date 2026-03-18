use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn list_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Pods</div>
<div class="filter-bar">
  <label>Namespace:</label>
  <select id="ns-filter" onchange="filterPods()"><option value="">All</option></select>
  <label>Status:</label>
  <select id="status-filter" onchange="filterPods()"><option value="">All</option></select>
  <input type="text" id="search" placeholder="Search..." oninput="filterPods()">
</div>
<div id="pods"><div class="loading">Loading...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
let allPods = [];
async function loadPods() {{
    const r = await fetch(MKUBE + '/api/v1/pods').then(r => r.json()).catch(() => null);
    allPods = (r && r.items) || [];
    // Populate filters
    const nss = [...new Set(allPods.map(p => p.metadata?.namespace || 'default'))].sort();
    const sts = [...new Set(allPods.map(p => (p.status?.phase||'Unknown').toLowerCase()))].sort();
    const nsf = document.getElementById('ns-filter');
    nsf.innerHTML = '<option value="">All</option>' + nss.map(n => `<option>${{n}}</option>`).join('');
    const sf = document.getElementById('status-filter');
    sf.innerHTML = '<option value="">All</option>' + sts.map(s => `<option>${{s}}</option>`).join('');
    filterPods();
}}
function filterPods() {{
    const ns = document.getElementById('ns-filter').value;
    const st = document.getElementById('status-filter').value;
    const q = document.getElementById('search').value.toLowerCase();
    let pods = allPods;
    if (ns) pods = pods.filter(p => (p.metadata?.namespace||'default') === ns);
    if (st) pods = pods.filter(p => (p.status?.phase||'unknown').toLowerCase() === st);
    if (q) pods = pods.filter(p => JSON.stringify(p).toLowerCase().includes(q));
    let h = '<table><tr><th>Name</th><th>Namespace</th><th>Status</th><th>Node</th><th>IP</th><th>Image</th><th>Restarts</th><th>Age</th></tr>';
    for (const p of pods) {{
        const name = p.metadata?.name || '-';
        const ns = p.metadata?.namespace || 'default';
        const phase = p.status?.phase || 'Unknown';
        const node = p.spec?.nodeName || '-';
        const ip = p.status?.podIP || '-';
        const img = (p.spec?.containers || [])[0]?.image || '-';
        const shortImg = img.split('/').pop();
        const restarts = (p.status?.containerStatuses || []).reduce((s,c) => s + (c.restartCount||0), 0);
        const age = timeSince(p.metadata?.creationTimestamp);
        h += `<tr style="cursor:pointer" onclick="location.href='/ui/pods/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}'">
            <td><a href="/ui/pods/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}">${{escapeHtml(name)}}</a></td>
            <td>${{escapeHtml(ns)}}</td>
            <td>${{statusBadge(phase)}}</td>
            <td>${{escapeHtml(node)}}</td>
            <td>${{escapeHtml(ip)}}</td>
            <td title="${{escapeHtml(img)}}">${{escapeHtml(shortImg)}}</td>
            <td>${{restarts}}</td>
            <td>${{age}}</td>
        </tr>`;
    }}
    h += '</table>';
    document.getElementById('pods').innerHTML = pods.length ? h : '<div class="empty">No pods</div>';
}}
loadPods();
setInterval(loadPods, 15000);
"#
    );
    Html(layout::page_with_js("Pods", "Pods", body, &js))
}

pub async fn detail_page(
    State(state): State<Arc<AppState>>,
    Path((ns, name)): Path<(String, String)>,
) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title"><a href="/ui/pods" style="color:#888">Pods</a> / <span style="color:#8be9fd">{ns}</span> / {name}</div>
<div class="card" id="detail"><div class="loading">Loading...</div></div>
<div class="card">
  <div class="card-header">Logs</div>
  <div class="term-output" id="logs" style="max-height:600px"><div class="loading">Loading...</div></div>
</div>"#,
        ns = ns, name = name,
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
const NS = '{ns}';
const NAME = '{name}';
async function loadDetail() {{
    const pod = await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/pods/${{NAME}}`).then(r => r.json()).catch(() => null);
    if (!pod) {{ document.getElementById('detail').innerHTML = '<div class="empty">Pod not found</div>'; return; }}
    let h = '<div class="kv-grid">';
    h += `<div class="kv-key">Status</div><div class="kv-val">${{statusBadge(pod.status?.phase||'Unknown')}}</div>`;
    h += `<div class="kv-key">Node</div><div class="kv-val">${{escapeHtml(pod.spec?.nodeName||'-')}}</div>`;
    h += `<div class="kv-key">IP</div><div class="kv-val">${{escapeHtml(pod.status?.podIP||'-')}}</div>`;
    h += `<div class="kv-key">Started</div><div class="kv-val">${{escapeHtml(pod.status?.startTime||'-')}}</div>`;
    h += `<div class="kv-key">Age</div><div class="kv-val">${{timeSince(pod.metadata?.creationTimestamp)}}</div>`;
    const containers = pod.spec?.containers || [];
    for (const c of containers) {{
        h += `<div class="kv-key">Container</div><div class="kv-val">${{escapeHtml(c.name||'-')}}</div>`;
        h += `<div class="kv-key">Image</div><div class="kv-val" style="word-break:break-all">${{escapeHtml(c.image||'-')}}</div>`;
    }}
    const statuses = pod.status?.containerStatuses || [];
    for (const cs of statuses) {{
        h += `<div class="kv-key">${{escapeHtml(cs.name||'')}} Ready</div><div class="kv-val">${{cs.ready ? '<span style="color:#50fa7b">Yes</span>' : '<span style="color:#e94560">No</span>'}}</div>`;
        h += `<div class="kv-key">${{escapeHtml(cs.name||'')}} Restarts</div><div class="kv-val">${{cs.restartCount||0}}</div>`;
    }}
    // Labels & Annotations
    const labels = pod.metadata?.labels;
    if (labels && typeof labels === 'object') {{
        const lstr = Object.entries(labels).map(([k,v]) => `${{k}}=${{v}}`).join(', ');
        h += `<div class="kv-key">Labels</div><div class="kv-val" style="word-break:break-all">${{escapeHtml(lstr)}}</div>`;
    }}
    const ann = pod.metadata?.annotations;
    if (ann && typeof ann === 'object') {{
        const astr = Object.entries(ann).map(([k,v]) => `${{k}}=${{v}}`).join(', ');
        h += `<div class="kv-key">Annotations</div><div class="kv-val" style="word-break:break-all;font-size:12px">${{escapeHtml(astr)}}</div>`;
    }}
    h += '</div>';
    h += `<div style="margin-top:12px" class="btn-group">
        <button class="btn btn-danger btn-sm" onclick="deletePod()">Delete</button>
    </div>`;
    document.getElementById('detail').innerHTML = h;
    // Logs
    const logs = await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/pods/${{NAME}}/log`).then(r => r.text()).catch(() => '');
    document.getElementById('logs').innerHTML = logs ? ansiToHtml(logs) : '<div class="empty">No logs</div>';
}}
async function deletePod() {{
    if (!confirm('Delete pod ' + NAME + '?')) return;
    await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/pods/${{NAME}}`, {{method:'DELETE'}});
    location.href = '/ui/pods';
}}
loadDetail();
"#,
        mkube_url = mkube_url, ns = ns, name = name,
    );
    Html(layout::page_with_js("Pod Detail", "Pods", &body, &js))
}

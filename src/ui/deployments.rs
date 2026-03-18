use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn list_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Deployments</div>
<div id="deployments"><div class="loading">Loading...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
async function loadDeployments() {{
    const r = await fetch(MKUBE + '/api/v1/deployments').then(r => r.json()).catch(() => null);
    const deps = (r && r.items) || [];
    let h = '<table><tr><th>Name</th><th>Namespace</th><th>Replicas</th><th>Ready</th><th>Image</th><th>Age</th><th>Actions</th></tr>';
    for (const d of deps) {{
        const name = d.metadata?.name || '-';
        const ns = d.metadata?.namespace || 'default';
        const replicas = d.spec?.replicas ?? 1;
        const ready = d.status?.readyReplicas ?? d.status?.availableReplicas ?? 0;
        const img = ((d.spec?.template?.spec?.containers||[])[0]?.image||'-').split('/').pop();
        const age = timeSince(d.metadata?.creationTimestamp);
        const color = ready >= replicas ? '#50fa7b' : ready > 0 ? '#f1fa8c' : '#e94560';
        h += `<tr style="cursor:pointer" onclick="location.href='/ui/deployments/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}'">
            <td><a href="/ui/deployments/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}">${{escapeHtml(name)}}</a></td>
            <td>${{escapeHtml(ns)}}</td>
            <td>${{replicas}}</td>
            <td style="color:${{color}}">${{ready}}/${{replicas}}</td>
            <td title="${{escapeHtml(img)}}">${{escapeHtml(img)}}</td>
            <td>${{age}}</td>
            <td><button class="btn btn-danger btn-sm" onclick="event.stopPropagation();deleteDep('${{ns}}','${{name}}')">Delete</button></td>
        </tr>`;
    }}
    h += '</table>';
    document.getElementById('deployments').innerHTML = deps.length ? h : '<div class="empty">No deployments</div>';
}}
async function deleteDep(ns, name) {{
    if (!confirm('Delete deployment ' + name + '?')) return;
    await fetch(MKUBE + `/api/v1/namespaces/${{ns}}/deployments/${{name}}`, {{method:'DELETE'}});
    setTimeout(loadDeployments, 1000);
}}
loadDeployments();
setInterval(loadDeployments, 15000);
"#
    );
    Html(layout::page_with_js("Deployments", "Deployments", body, &js))
}

pub async fn detail_page(
    State(state): State<Arc<AppState>>,
    Path((ns, name)): Path<(String, String)>,
) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title"><a href="/ui/deployments" style="color:#888">Deployments</a> / <span style="color:#8be9fd">{ns}</span> / {name}</div>
<div class="card" id="detail"><div class="loading">Loading...</div></div>
<div class="card">
  <div class="card-header">Owned Pods</div>
  <div id="pods"><div class="loading">Loading...</div></div>
</div>"#
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
const NS = '{ns}';
const NAME = '{name}';
async function loadDetail() {{
    const dep = await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/deployments/${{NAME}}`).then(r => r.json()).catch(() => null);
    if (!dep) {{ document.getElementById('detail').innerHTML = '<div class="empty">Not found</div>'; return; }}
    let h = '<div class="kv-grid">';
    h += `<div class="kv-key">Replicas</div><div class="kv-val">${{dep.spec?.replicas ?? 1}}</div>`;
    h += `<div class="kv-key">Ready</div><div class="kv-val">${{dep.status?.readyReplicas ?? dep.status?.availableReplicas ?? 0}}</div>`;
    const img = (dep.spec?.template?.spec?.containers||[])[0]?.image || '-';
    h += `<div class="kv-key">Image</div><div class="kv-val" style="word-break:break-all">${{escapeHtml(img)}}</div>`;
    h += `<div class="kv-key">Age</div><div class="kv-val">${{timeSince(dep.metadata?.creationTimestamp)}}</div>`;
    h += '</div>';
    document.getElementById('detail').innerHTML = h;
    // Owned pods — match by name prefix
    const podList = await fetch(MKUBE + '/api/v1/pods').then(r => r.json()).catch(() => null);
    const pods = ((podList && podList.items) || []).filter(p => p.metadata?.namespace === NS && (p.metadata?.name||'').startsWith(NAME));
    let ph = '<table><tr><th>Name</th><th>Status</th><th>IP</th><th>Restarts</th><th>Age</th></tr>';
    for (const p of pods) {{
        const pName = p.metadata?.name || '-';
        ph += `<tr><td><a href="/ui/pods/${{encodeURIComponent(NS)}}/${{encodeURIComponent(pName)}}">${{escapeHtml(pName)}}</a></td><td>${{statusBadge(p.status?.phase||'Unknown')}}</td><td>${{escapeHtml(p.status?.podIP||'-')}}</td><td>${{(p.status?.containerStatuses||[]).reduce((s,c) => s + (c.restartCount||0), 0)}}</td><td>${{timeSince(p.metadata?.creationTimestamp)}}</td></tr>`;
    }}
    ph += '</table>';
    document.getElementById('pods').innerHTML = pods.length ? ph : '<div class="empty">No owned pods</div>';
}}
loadDetail();
"#
    );
    Html(layout::page_with_js("Deployment Detail", "Deployments", &body, &js))
}

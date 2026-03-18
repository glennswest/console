use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Storage</div>
<div class="tabs">
  <div class="tab active" onclick="switchTab('pvcs')">PVCs</div>
  <div class="tab" onclick="switchTab('cdroms')">iSCSI CDROMs</div>
  <div class="tab" onclick="switchTab('disks')">iSCSI Disks</div>
</div>
<div id="tab-pvcs" class="tab-panel active"><div class="loading">Loading...</div></div>
<div id="tab-cdroms" class="tab-panel"><div class="loading">Loading...</div></div>
<div id="tab-disks" class="tab-panel"><div class="loading">Loading...</div></div>
<div class="card" style="margin-top:16px">
  <div class="card-header">Disk Capacity</div>
  <div id="capacity"><div class="loading">Loading...</div></div>
</div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
function switchTab(tab) {{
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.tab-panel').forEach(t => t.classList.remove('active'));
    event.target.classList.add('active');
    document.getElementById('tab-' + tab).classList.add('active');
}}
async function loadStorage() {{
    // PVCs
    const pvcs = await fetch(MKUBE + '/api/v1/persistentvolumeclaims').then(r => r.json()).catch(() => null);
    const pvcItems = (pvcs && pvcs.items) || [];
    let ph = '<table><tr><th>Name</th><th>Namespace</th><th>Status</th><th>Size</th></tr>';
    for (const p of pvcItems) {{
        const name = p.metadata?.name || '-';
        const ns = p.metadata?.namespace || 'default';
        const st = p.status?.phase || '-';
        const size = p.spec?.resources?.requests?.storage || p.spec?.size || '-';
        ph += `<tr><td>${{escapeHtml(name)}}</td><td>${{escapeHtml(ns)}}</td><td>${{statusBadge(st)}}</td><td>${{escapeHtml(String(size))}}</td></tr>`;
    }}
    ph += '</table>';
    document.getElementById('tab-pvcs').innerHTML = pvcItems.length ? ph : '<div class="empty">No PVCs</div>';
    // CDROMs
    const cdroms = await fetch(MKUBE + '/api/v1/iscsi-cdroms').then(r => r.json()).catch(() => null);
    const cdItems = (cdroms && cdroms.items) || [];
    let ch = '<table><tr><th>Name</th><th>Version</th><th>Status</th><th>Subscribers</th></tr>';
    for (const c of cdItems) {{
        const name = c.metadata?.name || '-';
        const ver = c.spec?.version || '-';
        const st = c.status?.phase || '-';
        const subs = c.status?.subscribers || [];
        ch += `<tr><td>${{escapeHtml(name)}}</td><td>${{escapeHtml(ver)}}</td><td>${{statusBadge(st)}}</td><td>${{Array.isArray(subs) ? subs.length : '-'}}</td></tr>`;
    }}
    ch += '</table>';
    document.getElementById('tab-cdroms').innerHTML = cdItems.length ? ch : '<div class="empty">No iSCSI CDROMs</div>';
    // Disks
    const disks = await fetch(MKUBE + '/api/v1/iscsi-disks').then(r => r.json()).catch(() => null);
    const diskItems = (disks && disks.items) || [];
    let dh = '<table><tr><th>Name</th><th>Host</th><th>Size</th><th>Phase</th><th>Source</th></tr>';
    for (const d of diskItems) {{
        const name = d.metadata?.name || '-';
        const host = d.spec?.host || '-';
        const size = d.spec?.size || '-';
        const phase = d.status?.phase || '-';
        const source = d.spec?.source || '-';
        dh += `<tr><td>${{escapeHtml(name)}}</td><td>${{escapeHtml(host)}}</td><td>${{escapeHtml(String(size))}}</td><td>${{statusBadge(phase)}}</td><td>${{escapeHtml(String(source))}}</td></tr>`;
    }}
    dh += '</table>';
    document.getElementById('tab-disks').innerHTML = diskItems.length ? dh : '<div class="empty">No iSCSI disks</div>';
    // Capacity
    const cap = await fetch(MKUBE + '/api/v1/iscsi-disks/capacity').then(r => r.json()).catch(() => null);
    if (cap) {{
        let kh = '<div class="kv-grid">';
        for (const [k,v] of Object.entries(cap)) {{
            kh += `<div class="kv-key">${{escapeHtml(k)}}</div><div class="kv-val">${{escapeHtml(String(v))}}</div>`;
        }}
        kh += '</div>';
        document.getElementById('capacity').innerHTML = kh;
    }} else {{
        document.getElementById('capacity').innerHTML = '<div class="empty">Unable to load</div>';
    }}
}}
loadStorage();
setInterval(loadStorage, 30000);
"#
    );
    Html(layout::page_with_js("Storage", "Storage", body, &js))
}

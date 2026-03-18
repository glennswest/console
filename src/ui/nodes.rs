use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn list_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Nodes</div>
<div id="nodes"><div class="loading">Loading...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
async function loadNodes() {{
    const r = await fetch(MKUBE + '/api/v1/nodes').then(r => r.json()).catch(() => null);
    const nodes = (r && r.items) || [];
    let h = '<table><tr><th>Name</th><th>Architecture</th><th>IP</th><th>OS</th><th>Version</th><th>Status</th><th>Last Heartbeat</th><th>stormd</th></tr>';
    for (const n of nodes) {{
        const name = n.metadata?.name || '-';
        const arch = n.status?.nodeInfo?.architecture || '-';
        const os = n.status?.nodeInfo?.operatingSystem || '-';
        const ver = n.status?.nodeInfo?.kubeletVersion || '-';
        const ip = (n.status?.addresses || []).find(a => a.type === 'InternalIP')?.address || '-';
        const ready = (n.status?.conditions || []).find(c => c.type === 'Ready');
        const st = ready && ready.status === 'True' ? 'Ready' : 'NotReady';
        const hb = ready?.lastHeartbeatTime ? new Date(ready.lastHeartbeatTime).toLocaleString() : '-';
        const stormdUrl = ip !== '-' ? `http://${{ip}}:9080/ui/` : '#';
        h += `<tr style="cursor:pointer" onclick="location.href='/ui/nodes/${{encodeURIComponent(name)}}'">
            <td><a href="/ui/nodes/${{encodeURIComponent(name)}}">${{escapeHtml(name)}}</a></td>
            <td>${{escapeHtml(arch)}}</td>
            <td>${{escapeHtml(ip)}}</td>
            <td>${{escapeHtml(os)}}</td>
            <td>${{escapeHtml(ver)}}</td>
            <td>${{statusBadge(st)}}</td>
            <td style="color:#666">${{escapeHtml(hb)}}</td>
            <td><a href="${{stormdUrl}}" target="_blank" class="btn btn-sm">Open</a></td>
        </tr>`;
    }}
    h += '</table>';
    document.getElementById('nodes').innerHTML = nodes.length ? h : '<div class="empty">No nodes</div>';
}}
loadNodes();
setInterval(loadNodes, 15000);
"#
    );
    Html(layout::page_with_js("Nodes", "Nodes", body, &js))
}

pub async fn detail_page(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title"><a href="/ui/nodes" style="color:#888">Nodes</a> / {name}</div>
<div class="card" id="node-info"><div class="loading">Loading...</div></div>
<div class="grid-2">
  <div class="card">
    <div class="card-header">Processes (stormd)</div>
    <div id="processes"><div class="loading">Loading...</div></div>
  </div>
  <div class="card">
    <div class="card-header">Disk Mounts</div>
    <div id="mounts"><div class="loading">Loading...</div></div>
  </div>
</div>
<div class="card">
  <div class="card-header">Pods on this node</div>
  <div id="pods"><div class="loading">Loading...</div></div>
</div>"#,
        name = name,
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
const NODE_NAME = '{name}';
let stormdUrl = '';
async function loadNode() {{
    // Get node info
    const nodes = await fetch(MKUBE + '/api/v1/nodes').then(r => r.json()).catch(() => null);
    const node = ((nodes && nodes.items) || []).find(n => n.metadata?.name === NODE_NAME);
    if (!node) {{ document.getElementById('node-info').innerHTML = '<div class="empty">Node not found</div>'; return; }}
    const ip = (node.status?.addresses || []).find(a => a.type === 'InternalIP')?.address || '';
    stormdUrl = ip ? `http://${{ip}}:9080` : '';
    let h = '<div class="kv-grid">';
    h += `<div class="kv-key">Architecture</div><div class="kv-val">${{escapeHtml(node.status?.nodeInfo?.architecture||'-')}}</div>`;
    h += `<div class="kv-key">OS</div><div class="kv-val">${{escapeHtml(node.status?.nodeInfo?.operatingSystem||'-')}}</div>`;
    h += `<div class="kv-key">Version</div><div class="kv-val">${{escapeHtml(node.status?.nodeInfo?.kubeletVersion||'-')}}</div>`;
    h += `<div class="kv-key">IP</div><div class="kv-val">${{escapeHtml(ip||'-')}}</div>`;
    const ready = (node.status?.conditions || []).find(c => c.type === 'Ready');
    h += `<div class="kv-key">Status</div><div class="kv-val">${{statusBadge(ready?.status === 'True' ? 'Ready' : 'NotReady')}}</div>`;
    if (stormdUrl) h += `<div class="kv-key">stormd</div><div class="kv-val"><a href="${{stormdUrl}}/ui/" target="_blank">${{stormdUrl}}/ui/</a></div>`;
    h += '</div>';
    document.getElementById('node-info').innerHTML = h;
    // Load stormd processes
    if (stormdUrl) {{
        try {{
            const procs = await fetch(stormdUrl + '/api/v1/processes').then(r => r.json());
            let ph = '<table><tr><th>Name</th><th>State</th><th>PID</th><th>Restarts</th><th>Uptime</th><th>Actions</th></tr>';
            for (const p of procs) {{
                ph += `<tr>
                    <td>${{escapeHtml(p.name||'-')}}</td>
                    <td>${{statusBadge(p.state||'unknown')}}</td>
                    <td>${{p.pid||'-'}}</td>
                    <td>${{p.restarts||0}}</td>
                    <td>${{escapeHtml(p.uptime||'-')}}</td>
                    <td class="btn-group">
                        <button class="btn btn-sm btn-success" onclick="procAction('${{p.name}}','start')">Start</button>
                        <button class="btn btn-sm" onclick="procAction('${{p.name}}','stop')">Stop</button>
                        <button class="btn btn-sm" onclick="procAction('${{p.name}}','restart')">Restart</button>
                    </td>
                </tr>`;
            }}
            ph += '</table>';
            document.getElementById('processes').innerHTML = procs.length ? ph : '<div class="empty">No processes</div>';
        }} catch(e) {{
            document.getElementById('processes').innerHTML = '<div class="empty">stormd unreachable</div>';
        }}
        // Mounts
        try {{
            const mounts = await fetch(stormdUrl + '/api/v1/mounts').then(r => r.json());
            let mh = '<table><tr><th>Mount</th><th>Device</th><th>FS</th><th>Usage</th></tr>';
            for (const m of mounts) {{
                const total = m.total || 1;
                const used = m.used || 0;
                const pct = Math.round(used / total * 100);
                const color = pct > 90 ? '#e94560' : pct > 70 ? '#f1fa8c' : '#50fa7b';
                mh += `<tr>
                    <td>${{escapeHtml(m.mount_point||'-')}}</td>
                    <td style="color:#666">${{escapeHtml(m.device||'-')}}</td>
                    <td>${{escapeHtml(m.fs_type||'-')}}</td>
                    <td><div class="usage-bar"><div class="usage-bar-fill" style="width:${{pct}}%;background:${{color}}"></div><div class="usage-bar-text">${{pct}}%</div></div></td>
                </tr>`;
            }}
            mh += '</table>';
            document.getElementById('mounts').innerHTML = mounts.length ? mh : '<div class="empty">No mounts</div>';
        }} catch(e) {{
            document.getElementById('mounts').innerHTML = '<div class="empty">stormd unreachable</div>';
        }}
    }} else {{
        document.getElementById('processes').innerHTML = '<div class="empty">No stormd URL</div>';
        document.getElementById('mounts').innerHTML = '<div class="empty">No stormd URL</div>';
    }}
    // Pods on this node
    const podList = await fetch(MKUBE + '/api/v1/pods').then(r => r.json()).catch(() => null);
    const pods = ((podList && podList.items) || []).filter(p => p.spec?.nodeName === NODE_NAME);
    let podH = '<table><tr><th>Name</th><th>Namespace</th><th>Status</th><th>IP</th><th>Image</th></tr>';
    for (const p of pods) {{
        const pName = p.metadata?.name || '-';
        const pNs = p.metadata?.namespace || 'default';
        const phase = p.status?.phase || 'Unknown';
        const ip = p.status?.podIP || '-';
        const img = ((p.spec?.containers||[])[0]?.image||'-').split('/').pop();
        podH += `<tr><td><a href="/ui/pods/${{encodeURIComponent(pNs)}}/${{encodeURIComponent(pName)}}">${{escapeHtml(pName)}}</a></td><td>${{escapeHtml(pNs)}}</td><td>${{statusBadge(phase)}}</td><td>${{escapeHtml(ip)}}</td><td>${{escapeHtml(img)}}</td></tr>`;
    }}
    podH += '</table>';
    document.getElementById('pods').innerHTML = pods.length ? podH : '<div class="empty">No pods on this node</div>';
}}
async function procAction(name, action) {{
    if (!stormdUrl) return;
    await fetch(stormdUrl + `/api/v1/processes/${{name}}/${{action}}`, {{method:'POST'}});
    setTimeout(loadNode, 1000);
}}
loadNode();
setInterval(loadNode, 15000);
"#,
        mkube_url = mkube_url, name = name,
    );
    Html(layout::page_with_js("Node Detail", "Nodes", &body, &js))
}

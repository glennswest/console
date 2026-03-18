use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title">Dashboard</div>
<div class="stats-grid" id="stats"><div class="loading">Loading...</div></div>
<div class="grid-2">
  <div>
    <div class="card">
      <div class="card-header">Nodes</div>
      <div id="nodes"><div class="loading">Loading...</div></div>
    </div>
    <div class="card">
      <div class="card-header">Consistency</div>
      <div id="consistency"><div class="loading">Loading...</div></div>
    </div>
  </div>
  <div>
    <div class="card">
      <div class="card-header">Recent Events</div>
      <div id="events" style="max-height:500px;overflow-y:auto"><div class="loading">Loading...</div></div>
    </div>
  </div>
</div>"#
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
async function fetchJSON(url) {{
    try {{
        const r = await fetch(url);
        return await r.json();
    }} catch(e) {{ return null; }}
}}
async function loadDashboard() {{
    const [health, nodeList, podList, netList, bmhList, evtList, consistency] = await Promise.all([
        fetchJSON(MKUBE + '/healthz'),
        fetchJSON(MKUBE + '/api/v1/nodes'),
        fetchJSON(MKUBE + '/api/v1/pods'),
        fetchJSON(MKUBE + '/api/v1/networks'),
        fetchJSON(MKUBE + '/api/v1/baremetalhosts'),
        fetchJSON(MKUBE + '/api/v1/events'),
        fetchJSON(MKUBE + '/api/v1/consistency'),
    ]);
    const nodes = (nodeList && nodeList.items) || [];
    const pods = (podList && podList.items) || [];
    const nets = (netList && netList.items) || [];
    const bmhs = (bmhList && bmhList.items) || [];
    const events = (evtList && evtList.items) || [];
    const running = pods.filter(p => (p.status?.phase||'').toLowerCase() === 'running').length;
    const version = (health && health.version) || '-';
    // Stats
    document.getElementById('stats').innerHTML = `
        <div class="stat-card"><div class="label">Nodes</div><div class="value cyan">${{nodes.length}}</div></div>
        <div class="stat-card"><div class="label">Pods</div><div class="value green">${{pods.length}}</div></div>
        <div class="stat-card"><div class="label">Running</div><div class="value green">${{running}}</div></div>
        <div class="stat-card"><div class="label">Networks</div><div class="value cyan">${{nets.length}}</div></div>
        <div class="stat-card"><div class="label">BMH</div><div class="value yellow">${{bmhs.length}}</div></div>
        <div class="stat-card"><div class="label">Version</div><div class="value" style="font-size:16px;color:#888">${{escapeHtml(version)}}</div></div>
    `;
    // Nodes
    let nh = '<table><tr><th>Name</th><th>Arch</th><th>IP</th><th>Status</th><th>stormd</th></tr>';
    for (const n of nodes) {{
        const name = n.metadata?.name || '-';
        const arch = n.status?.nodeInfo?.architecture || '-';
        const ip = (n.status?.addresses || []).find(a => a.type === 'InternalIP')?.address || '-';
        const ready = (n.status?.conditions || []).find(c => c.type === 'Ready');
        const st = ready && ready.status === 'True' ? 'Ready' : 'NotReady';
        const stormdUrl = ip !== '-' ? `http://${{ip}}:9080/ui/` : '#';
        nh += `<tr><td>${{escapeHtml(name)}}</td><td>${{escapeHtml(arch)}}</td><td>${{escapeHtml(ip)}}</td><td>${{statusBadge(st)}}</td><td><a href="${{stormdUrl}}" target="_blank" class="btn btn-sm">stormd</a></td></tr>`;
    }}
    nh += '</table>';
    document.getElementById('nodes').innerHTML = nodes.length ? nh : '<div class="empty">No nodes</div>';
    // Events
    const sortedEvts = events.sort((a,b) => new Date(b.lastTimestamp||0) - new Date(a.lastTimestamp||0)).slice(0, 50);
    let eh = '';
    for (const e of sortedEvts) {{
        const t = e.lastTimestamp ? new Date(e.lastTimestamp).toLocaleString() : '-';
        const tp = (e.type || 'Normal').toLowerCase();
        const color = tp === 'warning' ? '#f1fa8c' : tp === 'error' ? '#e94560' : '#888';
        const obj = e.involvedObject ? `${{e.involvedObject.kind||''}}/${{e.involvedObject.name||''}}` : '';
        eh += `<div class="event-row"><span class="event-time">${{escapeHtml(t)}}</span><span class="event-reason" style="color:${{color}}">${{escapeHtml(e.reason||'')}}</span><span style="color:#8be9fd">${{escapeHtml(obj)}}</span> ${{escapeHtml(e.message||'')}}</div>`;
    }}
    document.getElementById('events').innerHTML = eh || '<div class="empty">No events</div>';
    // Consistency
    if (consistency) {{
        const summary = consistency.summary || consistency;
        let ch = '<div class="kv-grid">';
        for (const [k,v] of Object.entries(summary)) {{
            const val = typeof v === 'object' ? JSON.stringify(v) : String(v);
            const color = val === 'pass' || val === 'ok' ? '#50fa7b' : val === 'fail' || val === 'error' ? '#e94560' : '#e0e0e0';
            ch += `<div class="kv-key">${{escapeHtml(k)}}</div><div class="kv-val" style="color:${{color}}">${{escapeHtml(val)}}</div>`;
        }}
        ch += '</div>';
        document.getElementById('consistency').innerHTML = ch;
    }} else {{
        document.getElementById('consistency').innerHTML = '<div class="empty">Unable to load</div>';
    }}
}}
loadDashboard();
setInterval(loadDashboard, 30000);
"#
    );
    Html(layout::page_with_js("Dashboard", "Dashboard", &body, &js))
}

use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn list_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Bare Metal Hosts</div>
<div id="bmh"><div class="loading">Loading...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
async function loadBMH() {{
    const r = await fetch(MKUBE + '/api/v1/baremetalhosts').then(r => r.json()).catch(() => null);
    const bmhs = (r && r.items) || [];
    let h = '<table><tr><th>Name</th><th>Namespace</th><th>State</th><th>Power</th><th>Network</th><th>IP</th><th>Image</th><th>Actions</th></tr>';
    for (const b of bmhs) {{
        const name = b.metadata?.name || '-';
        const ns = b.metadata?.namespace || 'default';
        const spec = b.spec || {{}};
        const status = b.status || {{}};
        const state = status.state || spec.state || '-';
        const online = spec.online;
        const power = online === true ? 'on' : online === false ? 'off' : '-';
        const net = spec.network || '-';
        const ip = spec.ip || '-';
        const img = (spec.image || spec.disk || '-').toString().split('/').pop();
        h += `<tr style="cursor:pointer" onclick="location.href='/ui/bmh/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}'">
            <td><a href="/ui/bmh/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}">${{escapeHtml(name)}}</a></td>
            <td>${{escapeHtml(ns)}}</td>
            <td>${{statusBadge(state)}}</td>
            <td>${{power === 'on' ? '<span class="badge badge-green">ON</span>' : power === 'off' ? '<span class="badge badge-gray">OFF</span>' : '-'}}</td>
            <td>${{escapeHtml(net)}}</td>
            <td>${{escapeHtml(ip)}}</td>
            <td title="${{escapeHtml(spec.image||spec.disk||'')}}">${{escapeHtml(img)}}</td>
            <td class="btn-group">
                <button class="btn btn-sm btn-success" onclick="event.stopPropagation();togglePower('${{ns}}','${{name}}',true)">On</button>
                <button class="btn btn-sm btn-danger" onclick="event.stopPropagation();togglePower('${{ns}}','${{name}}',false)">Off</button>
            </td>
        </tr>`;
    }}
    h += '</table>';
    document.getElementById('bmh').innerHTML = bmhs.length ? h : '<div class="empty">No bare metal hosts</div>';
}}
async function togglePower(ns, name, on) {{
    await fetch(MKUBE + `/api/v1/namespaces/${{ns}}/baremetalhosts/${{name}}`, {{
        method: 'PATCH',
        headers: {{'Content-Type': 'application/merge-patch+json'}},
        body: JSON.stringify({{spec: {{online: on}}}})
    }});
    setTimeout(loadBMH, 1000);
}}
loadBMH();
setInterval(loadBMH, 15000);
"#
    );
    Html(layout::page_with_js("Bare Metal Hosts", "BMH", body, &js))
}

pub async fn detail_page(
    State(state): State<Arc<AppState>>,
    Path((ns, name)): Path<(String, String)>,
) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title"><a href="/ui/bmh" style="color:#888">BMH</a> / <span style="color:#8be9fd">{ns}</span> / {name}</div>
<div class="card" id="detail"><div class="loading">Loading...</div></div>"#
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
const NS = '{ns}';
const NAME = '{name}';
async function loadDetail() {{
    const bmh = await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/baremetalhosts/${{NAME}}`).then(r => r.json()).catch(() => null);
    if (!bmh) {{ document.getElementById('detail').innerHTML = '<div class="empty">Not found</div>'; return; }}
    const spec = bmh.spec || {{}};
    const status = bmh.status || {{}};
    let h = '<div class="kv-grid">';
    // Spec fields
    for (const k of ['online','state','network','ip','hostname','mac','image','disk','bootConfigRef']) {{
        if (spec[k] !== undefined) h += `<div class="kv-key">${{k}}</div><div class="kv-val">${{escapeHtml(String(spec[k]))}}</div>`;
    }}
    // BMC fields
    const bmc = spec.bmc || {{}};
    if (bmc.address) h += `<div class="kv-key">BMC Address</div><div class="kv-val">${{escapeHtml(bmc.address)}}</div>`;
    if (bmc.network) h += `<div class="kv-key">BMC Network</div><div class="kv-val">${{escapeHtml(bmc.network)}}</div>`;
    if (bmc.mac) h += `<div class="kv-key">BMC MAC</div><div class="kv-val" style="font-family:monospace">${{escapeHtml(bmc.mac)}}</div>`;
    if (bmc.hostname) h += `<div class="kv-key">BMC Hostname</div><div class="kv-val">${{escapeHtml(bmc.hostname)}}</div>`;
    // Status
    for (const [k,v] of Object.entries(status)) {{
        const val = typeof v === 'object' ? JSON.stringify(v) : String(v);
        h += `<div class="kv-key">${{escapeHtml(k)}}</div><div class="kv-val">${{escapeHtml(val)}}</div>`;
    }}
    h += '</div>';
    h += `<div style="margin-top:12px" class="btn-group">
        <button class="btn btn-sm btn-success" onclick="togglePower(true)">Power On</button>
        <button class="btn btn-sm btn-danger" onclick="togglePower(false)">Power Off</button>
    </div>`;
    document.getElementById('detail').innerHTML = h;
}}
async function togglePower(on) {{
    await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/baremetalhosts/${{NAME}}`, {{
        method: 'PATCH',
        headers: {{'Content-Type': 'application/merge-patch+json'}},
        body: JSON.stringify({{spec: {{online: on}}}})
    }});
    setTimeout(loadDetail, 1000);
}}
loadDetail();
setInterval(loadDetail, 15000);
"#
    );
    Html(layout::page_with_js("BMH Detail", "BMH", &body, &js))
}

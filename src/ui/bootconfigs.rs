use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn list_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Boot Configs</div>
<div id="bootconfigs"><div class="loading">Loading...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
async function loadBootConfigs() {{
    const r = await fetch(MKUBE + '/api/v1/bootconfigs').then(r => r.json()).catch(() => null);
    const items = (r && r.items) || [];
    let h = '<table><tr><th>Name</th><th>Namespace</th><th>Kernel</th><th>Initrd</th><th>iPXE</th><th>Age</th><th>Actions</th></tr>';
    for (const b of items) {{
        const name = b.metadata?.name || '-';
        const ns = b.metadata?.namespace || 'default';
        const spec = b.spec || {{}};
        const kernel = spec.kernel || '-';
        const initrd = spec.initrd || '-';
        const ipxe = spec.ipxeScript ? 'yes' : '-';
        const age = timeSince(b.metadata?.creationTimestamp);
        h += `<tr style="cursor:pointer" onclick="location.href='/ui/bootconfigs/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}'">
            <td><a href="/ui/bootconfigs/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}">${{escapeHtml(name)}}</a></td>
            <td>${{escapeHtml(ns)}}</td>
            <td title="${{escapeHtml(kernel)}}">${{escapeHtml(kernel.split('/').pop())}}</td>
            <td title="${{escapeHtml(initrd)}}">${{escapeHtml(initrd.split('/').pop())}}</td>
            <td>${{ipxe}}</td>
            <td>${{age}}</td>
            <td><button class="btn btn-danger btn-sm" onclick="event.stopPropagation();deleteBC('${{ns}}','${{name}}')">Delete</button></td>
        </tr>`;
    }}
    h += '</table>';
    document.getElementById('bootconfigs').innerHTML = items.length ? h : '<div class="empty">No boot configs</div>';
}}
async function deleteBC(ns, name) {{
    if (!confirm('Delete boot config ' + name + '?')) return;
    await fetch(MKUBE + `/api/v1/namespaces/${{ns}}/bootconfigs/${{name}}`, {{method:'DELETE'}});
    setTimeout(loadBootConfigs, 1000);
}}
loadBootConfigs();
setInterval(loadBootConfigs, 15000);
"#
    );
    Html(layout::page_with_js("Boot Configs", "BootConfigs", body, &js))
}

pub async fn detail_page(
    State(state): State<Arc<AppState>>,
    Path((ns, name)): Path<(String, String)>,
) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title"><a href="/ui/bootconfigs" style="color:#888">BootConfigs</a> / <span style="color:#8be9fd">{ns}</span> / {name}</div>
<div class="card" id="detail"><div class="loading">Loading...</div></div>
<div class="card">
  <div class="card-header">Referencing Hosts</div>
  <div id="refs"><div class="loading">Loading...</div></div>
</div>"#
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
const NS = '{ns}';
const NAME = '{name}';
async function loadDetail() {{
    const bc = await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/bootconfigs/${{NAME}}`).then(r => r.json()).catch(() => null);
    if (!bc) {{ document.getElementById('detail').innerHTML = '<div class="empty">Not found</div>'; return; }}
    const spec = bc.spec || {{}};
    const status = bc.status || {{}};
    let h = '<div class="kv-grid">';
    for (const k of ['kernel','initrd','cmdline','ipxeScript','diskImage','installScript']) {{
        if (spec[k] !== undefined && spec[k] !== null) {{
            const val = typeof spec[k] === 'object' ? JSON.stringify(spec[k]) : String(spec[k]);
            h += `<div class="kv-key">${{k}}</div><div class="kv-val" style="word-break:break-all">${{escapeHtml(val)}}</div>`;
        }}
    }}
    // Show any remaining spec fields not in the known list
    const knownKeys = new Set(['kernel','initrd','cmdline','ipxeScript','diskImage','installScript']);
    for (const [k,v] of Object.entries(spec)) {{
        if (knownKeys.has(k)) continue;
        const val = typeof v === 'object' ? JSON.stringify(v) : String(v);
        h += `<div class="kv-key">${{escapeHtml(k)}}</div><div class="kv-val" style="word-break:break-all">${{escapeHtml(val)}}</div>`;
    }}
    // Status
    for (const [k,v] of Object.entries(status)) {{
        const val = typeof v === 'object' ? JSON.stringify(v) : String(v);
        h += `<div class="kv-key">${{escapeHtml(k)}}</div><div class="kv-val">${{escapeHtml(val)}}</div>`;
    }}
    h += '</div>';
    // Labels
    const labels = bc.metadata?.labels;
    if (labels && typeof labels === 'object' && Object.keys(labels).length) {{
        h += '<div style="margin-top:12px"><strong style="color:#888;font-size:12px">LABELS</strong></div><div class="kv-grid">';
        for (const [k,v] of Object.entries(labels)) {{
            h += `<div class="kv-key">${{escapeHtml(k)}}</div><div class="kv-val">${{escapeHtml(String(v))}}</div>`;
        }}
        h += '</div>';
    }}
    document.getElementById('detail').innerHTML = h;
    // Find BMH referencing this boot config
    const bmhList = await fetch(MKUBE + '/api/v1/baremetalhosts').then(r => r.json()).catch(() => null);
    const bmhs = ((bmhList && bmhList.items) || []).filter(b => b.spec?.bootConfigRef === NAME);
    if (bmhs.length) {{
        let rh = '<table><tr><th>Name</th><th>Namespace</th><th>State</th><th>IP</th></tr>';
        for (const b of bmhs) {{
            const bName = b.metadata?.name || '-';
            const bNs = b.metadata?.namespace || 'default';
            const bState = b.status?.state || b.spec?.state || '-';
            const bIp = b.spec?.ip || '-';
            rh += `<tr style="cursor:pointer" onclick="location.href='/ui/bmh/${{encodeURIComponent(bNs)}}/${{encodeURIComponent(bName)}}'">
                <td><a href="/ui/bmh/${{encodeURIComponent(bNs)}}/${{encodeURIComponent(bName)}}">${{escapeHtml(bName)}}</a></td>
                <td>${{escapeHtml(bNs)}}</td>
                <td>${{statusBadge(bState)}}</td>
                <td>${{escapeHtml(bIp)}}</td>
            </tr>`;
        }}
        rh += '</table>';
        document.getElementById('refs').innerHTML = rh;
    }} else {{
        document.getElementById('refs').innerHTML = '<div class="empty">No hosts reference this boot config</div>';
    }}
}}
loadDetail();
setInterval(loadDetail, 15000);
"#
    );
    Html(layout::page_with_js("BootConfig Detail", "BootConfigs", &body, &js))
}

use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn list_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Registries</div>
<div id="registries"><div class="loading">Loading...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
async function loadRegistries() {{
    const r = await fetch(MKUBE + '/api/v1/registries').then(r => r.json()).catch(() => null);
    const items = (r && r.items) || [];
    let h = '<table><tr><th>Name</th><th>Namespace</th><th>URL</th><th>Mirrors</th><th>Insecure</th><th>Age</th><th>Actions</th></tr>';
    for (const reg of items) {{
        const name = reg.metadata?.name || '-';
        const ns = reg.metadata?.namespace || 'default';
        const spec = reg.spec || {{}};
        const url = spec.url || spec.endpoint || '-';
        const mirrors = Array.isArray(spec.mirrors) ? spec.mirrors.length : '-';
        const insecure = spec.insecure === true ? '<span class="badge badge-yellow">yes</span>' : '<span class="badge badge-green">no</span>';
        const age = timeSince(reg.metadata?.creationTimestamp);
        h += `<tr style="cursor:pointer" onclick="location.href='/ui/registries/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}'">
            <td><a href="/ui/registries/${{encodeURIComponent(ns)}}/${{encodeURIComponent(name)}}">${{escapeHtml(name)}}</a></td>
            <td>${{escapeHtml(ns)}}</td>
            <td style="font-family:monospace">${{escapeHtml(url)}}</td>
            <td>${{mirrors}}</td>
            <td>${{insecure}}</td>
            <td>${{age}}</td>
            <td><button class="btn btn-danger btn-sm" onclick="event.stopPropagation();deleteReg('${{ns}}','${{name}}')">Delete</button></td>
        </tr>`;
    }}
    h += '</table>';
    document.getElementById('registries').innerHTML = items.length ? h : '<div class="empty">No registries</div>';
}}
async function deleteReg(ns, name) {{
    if (!confirm('Delete registry ' + name + '?')) return;
    await fetch(MKUBE + `/api/v1/namespaces/${{ns}}/registries/${{name}}`, {{method:'DELETE'}});
    setTimeout(loadRegistries, 1000);
}}
loadRegistries();
setInterval(loadRegistries, 15000);
"#
    );
    Html(layout::page_with_js("Registries", "Registries", body, &js))
}

pub async fn detail_page(
    State(state): State<Arc<AppState>>,
    Path((ns, name)): Path<(String, String)>,
) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title"><a href="/ui/registries" style="color:#888">Registries</a> / <span style="color:#8be9fd">{ns}</span> / {name}</div>
<div class="card" id="detail"><div class="loading">Loading...</div></div>
<div class="card">
  <div class="card-header">Images</div>
  <div id="images"><div class="loading">Loading...</div></div>
</div>"#
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
const NS = '{ns}';
const NAME = '{name}';
async function loadDetail() {{
    const reg = await fetch(MKUBE + `/api/v1/namespaces/${{NS}}/registries/${{NAME}}`).then(r => r.json()).catch(() => null);
    if (!reg) {{ document.getElementById('detail').innerHTML = '<div class="empty">Not found</div>'; return; }}
    const spec = reg.spec || {{}};
    const status = reg.status || {{}};
    let h = '<div class="kv-grid">';
    // Core spec fields
    for (const k of ['url','endpoint','insecure','interval','push']) {{
        if (spec[k] !== undefined && spec[k] !== null) {{
            const val = typeof spec[k] === 'object' ? JSON.stringify(spec[k]) : String(spec[k]);
            h += `<div class="kv-key">${{k}}</div><div class="kv-val" style="word-break:break-all">${{escapeHtml(val)}}</div>`;
        }}
    }}
    // Mirrors
    if (Array.isArray(spec.mirrors) && spec.mirrors.length) {{
        h += `<div class="kv-key">mirrors</div><div class="kv-val" style="font-family:monospace">${{spec.mirrors.map(m => escapeHtml(m)).join('<br>')}}</div>`;
    }}
    // Remaining spec fields
    const knownKeys = new Set(['url','endpoint','insecure','interval','push','mirrors']);
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
    const labels = reg.metadata?.labels;
    if (labels && typeof labels === 'object' && Object.keys(labels).length) {{
        h += '<div style="margin-top:12px"><strong style="color:#888;font-size:12px">LABELS</strong></div><div class="kv-grid">';
        for (const [k,v] of Object.entries(labels)) {{
            h += `<div class="kv-key">${{escapeHtml(k)}}</div><div class="kv-val">${{escapeHtml(String(v))}}</div>`;
        }}
        h += '</div>';
    }}
    document.getElementById('detail').innerHTML = h;
    // Images (if the registry status has an images list or we can fetch catalog)
    const images = status.images || status.catalog || [];
    if (Array.isArray(images) && images.length) {{
        let ih = '<table><tr><th>Image</th><th>Tag</th><th>Size</th></tr>';
        for (const img of images) {{
            if (typeof img === 'string') {{
                ih += `<tr><td>${{escapeHtml(img)}}</td><td>-</td><td>-</td></tr>`;
            }} else {{
                ih += `<tr><td>${{escapeHtml(img.name || img.image || '-')}}</td><td>${{escapeHtml(img.tag || img.version || '-')}}</td><td>${{escapeHtml(img.size || '-')}}</td></tr>`;
            }}
        }}
        ih += '</table>';
        document.getElementById('images').innerHTML = ih;
    }} else {{
        document.getElementById('images').innerHTML = '<div class="empty">No image catalog available</div>';
    }}
}}
loadDetail();
setInterval(loadDetail, 15000);
"#
    );
    Html(layout::page_with_js("Registry Detail", "Registries", &body, &js))
}

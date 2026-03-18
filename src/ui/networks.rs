use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn list_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Networks</div>
<div id="networks"><div class="loading">Loading...</div></div>"#;
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
async function loadNetworks() {{
    const r = await fetch(MKUBE + '/api/v1/networks').then(r => r.json()).catch(() => null);
    const nets = (r && r.items) || [];
    let h = '<table><tr><th>Name</th><th>Type</th><th>CIDR</th><th>Gateway</th><th>DNS</th><th>Pods</th><th>DNS Alive</th><th>DHCP</th></tr>';
    for (const n of nets) {{
        const name = n.metadata?.name || '-';
        const spec = n.spec || {{}};
        const status = n.status || {{}};
        const ntype = spec.type || '-';
        const cidr = spec.cidr || '-';
        const gw = spec.gateway || '-';
        const dns = spec.dns || '-';
        const pods = status.podCount ?? '-';
        const dnsAlive = status.dnsLiveness;
        const dnsStatus = dnsAlive === true ? 'healthy' : dnsAlive === false ? 'down' : '-';
        const dhcp = spec.dhcp?.enabled ? 'Yes' : 'No';
        h += `<tr style="cursor:pointer" onclick="location.href='/ui/networks/${{encodeURIComponent(name)}}'">
            <td><a href="/ui/networks/${{encodeURIComponent(name)}}">${{escapeHtml(name)}}</a></td>
            <td>${{statusBadge(ntype)}}</td>
            <td>${{escapeHtml(cidr)}}</td>
            <td>${{escapeHtml(gw)}}</td>
            <td>${{escapeHtml(dns)}}</td>
            <td>${{pods}}</td>
            <td>${{statusBadge(dnsStatus)}}</td>
            <td>${{dhcp}}</td>
        </tr>`;
    }}
    h += '</table>';
    document.getElementById('networks').innerHTML = nets.length ? h : '<div class="empty">No networks</div>';
}}
loadNetworks();
setInterval(loadNetworks, 15000);
"#
    );
    Html(layout::page_with_js("Networks", "Networks", body, &js))
}

pub async fn detail_page(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = format!(
        r#"<div class="page-title"><a href="/ui/networks" style="color:#888">Networks</a> / {name}</div>
<div class="card" id="info"><div class="loading">Loading...</div></div>
<div style="margin-bottom:8px">
  <button class="btn btn-sm" onclick="smoketest()" id="smoketest-btn">Run Smoketest</button>
  <span id="smoketest-result" style="margin-left:8px;font-size:12px"></span>
</div>
<div class="tabs">
  <div class="tab active" onclick="switchTab('dns')">DNS Records</div>
  <div class="tab" onclick="switchTab('pools')">DHCP Pools</div>
  <div class="tab" onclick="switchTab('reservations')">DHCP Reservations</div>
  <div class="tab" onclick="switchTab('leases')">DHCP Leases</div>
  <div class="tab" onclick="switchTab('forwarders')">DNS Forwarders</div>
</div>
<div id="tab-dns" class="tab-panel active"><div class="loading">Loading...</div></div>
<div id="tab-pools" class="tab-panel"><div class="loading">Loading...</div></div>
<div id="tab-reservations" class="tab-panel"><div class="loading">Loading...</div></div>
<div id="tab-leases" class="tab-panel"><div class="loading">Loading...</div></div>
<div id="tab-forwarders" class="tab-panel"><div class="loading">Loading...</div></div>"#
    );
    let js = format!(
        r#"
const MKUBE = '{mkube_url}';
const NET = '{name}';
function switchTab(tab) {{
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.tab-panel').forEach(t => t.classList.remove('active'));
    event.target.classList.add('active');
    document.getElementById('tab-' + tab).classList.add('active');
}}
async function loadNetwork() {{
    const net = await fetch(MKUBE + `/api/v1/networks/${{NET}}`).then(r => r.json()).catch(() => null);
    if (!net) {{ document.getElementById('info').innerHTML = '<div class="empty">Not found</div>'; return; }}
    const spec = net.spec || {{}};
    const status = net.status || {{}};
    let h = '<div class="kv-grid">';
    for (const k of ['type','cidr','gateway','dns','bridge','vlan','router','managed','externalDNS']) {{
        if (spec[k] !== undefined) h += `<div class="kv-key">${{k}}</div><div class="kv-val">${{escapeHtml(String(spec[k]))}}</div>`;
    }}
    if (status.podCount !== undefined) h += `<div class="kv-key">Pod Count</div><div class="kv-val">${{status.podCount}}</div>`;
    h += '</div>';
    document.getElementById('info').innerHTML = h;
    // DNS Records
    const dns = await fetch(MKUBE + `/api/v1/namespaces/${{NET}}/dnsrecords`).then(r => r.json()).catch(() => null);
    const dnsItems = (dns && dns.items) || [];
    let dh = '<table><tr><th>Name</th><th>Type</th><th>Data</th><th>TTL</th></tr>';
    for (const r of dnsItems) {{
        const spec = r.spec || {{}};
        dh += `<tr><td>${{escapeHtml(r.metadata?.name||spec.name||'-')}}</td><td>${{escapeHtml(spec.type||spec.data?.type||'-')}}</td><td style="word-break:break-all">${{escapeHtml(typeof spec.data === 'object' ? JSON.stringify(spec.data?.data||spec.data) : String(spec.data||'-'))}}</td><td>${{spec.ttl||'-'}}</td></tr>`;
    }}
    dh += '</table>';
    document.getElementById('tab-dns').innerHTML = dnsItems.length ? dh : '<div class="empty">No DNS records</div>';
    // DHCP Pools
    const pools = await fetch(MKUBE + `/api/v1/namespaces/${{NET}}/dhcppools`).then(r => r.json()).catch(() => null);
    const poolItems = (pools && pools.items) || [];
    let ph = '<table><tr><th>Name</th><th>Range Start</th><th>Range End</th><th>Subnet</th><th>Gateway</th><th>Lease Time</th></tr>';
    for (const p of poolItems) {{
        const s = p.spec || {{}};
        ph += `<tr><td>${{escapeHtml(p.metadata?.name||'-')}}</td><td>${{escapeHtml(s.range_start||s.rangeStart||'-')}}</td><td>${{escapeHtml(s.range_end||s.rangeEnd||'-')}}</td><td>${{escapeHtml(s.subnet||'-')}}</td><td>${{escapeHtml(s.gateway||'-')}}</td><td>${{s.lease_time_secs||s.leaseTimeSecs||'-'}}s</td></tr>`;
    }}
    ph += '</table>';
    document.getElementById('tab-pools').innerHTML = poolItems.length ? ph : '<div class="empty">No DHCP pools</div>';
    // DHCP Reservations
    const res = await fetch(MKUBE + `/api/v1/namespaces/${{NET}}/dhcpreservations`).then(r => r.json()).catch(() => null);
    const resItems = (res && res.items) || [];
    let rh = '<table><tr><th>Name</th><th>MAC</th><th>IP</th><th>Hostname</th></tr>';
    for (const r of resItems) {{
        const s = r.spec || {{}};
        rh += `<tr><td>${{escapeHtml(r.metadata?.name||'-')}}</td><td style="font-family:monospace">${{escapeHtml(s.mac||'-')}}</td><td>${{escapeHtml(s.ip||'-')}}</td><td>${{escapeHtml(s.hostname||'-')}}</td></tr>`;
    }}
    rh += '</table>';
    document.getElementById('tab-reservations').innerHTML = resItems.length ? rh : '<div class="empty">No reservations</div>';
    // DHCP Leases
    const leases = await fetch(MKUBE + `/api/v1/namespaces/${{NET}}/dhcpleases`).then(r => r.json()).catch(() => null);
    const leaseItems = (leases && leases.items) || [];
    let lh = '<table><tr><th>Name</th><th>MAC</th><th>IP</th><th>Hostname</th><th>Expires</th></tr>';
    for (const l of leaseItems) {{
        const s = l.spec || {{}};
        lh += `<tr><td>${{escapeHtml(l.metadata?.name||'-')}}</td><td style="font-family:monospace">${{escapeHtml(s.mac||'-')}}</td><td>${{escapeHtml(s.ip||'-')}}</td><td>${{escapeHtml(s.hostname||'-')}}</td><td>${{escapeHtml(s.expires||'-')}}</td></tr>`;
    }}
    lh += '</table>';
    document.getElementById('tab-leases').innerHTML = leaseItems.length ? lh : '<div class="empty">No leases</div>';
    // Forwarders
    const fwd = await fetch(MKUBE + `/api/v1/namespaces/${{NET}}/dnsforwarders`).then(r => r.json()).catch(() => null);
    const fwdItems = (fwd && fwd.items) || [];
    let fh = '<table><tr><th>Name</th><th>Zone</th><th>Servers</th></tr>';
    for (const f of fwdItems) {{
        const s = f.spec || {{}};
        fh += `<tr><td>${{escapeHtml(f.metadata?.name||'-')}}</td><td>${{escapeHtml(s.zone||'-')}}</td><td>${{escapeHtml(Array.isArray(s.servers) ? s.servers.join(', ') : String(s.servers||'-'))}}</td></tr>`;
    }}
    fh += '</table>';
    document.getElementById('tab-forwarders').innerHTML = fwdItems.length ? fh : '<div class="empty">No forwarders</div>';
}}
async function smoketest() {{
    document.getElementById('smoketest-result').textContent = 'Running...';
    document.getElementById('smoketest-result').style.color = '#888';
    try {{
        const r = await fetch(MKUBE + `/api/v1/networks/${{NET}}/smoketest`, {{method:'POST'}}).then(r => r.json());
        const ok = r.success || r.status === 'pass';
        document.getElementById('smoketest-result').textContent = ok ? 'PASS' : JSON.stringify(r);
        document.getElementById('smoketest-result').style.color = ok ? '#50fa7b' : '#e94560';
    }} catch(e) {{
        document.getElementById('smoketest-result').textContent = 'Error: ' + e;
        document.getElementById('smoketest-result').style.color = '#e94560';
    }}
}}
loadNetwork();
"#
    );
    Html(layout::page_with_js("Network Detail", "Networks", &body, &js))
}

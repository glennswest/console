/// Shared Dracula-themed CSS matching stormd's dark UI.
pub fn css() -> &'static str {
    r#"
* { margin: 0; padding: 0; box-sizing: border-box; }
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0f0f1a;
    color: #e0e0e0;
    font-size: 14px;
    line-height: 1.5;
}
a { color: #8be9fd; text-decoration: none; }
a:hover { text-decoration: underline; }

/* Navigation */
nav {
    display: flex;
    align-items: center;
    height: 48px;
    padding: 0 20px;
    background: #16192e;
    border-bottom: 1px solid #2a2d45;
    gap: 8px;
    position: sticky;
    top: 0;
    z-index: 100;
}
nav .brand { font-size: 18px; font-weight: 700; color: #e94560; }
nav .links { display: flex; gap: 4px; margin-left: 16px; }
nav .links a {
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    color: #888;
    transition: background 0.15s;
}
nav .links a:hover { background: #1e2140; color: #e0e0e0; text-decoration: none; }
nav .links a.active { background: #2a2d50; color: #8be9fd; }

/* Content area */
.content { padding: 20px; max-width: 1600px; margin: 0 auto; }
.page-title { font-size: 20px; font-weight: 700; margin-bottom: 16px; color: #e0e0e0; }
.section-title { font-size: 14px; font-weight: 600; color: #888; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 10px; }

/* Cards */
.card {
    background: #16192e;
    border: 1px solid #2a2d45;
    border-radius: 8px;
    padding: 16px 20px;
    margin-bottom: 16px;
}
.card-header {
    font-size: 14px;
    font-weight: 600;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 12px;
    padding-bottom: 8px;
    border-bottom: 1px solid #2a2d45;
}

/* Stats grid */
.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 12px;
    margin-bottom: 16px;
}
.stat-card {
    background: #16192e;
    border: 1px solid #2a2d45;
    border-radius: 8px;
    padding: 16px;
}
.stat-card .label {
    font-size: 11px;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 4px;
}
.stat-card .value { font-size: 24px; font-weight: 700; }
.stat-card .value.green { color: #50fa7b; }
.stat-card .value.red { color: #e94560; }
.stat-card .value.yellow { color: #f1fa8c; }
.stat-card .value.cyan { color: #8be9fd; }

/* Tables */
table { width: 100%; border-collapse: collapse; }
th {
    text-align: left;
    padding: 10px 12px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #666;
    border-bottom: 1px solid #2a2d45;
}
td { padding: 10px 12px; font-size: 13px; border-bottom: 1px solid #1a1d32; }
tr:hover { background: #1a1d32; }
td a { color: #8be9fd; }

/* Badges */
.badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}
.badge-green { background: #1a4a2a; color: #50fa7b; }
.badge-red { background: #4a1a2a; color: #e94560; }
.badge-yellow { background: #4a3a1a; color: #f1fa8c; }
.badge-cyan { background: #1a3a4a; color: #8be9fd; }
.badge-gray { background: #2a2d45; color: #888; }
.badge-magenta { background: #4a1a4a; color: #ff79c6; }

/* Buttons */
.btn {
    display: inline-block;
    padding: 5px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    border: 1px solid #2a2d45;
    background: #16192e;
    color: #e0e0e0;
    cursor: pointer;
    transition: background 0.15s;
}
.btn:hover { background: #1e2140; text-decoration: none; }
.btn-danger { border-color: #4a1a2a; color: #e94560; }
.btn-danger:hover { background: #3a1020; }
.btn-success { border-color: #1a4a2a; color: #50fa7b; }
.btn-success:hover { background: #1a3a20; }
.btn-sm { padding: 3px 8px; font-size: 11px; }
.btn-group { display: flex; gap: 4px; }

/* Tabs */
.tabs { display: flex; gap: 4px; margin-bottom: 16px; border-bottom: 1px solid #2a2d45; padding-bottom: 0; }
.tab {
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    color: #888;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: color 0.15s;
}
.tab:hover { color: #e0e0e0; }
.tab.active { color: #8be9fd; border-bottom-color: #8be9fd; }
.tab-panel { display: none; }
.tab-panel.active { display: block; }

/* Terminal / log output */
.term-output {
    background: #0a0a14;
    border: 1px solid #2a2d45;
    border-radius: 8px;
    padding: 12px;
    font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
    font-size: 13px;
    line-height: 1.5;
    overflow-y: auto;
    white-space: pre-wrap;
    max-height: calc(100vh - 200px);
    min-height: 300px;
}
.log-entry { padding: 1px 0; }

/* Usage bars */
.usage-bar { background: #1a1d32; border-radius: 4px; height: 18px; overflow: hidden; position: relative; }
.usage-bar-fill { height: 100%; border-radius: 4px; transition: width 0.3s; }
.usage-bar-text { position: absolute; top: 0; left: 8px; right: 8px; height: 18px; line-height: 18px; font-size: 11px; color: #ccc; }

/* Two-column layout */
.grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
@media (max-width: 1000px) { .grid-2 { grid-template-columns: 1fr; } }

/* Events timeline */
.event-row { padding: 6px 0; border-bottom: 1px solid #1a1d32; font-size: 13px; }
.event-time { color: #666; font-size: 12px; margin-right: 8px; }
.event-reason { color: #8be9fd; font-weight: 600; margin-right: 8px; }

/* Loading / empty states */
.loading { color: #666; text-align: center; padding: 40px; }
.empty { color: #555; text-align: center; padding: 40px; font-style: italic; }

/* Filter bar */
.filter-bar { display: flex; gap: 8px; margin-bottom: 12px; align-items: center; flex-wrap: wrap; }
.filter-bar select, .filter-bar input {
    padding: 6px 10px;
    border-radius: 6px;
    border: 1px solid #2a2d45;
    background: #16192e;
    color: #e0e0e0;
    font-size: 13px;
}
.filter-bar label { font-size: 12px; color: #888; }

/* Detail key-value */
.kv-grid { display: grid; grid-template-columns: 140px 1fr; gap: 4px 16px; font-size: 13px; }
.kv-key { color: #666; font-weight: 600; }
.kv-val { color: #e0e0e0; }
"#
}

/// ANSI to HTML converter (JavaScript).
pub fn ansi_js() -> &'static str {
    r#"
function escapeHtml(t) {
    return t.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
}
function ansiToHtml(text) {
    const cm = {
        '1':'font-weight:bold','2':'opacity:0.7','3':'font-style:italic','4':'text-decoration:underline',
        '30':'color:#555','31':'color:#e94560','32':'color:#50fa7b','33':'color:#f1fa8c',
        '34':'color:#6272a4','35':'color:#ff79c6','36':'color:#8be9fd','37':'color:#ccc',
        '90':'color:#666','91':'color:#ff6e6e','92':'color:#69ff94','93':'color:#ffffa5',
        '94':'color:#d6acff','95':'color:#ff92df','96':'color:#a4ffff','97':'color:#fff'
    };
    text = text.replace(/\x1b\[\d*[ABCDHJ]/g, '');
    text = text.replace(/\x1b\[\d*;\d*[Hf]/g, '');
    text = text.replace(/\x1b\[\??\d*[hlr]/g, '');
    let r = '', o = 0;
    const p = text.split(/\x1b\[/);
    r += escapeHtml(p[0]);
    for (let i = 1; i < p.length; i++) {
        const m = p[i].match(/^([\d;]*)m([\s\S]*)/);
        if (m) {
            const codes = m[1], rest = m[2];
            if (codes === '0' || codes === '') { while (o > 0) { r += '</span>'; o--; } }
            else {
                const s = [];
                for (const c of codes.split(';')) { if (cm[c]) s.push(cm[c]); }
                if (s.length > 0) { r += '<span style="' + s.join(';') + '">'; o++; }
            }
            r += escapeHtml(rest);
        } else { r += escapeHtml(p[i]); }
    }
    while (o > 0) { r += '</span>'; o--; }
    return r;
}
function statusBadge(s) {
    s = (s || '').toLowerCase();
    const cls = s === 'running' ? 'badge-green' :
                s === 'ready' ? 'badge-green' :
                s === 'healthy' ? 'badge-green' :
                s === 'pass' ? 'badge-green' :
                s === 'failed' ? 'badge-red' :
                s === 'error' ? 'badge-red' :
                s === 'fail' ? 'badge-red' :
                s === 'stopped' ? 'badge-yellow' :
                s === 'pending' ? 'badge-yellow' :
                s === 'warning' ? 'badge-yellow' :
                s === 'warn' ? 'badge-yellow' :
                s === 'provisioning' ? 'badge-cyan' :
                s === 'creating' ? 'badge-cyan' :
                s === 'cloning' ? 'badge-cyan' :
                s === 'migrating' ? 'badge-cyan' : 'badge-gray';
    return `<span class="badge ${cls}">${escapeHtml(s)}</span>`;
}
function timeSince(dateStr) {
    if (!dateStr) return '-';
    const s = Math.floor((Date.now() - new Date(dateStr).getTime()) / 1000);
    if (s < 60) return s + 's';
    if (s < 3600) return Math.floor(s/60) + 'm';
    if (s < 86400) return Math.floor(s/3600) + 'h';
    return Math.floor(s/86400) + 'd';
}
"#
}

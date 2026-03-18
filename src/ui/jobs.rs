use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use crate::AppState;
use super::layout;

pub async fn page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mkube_url = &state.mkube_url;
    let body = r#"<div class="page-title">Jobs</div>
<div class="tabs">
  <div class="tab active" onclick="switchTab('jobs')">Jobs</div>
  <div class="tab" onclick="switchTab('runners')">Job Runners</div>
  <div class="tab" onclick="switchTab('queue')">Queue</div>
</div>
<div id="tab-jobs" class="tab-panel active"><div class="loading">Loading...</div></div>
<div id="tab-runners" class="tab-panel"><div class="loading">Loading...</div></div>
<div id="tab-queue" class="tab-panel"><div class="loading">Loading...</div></div>
<div class="card" style="margin-top:16px">
  <div class="card-header">Job Logs</div>
  <div class="filter-bar">
    <select id="job-select" onchange="loadJobLogs()"><option value="">Select a job...</option></select>
  </div>
  <div class="term-output" id="job-logs" style="max-height:400px"><div class="empty">Select a job to view logs</div></div>
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
let allJobs = [];
async function loadJobs() {{
    // Jobs - try all-namespaces first
    const jobs = await fetch(MKUBE + '/api/v1/namespaces/default/jobs').then(r => r.json()).catch(() => null);
    allJobs = (jobs && jobs.items) || [];
    let jh = '<table><tr><th>Name</th><th>Phase</th><th>Pool</th><th>Priority</th><th>BMH</th><th>Age</th><th>Actions</th></tr>';
    const sel = document.getElementById('job-select');
    sel.innerHTML = '<option value="">Select a job...</option>';
    for (const j of allJobs) {{
        const name = j.metadata?.name || '-';
        const ns = j.metadata?.namespace || 'default';
        const spec = j.spec || {{}};
        const status = j.status || {{}};
        const phase = status.phase || '-';
        const pool = spec.pool || '-';
        const priority = spec.priority ?? '-';
        const bmh = status.assignedBMH || spec.bmh || '-';
        const age = timeSince(j.metadata?.creationTimestamp);
        jh += `<tr>
            <td>${{escapeHtml(name)}}</td>
            <td>${{statusBadge(phase)}}</td>
            <td>${{escapeHtml(pool)}}</td>
            <td>${{priority}}</td>
            <td>${{escapeHtml(bmh)}}</td>
            <td>${{age}}</td>
            <td><button class="btn btn-sm btn-danger" onclick="cancelJob('${{ns}}','${{name}}')">Cancel</button></td>
        </tr>`;
        sel.innerHTML += `<option value="${{ns}}/${{name}}">${{name}}</option>`;
    }}
    jh += '</table>';
    document.getElementById('tab-jobs').innerHTML = allJobs.length ? jh : '<div class="empty">No jobs</div>';
    // Runners
    const runners = await fetch(MKUBE + '/api/v1/jobrunners').then(r => r.json()).catch(() => null);
    const runnerItems = (runners && runners.items) || [];
    let rh = '<table><tr><th>Name</th><th>Pool</th><th>Status</th></tr>';
    for (const r of runnerItems) {{
        const name = r.metadata?.name || '-';
        const pool = r.spec?.pool || '-';
        const st = r.status?.state || '-';
        rh += `<tr><td>${{escapeHtml(name)}}</td><td>${{escapeHtml(pool)}}</td><td>${{statusBadge(st)}}</td></tr>`;
    }}
    rh += '</table>';
    document.getElementById('tab-runners').innerHTML = runnerItems.length ? rh : '<div class="empty">No job runners</div>';
    // Queue
    const queue = await fetch(MKUBE + '/api/v1/jobqueue').then(r => r.json()).catch(() => null);
    if (queue) {{
        document.getElementById('tab-queue').innerHTML = '<pre style="color:#e0e0e0;font-size:13px;white-space:pre-wrap">' + escapeHtml(JSON.stringify(queue, null, 2)) + '</pre>';
    }} else {{
        document.getElementById('tab-queue').innerHTML = '<div class="empty">Unable to load queue</div>';
    }}
}}
async function cancelJob(ns, name) {{
    if (!confirm('Cancel job ' + name + '?')) return;
    await fetch(MKUBE + `/api/v1/namespaces/${{ns}}/jobs/${{name}}/cancel`, {{method:'POST'}});
    setTimeout(loadJobs, 1000);
}}
async function loadJobLogs() {{
    const val = document.getElementById('job-select').value;
    if (!val) {{ document.getElementById('job-logs').innerHTML = '<div class="empty">Select a job</div>'; return; }}
    const [ns, name] = val.split('/');
    const logs = await fetch(MKUBE + `/api/v1/namespaces/${{ns}}/jobs/${{name}}/logs`).then(r => r.text()).catch(() => '');
    document.getElementById('job-logs').innerHTML = logs ? ansiToHtml(logs) : '<div class="empty">No logs</div>';
}}
loadJobs();
setInterval(loadJobs, 15000);
"#
    );
    Html(layout::page_with_js("Jobs", "Jobs", body, &js))
}

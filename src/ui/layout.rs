use super::style;

/// Wraps page content in the full HTML shell with nav bar (no extra JS).
#[allow(dead_code)]
pub fn page(title: &str, active: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<title>console — {title}</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>{css}</style>
</head>
<body>
{nav}
<div class="content">
{body}
</div>
<script>
{ansi_js}
</script>
</body>
</html>"#,
        title = title,
        css = style::css(),
        nav = nav_html(active),
        body = body,
        ansi_js = style::ansi_js(),
    )
}

/// Wraps page content with extra page-specific JS appended.
pub fn page_with_js(title: &str, active: &str, body: &str, js: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<title>console — {title}</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>{css}</style>
</head>
<body>
{nav}
<div class="content">
{body}
</div>
<script>
{ansi_js}
{js}
</script>
</body>
</html>"#,
        title = title,
        css = style::css(),
        nav = nav_html(active),
        body = body,
        ansi_js = style::ansi_js(),
        js = js,
    )
}

fn nav_html(active: &str) -> String {
    let links = [
        ("Dashboard", "/ui/"),
        ("Nodes", "/ui/nodes"),
        ("Pods", "/ui/pods"),
        ("Deployments", "/ui/deployments"),
        ("Networks", "/ui/networks"),
        ("BMH", "/ui/bmh"),
        ("Storage", "/ui/storage"),
        ("Jobs", "/ui/jobs"),
        ("Logs", "/ui/logs"),
    ];
    let links_html: String = links
        .iter()
        .map(|(label, href)| {
            let cls = if *label == active { " class=\"active\"" } else { "" };
            format!(r#"<a href="{href}"{cls}>{label}</a>"#)
        })
        .collect::<Vec<_>>()
        .join("\n    ");

    format!(
        r#"<nav>
  <span class="brand">console</span>
  <div class="links">
    {links_html}
  </div>
  <span style="margin-left:auto;font-size:12px;color:#888;font-weight:500">mkube cluster</span>
</nav>"#
    )
}

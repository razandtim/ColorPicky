use crate::colors::NamedColor;
use core::fmt::Write;
use heapless::String;

/// Generate the HTML page for the color history
pub fn generate_html_page(
    history: &[Option<NamedColor>; 10],
    current: Option<&NamedColor>,
) -> String<4096> {
    let mut html = String::<4096>::new();

    // HTML Header
    let _ = html.write_str(r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>ColorPicky</title>
<style>
body { font-family: Arial, sans-serif; background: #1a1a2e; color: #eee; padding: 20px; }
h1 { color: #ffd700; text-align: center; }
.color-box { width: 100px; height: 100px; border-radius: 10px; display: inline-block; vertical-align: middle; margin-right: 15px; border: 2px solid #fff; }
.color-item { background: #16213e; padding: 15px; margin: 10px 0; border-radius: 10px; display: flex; align-items: center; }
.color-info { flex: 1; }
.color-name { font-size: 1.2em; font-weight: bold; }
.color-hex { font-family: monospace; font-size: 1.1em; color: #0ff; cursor: pointer; }
.color-hex:hover { text-decoration: underline; }
.copy-btn { background: #0f3460; color: #fff; border: none; padding: 8px 16px; border-radius: 5px; cursor: pointer; margin-left: 10px; }
.copy-btn:hover { background: #e94560; }
.current { border: 3px solid #ffd700; }
.empty { opacity: 0.5; }
</style>
</head>
<body>
<h1>🎨 ColorPicky</h1>
"#);

    // Current color section
    let _ = html.write_str("<h2>Current Color</h2>");
    if let Some(c) = current {
        let r = c.color.r();
        let g = c.color.g();
        let b = c.color.b();
        let _ = write!(
            html,
            r#"<div class="color-item current">
<div class="color-box" style="background:rgb({},{},{})"></div>
<div class="color-info">
<div class="color-name">{}</div>
<div class="color-hex" onclick="copyHex('#{:02X}{:02X}{:02X}')">#{:02X}{:02X}{:02X}</div>
<div>RGB({}, {}, {})</div>
</div>
</div>"#,
            r, g, b, c.name, r, g, b, r, g, b, r, g, b
        );
    } else {
        let _ = html.write_str(r#"<div class="color-item empty">No color detected</div>"#);
    }

    // History section
    let _ = html.write_str("<h2>History</h2>");
    for (i, item) in history.iter().enumerate() {
        if let Some(c) = item {
            let r = c.color.r();
            let g = c.color.g();
            let b = c.color.b();
            let _ = write!(
                html,
                r#"<div class="color-item">
<div class="color-box" style="background:rgb({},{},{})"></div>
<div class="color-info">
<div class="color-name">{}. {}</div>
<div class="color-hex" onclick="copyHex('#{:02X}{:02X}{:02X}')">#{:02X}{:02X}{:02X}</div>
</div>
<button class="copy-btn" onclick="copyHex('#{:02X}{:02X}{:02X}')">Copy</button>
</div>"#,
                r,
                g,
                b,
                i + 1,
                c.name,
                r,
                g,
                b,
                r,
                g,
                b,
                r,
                g,
                b
            );
        }
    }

    // JavaScript for clipboard
    let _ = html.write_str(
        r#"
<script>
function copyHex(hex) {
    navigator.clipboard.writeText(hex).then(() => {
        alert('Copied: ' + hex);
    });
}
</script>
</body>
</html>"#,
    );

    html
}

/// Generate a simple HTTP response
pub fn http_response(body: &str) -> String<4200> {
    let mut response = String::<4200>::new();
    let _ = write!(response, "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
    response
}

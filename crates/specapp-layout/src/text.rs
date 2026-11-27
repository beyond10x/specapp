//! Text widths for layout and escaping for SVG and HTML.

/// Estimated width in pixels of `s` in a sans-serif face of `size` pixels.
pub fn width(s: &str, size: f64, bold: bool) -> f64 {
    let em: f64 = s
        .chars()
        .map(|c| match c {
            'i' | 'j' | 'l' | '.' | ',' | '\'' | ':' | ';' | '|' | '!' | ' ' | 'I' => 0.30,
            'f' | 't' | 'r' | '(' | ')' | '[' | ']' | '-' => 0.40,
            'm' | 'w' | 'M' | 'W' | '…' | '—' => 0.88,
            '_' => 0.55,
            'A'..='Z' => 0.70,
            '0'..='9' => 0.60,
            _ => 0.58,
        })
        .sum();
    em * size * if bold { 1.07 } else { 1.0 }
}

/// Text for an HTML or SVG text node or a quoted attribute.
pub fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// A coordinate as SVG text: at most one decimal, no trailing zero.
pub fn px(n: f64) -> String {
    let r = (n * 10.0).round() / 10.0;
    if r.fract() == 0.0 {
        format!("{}", r as i64)
    } else {
        format!("{r:.1}")
    }
}

/// A JSON value as short text for a box or a table.
pub fn value(v: &serde_json::Value) -> String {
    use serde_json::Value;
    match v {
        Value::Null => String::from("null"),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(items) => format!(
            "[{}]",
            items.iter().map(value).collect::<Vec<_>>().join(", ")
        ),
        Value::Object(_) => v.to_string(),
    }
}

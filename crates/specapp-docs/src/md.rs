//! MDX text: escaping, front matter values, type references and conditions of the compiled ESS
//! model in plain words. Lifted from agenticspec's generator; nothing here names a pack.

use serde_json::Value;

/// The last segment of a qualified name.
pub fn short(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

/// Text from a pack as MDX prose: braces, angle brackets and Markdown marks escaped; code spans
/// in it (balanced backticks) stay code.
pub fn text(s: &str) -> String {
    let s = s.replace(['\n', '\r'], " ");
    if !s.matches('`').count().is_multiple_of(2) || !s.contains('`') {
        return plain(&s);
    }
    s.split('`')
        .enumerate()
        .map(|(i, part)| {
            if i % 2 == 0 {
                plain(part)
            } else {
                format!("`{}`", part.replace('|', "\\|"))
            }
        })
        .collect()
}

fn plain(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '*' => out.push_str("\\*"),
            '_' => out.push_str("\\_"),
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            '|' => out.push_str("\\|"),
            '\\' => out.push_str("\\\\"),
            '`' => out.push_str("\\`"),
            '#' => out.push_str("\\#"),
            '\n' | '\r' => out.push(' '),
            _ => out.push(c),
        }
    }
    out
}

/// A code span that is safe inside a table cell.
pub fn code(s: &str) -> String {
    let inner = s.replace('|', "\\|").replace(['\n', '\r'], " ");
    if inner.contains('`') {
        format!("`` {inner} ``")
    } else {
        format!("`{inner}`")
    }
}

/// A front-matter value: plain when it is a simple word, a JSON string otherwise.
pub fn yaml(s: &str) -> String {
    if !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        && !s.chars().next().is_some_and(|c| c.is_ascii_digit())
        && !matches!(s, "true" | "false" | "null" | "yes" | "no" | "on" | "off")
    {
        s.to_string()
    } else {
        serde_json::to_string(s).unwrap_or_default()
    }
}

/// The first sentence of a summary, or the whole when it has one.
pub fn first_sentence(s: &str) -> String {
    let s = s.trim();
    let mut end = s.len();
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    for (i, (at, c)) in chars.iter().enumerate() {
        if *c == '.' {
            let next = chars.get(i + 1).map(|(_, c)| *c);
            if next.is_none() || next == Some(' ') {
                end = at + 1;
                break;
            }
        }
    }
    s[..end].to_string()
}

/// `n word` or `n words`.
pub fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// A type reference in words: `list of AnswerOption`, `optional PartyId`, `timestamp`.
pub fn type_ref(t: &Value) -> String {
    match t["kind"].as_str() {
        Some("primitive") => t["name"].as_str().unwrap_or("?").to_string(),
        Some("declared") => short(t["name"].as_str().unwrap_or("?")).to_string(),
        Some("optional") => format!("optional {}", type_ref(&t["of"])),
        Some("list") => format!("list of {}", type_ref(&t["of"])),
        Some("map") => format!("map of {} to {}", type_ref(&t["key"]), type_ref(&t["of"])),
        Some(other) => other.to_string(),
        None => "?".into(),
    }
}

/// A predicate of the specification, compactly: strings as written, `not`, `all`, `any` and
/// `exists` spelled out.
pub fn predicate(p: &Value) -> String {
    match p {
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Object(o) => {
            if let Some(x) = o.get("not") {
                return format!("not ({})", predicate(x));
            }
            if let Some(Value::Array(xs)) = o.get("all") {
                return xs.iter().map(predicate).collect::<Vec<_>>().join(" and ");
            }
            if let Some(Value::Array(xs)) = o.get("any") {
                return format!(
                    "({})",
                    xs.iter().map(predicate).collect::<Vec<_>>().join(" or ")
                );
            }
            if let Some(e) = o.get("exists") {
                return format!(
                    "some {} in {} with {}",
                    e["as"].as_str().unwrap_or("x"),
                    collection(&e["in"]),
                    predicate(&e["that"])
                );
            }
            if let Some(e) = o.get("forall") {
                return format!(
                    "every {} in {} has {}",
                    e["as"].as_str().unwrap_or("x"),
                    collection(&e["in"]),
                    predicate(&e["that"])
                );
            }
            if let Some(e) = o.get("distinct") {
                return format!(
                    "each {} in {} has its own {}",
                    e["as"].as_str().unwrap_or("x"),
                    collection(&e["in"]),
                    e["by"].as_str().unwrap_or("?")
                );
            }
            if let Some(c) = o.get("compare") {
                return format!(
                    "{} {} {}",
                    operand(&c["left"]),
                    op(c["op"].as_str().unwrap_or("?")),
                    operand(&c["right"])
                );
            }
            if let Some(h) = o.get("holds") {
                return predicate(&h["predicate"]);
            }
            // `{ field: { op: operand } }`
            if o.len() == 1
                && let Some((field, test)) = o.iter().next()
                && let Some(t) = test.as_object()
                && t.len() == 1
                && let Some((o2, rhs)) = t.iter().next()
                && OPS.iter().any(|(k, _)| k == o2)
            {
                return format!("{field} {} {}", op(o2), operand(rhs));
            }
            serde_json::to_string(p).unwrap_or_default()
        }
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

const OPS: [(&str, &str); 6] = [
    ("eq", "=="),
    ("ne", "!="),
    ("gt", ">"),
    ("gte", ">="),
    ("lt", "<"),
    ("lte", "<="),
];

fn op(o: &str) -> &str {
    OPS.iter().find(|(k, _)| *k == o).map_or(o, |(_, s)| s)
}

fn collection(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Object(o) => match o.get("entity").and_then(Value::as_str) {
            Some(e) => format!("each {}", short(e)),
            None => serde_json::to_string(v).unwrap_or_default(),
        },
        _ => "?".into(),
    }
}

fn operand(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Object(o) => {
            if let Some(off) = o.get("offset") {
                let add = off["add"].as_f64().unwrap_or(0.0);
                let fact = off["fact"].as_str().unwrap_or("?");
                return if add < 0.0 {
                    format!("{fact} - {}", trim_num(-add))
                } else {
                    format!("{fact} + {}", trim_num(add))
                };
            }
            if let Some(f) = o.get("fact").and_then(Value::as_str) {
                return f.to_string();
            }
            serde_json::to_string(v).unwrap_or_default()
        }
        Value::Number(n) => n.as_f64().map_or(n.to_string(), trim_num),
        other => other.to_string(),
    }
}

fn trim_num(x: f64) -> String {
    if x.fract() == 0.0 {
        format!("{}", x as i64)
    } else {
        x.to_string()
    }
}

fn states(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(xs) => xs
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(" or "),
        _ => "?".into(),
    }
}

/// When an outcome applies, in words with code spans, for a table cell. `subject` is the entity
/// the command acts on, if any.
pub fn condition(c: &Value, subject: Option<&str>) -> String {
    let thing = subject.map(short).unwrap_or("subject");
    match c["kind"].as_str() {
        Some("otherwise") => "otherwise".into(),
        Some("unknown_instance") => format!("no such {thing}"),
        Some("existing_instance") => format!("the {thing} exists already"),
        Some("wrong_state") => format!("the {thing} is in a state that does not allow it"),
        Some("subject_state") => {
            let mut s = format!("the {thing} is {}", states(&c["state"]));
            if !c["predicate"].is_null() {
                s.push_str(&format!(" and {}", code(&predicate(&c["predicate"]))));
            }
            s
        }
        Some("subject_predicate") | Some("when") => {
            let mut s = code(&predicate(&c["predicate"]));
            if let Some(i) = c.get("input").filter(|i| !i.is_null()) {
                s.push_str(&format!(" and {}", code(&predicate(i))));
            }
            s
        }
        Some("related") => {
            let entity = short(c["entity"].as_str().unwrap_or("?"));
            let field = c["via"]["field"].as_str().unwrap_or("?");
            match &c["test"] {
                Value::String(t) if t == "absent" => {
                    format!("no {entity} for {}", code(field))
                }
                Value::String(t) => format!("the {entity} of {} is {t}", code(field)),
                t => format!(
                    "the {entity} of {} has {}",
                    code(field),
                    code(&predicate(t))
                ),
            }
        }
        Some("related_set") => {
            let entity = short(c["selection"]["entity"].as_str().unwrap_or("?"));
            let wh = predicate(&c["selection"]["where"]);
            let exists = c["test"]["exists"].as_bool().unwrap_or(true);
            format!(
                "{} {entity} with {}",
                if exists { "a" } else { "no" },
                code(&wh)
            )
        }
        Some("external") => text(c["cause"].as_str().unwrap_or("an outside cause")),
        Some("external_when") => format!(
            "{} ({})",
            text(c["cause"].as_str().unwrap_or("an outside cause")),
            code(&predicate(&c["predicate"]))
        ),
        Some(other) => other.to_string(),
        None => String::new(),
    }
}

/// A JSON value as short text for a table cell: a string as it is, a list joined, a mapping as
/// `key value` pairs.
pub fn cell(v: &Value) -> String {
    match v {
        Value::String(s) => text(s),
        Value::Array(xs) => xs.iter().map(cell).collect::<Vec<_>>().join(", "),
        Value::Object(o) => o
            .iter()
            .map(|(k, v)| format!("{} {}", text(k), cell(v)))
            .collect::<Vec<_>>()
            .join(", "),
        Value::Null => "–".into(),
        other => other.to_string(),
    }
}

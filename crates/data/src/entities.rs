//! A map's entity text: `{ "key" "value" … }` blocks, read as `ED_ParseEdict` and `COM_Parse` do
//! (quoted strings, `//` comments, bare words).

pub type Entity = Vec<(String, String)>;

fn token(s: &[u8], at: &mut usize) -> Option<String> {
    loop {
        while *at < s.len() && s[*at] <= b' ' {
            *at += 1;
        }
        if *at >= s.len() {
            return None;
        }
        if s[*at] == b'/' && s.get(*at + 1) == Some(&b'/') {
            while *at < s.len() && s[*at] != b'\n' {
                *at += 1;
            }
            continue;
        }
        break;
    }
    if s[*at] == b'"' {
        *at += 1;
        let start = *at;
        while *at < s.len() && s[*at] != b'"' {
            *at += 1;
        }
        let t = String::from_utf8_lossy(&s[start..*at]).into_owned();
        *at += 1;
        return Some(t);
    }
    if matches!(s[*at], b'{' | b'}' | b'(' | b')' | b'\'' | b':') {
        *at += 1;
        return Some((s[*at - 1] as char).to_string());
    }
    let start = *at;
    while *at < s.len() && s[*at] > b' ' && !matches!(s[*at], b'{' | b'}' | b'(' | b')' | b'\'' | b':') {
        *at += 1;
    }
    Some(String::from_utf8_lossy(&s[start..*at]).into_owned())
}

pub fn parse(text: &str) -> Vec<Entity> {
    let s = text.as_bytes();
    let mut at = 0;
    let mut out = Vec::new();
    while let Some(t) = token(s, &mut at) {
        if t != "{" {
            break;
        }
        let mut e = Entity::new();
        while let Some(k) = token(s, &mut at) {
            if k == "}" {
                break;
            }
            let Some(v) = token(s, &mut at) else { break };
            e.push((k, v));
        }
        out.push(e);
    }
    out
}

pub fn get<'a>(e: &'a Entity, key: &str) -> Option<&'a str> {
    e.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

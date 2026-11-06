use serde::Serialize;

// ---------- JSON ----------

#[derive(Debug, Clone, Serialize)]
pub struct JsonResult {
    pub valid: bool,
    pub formatted: String,
    pub error: Option<String>,
}

pub fn json_format(text: &str, indent: usize) -> JsonResult {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(v) => {
            let fmt = serde_json::to_string_pretty(&v).unwrap_or_default();
            let formatted = if indent != 2 {
                let pad = " ".repeat(indent.clamp(1, 8));
                fmt.replace("\n  ", &format!("\n{pad}"))
            } else {
                fmt
            };
            JsonResult {
                valid: true,
                formatted,
                error: None,
            }
        }
        Err(e) => JsonResult {
            valid: false,
            formatted: String::new(),
            error: Some(e.to_string()),
        },
    }
}

// ---------- regex ----------

#[derive(Debug, Clone, Serialize)]
pub struct RegexMatch {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegexResult {
    pub valid: bool,
    pub error: Option<String>,
    pub matches: Vec<RegexMatch>,
    pub match_count: usize,
}

pub fn regex_test(pattern: &str, text: &str) -> RegexResult {
    let re = match regex::Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => {
            return RegexResult {
                valid: false,
                error: Some(e.to_string()),
                matches: Vec::new(),
                match_count: 0,
            }
        }
    };
    let mut matches = Vec::new();
    for m in re.find_iter(text).take(200) {
        let groups: Vec<String> = re
            .captures(m.as_str())
            .map(|c| {
                c.iter()
                    .skip(1)
                    .filter_map(|g| g.map(|gg| gg.as_str().to_string()))
                    .collect()
            })
            .unwrap_or_default();
        matches.push(RegexMatch {
            text: m.as_str().to_string(),
            start: m.start(),
            end: m.end(),
            groups,
        });
    }
    RegexResult {
        valid: true,
        error: None,
        match_count: matches.len(),
        matches,
    }
}

// ---------- line diff (simple LCS) ----------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DiffOp {
    Same,
    Add,
    Del,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffLine {
    pub op: DiffOp,
    pub text: String,
    pub line_a: Option<usize>,
    pub line_b: Option<usize>,
}

pub fn diff_lines(a: &str, b: &str) -> Vec<DiffLine> {
    let old: Vec<&str> = a.lines().collect();
    let new: Vec<&str> = b.lines().collect();
    // LCS table — fine for tool-sized inputs (cap at 4000 lines each side)
    let (n, m) = (old.len().min(4000), new.len().min(4000));
    let mut dp = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i][j] = if old[i] == new[j] {
                dp[i + 1][j + 1] + 1
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            };
        }
    }
    let mut out = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < n && j < m {
        if old[i] == new[j] {
            out.push(DiffLine {
                op: DiffOp::Same,
                text: old[i].to_string(),
                line_a: Some(i + 1),
                line_b: Some(j + 1),
            });
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            out.push(DiffLine {
                op: DiffOp::Del,
                text: old[i].to_string(),
                line_a: Some(i + 1),
                line_b: None,
            });
            i += 1;
        } else {
            out.push(DiffLine {
                op: DiffOp::Add,
                text: new[j].to_string(),
                line_a: None,
                line_b: Some(j + 1),
            });
            j += 1;
        }
    }
    while i < n {
        out.push(DiffLine {
            op: DiffOp::Del,
            text: old[i].to_string(),
            line_a: Some(i + 1),
            line_b: None,
        });
        i += 1;
    }
    while j < m {
        out.push(DiffLine {
            op: DiffOp::Add,
            text: new[j].to_string(),
            line_a: None,
            line_b: Some(j + 1),
        });
        j += 1;
    }
    out
}

// ---------- case & whitespace ----------

pub fn case_convert(text: &str, style: &str) -> Result<String, String> {
    let words: Vec<String> = text
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect();
    match style {
        "camel" => Ok(words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                if i == 0 {
                    w.clone()
                } else {
                    let mut c = w.chars();
                    match c.next() {
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        None => String::new(),
                    }
                }
            })
            .collect()),
        "pascal" => Ok(words
            .iter()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect()),
        "snake" => Ok(words.join("_")),
        "kebab" => Ok(words.join("-")),
        "upper" => Ok(text.to_uppercase()),
        "lower" => Ok(text.to_lowercase()),
        "title" => Ok(words
            .iter()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")),
        _ => Err(format!(
            "unknown style '{style}' — camel, pascal, snake, kebab, upper, lower, title"
        )),
    }
}

pub fn clean_whitespace(text: &str, collapse: bool, trim_lines: bool) -> String {
    text.lines()
        .map(|l| {
            let l = if trim_lines { l.trim() } else { l };
            if collapse {
                let mut out = String::with_capacity(l.len());
                let mut last_space = false;
                for c in l.chars() {
                    if c.is_whitespace() {
                        if !last_space {
                            out.push(' ');
                        }
                        last_space = true;
                    } else {
                        out.push(c);
                        last_space = false;
                    }
                }
                out
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_validate_and_format() {
        let r = json_format(r#"{"a":1,"b":[2,3]}"#, 2);
        assert!(r.valid);
        assert!(r.formatted.contains("\"a\": 1"));
        assert!(r.formatted.contains("[\n    2,"));
        let r = json_format("{bad json", 2);
        assert!(!r.valid);
        assert!(r.error.unwrap().len() > 5);
    }

    #[test]
    fn regex_find_and_groups() {
        let r = regex_test(r"(\d+)\.(\d+)\.(\d+)", "v1.2.3 and v10.20.30");
        assert!(r.valid);
        assert_eq!(r.match_count, 2);
        assert_eq!(r.matches[0].groups, vec!["1", "2", "3"]);
        assert_eq!(r.matches[1].text, "10.20.30");
        let r = regex_test("(unclosed", "x");
        assert!(!r.valid);
        assert!(r.error.is_some());
    }

    #[test]
    fn diff_ops() {
        let d = diff_lines("a\nb\nc", "a\nx\nc");
        let ops: Vec<&str> = d.iter().map(|l| match l.op {
            DiffOp::Same => "=",
            DiffOp::Add => "+",
            DiffOp::Del => "-",
        }).collect();
        assert_eq!(ops, vec!["=", "-", "+", "="]);
        assert!(matches!(d[2].op, DiffOp::Add));
        assert_eq!(d[2].text, "x");
        assert!(diff_lines("same", "same").iter().all(|l| matches!(l.op, DiffOp::Same)));
    }

    #[test]
    fn case_styles() {
        assert_eq!(case_convert("hello world-test", "camel").unwrap(), "helloWorldTest");
        assert_eq!(case_convert("hello world-test", "pascal").unwrap(), "HelloWorldTest");
        assert_eq!(case_convert("Hello World", "snake").unwrap(), "hello_world");
        assert_eq!(case_convert("Hello World", "kebab").unwrap(), "hello-world");
        assert_eq!(case_convert("hello world", "title").unwrap(), "Hello World");
        assert!(case_convert("x", "nope").is_err());
    }

    #[test]
    fn whitespace_cleaning() {
        assert_eq!(clean_whitespace("  a   b  \n\tc  ", true, true), "a b\nc");
        assert_eq!(clean_whitespace("a  b", false, true), "a  b");
    }
}

use crate::rule::{Fix, LintError, LintResult, LintWarning, Rule};
use regex::Regex;

#[derive(Debug, Default)]
pub struct MD011ReversedLink;

impl Rule for MD011ReversedLink {
    fn name(&self) -> &'static str {
        "MD011"
    }

    fn description(&self) -> &'static str {
        "Reversed link syntax should be fixed"
    }

    fn check(&self, content: &str) -> LintResult {
        let mut warnings = Vec::new();
        let re = Regex::new(r"\(([^)]+)\)\[([^\]]+)\]").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for cap in re.captures_iter(line) {
                let text = &cap[1];
                let url = &cap[2];
                let start = cap.get(0).unwrap().start();
                warnings.push(LintWarning {
                    line: line_num + 1,
                    column: start + 1,
                    message: "Reversed link syntax found".to_string(),
                    fix: Some(Fix {
                        line: line_num + 1,
                        column: start + 1,
                        replacement: format!("[{}]({})", text, url),
                    }),
                });
            }
        }

        Ok(warnings)
    }

    fn fix(&self, content: &str) -> Result<String, LintError> {
        let mut result = String::new();
        let re = Regex::new(r"\(([^)]+)\)\[([^\]]+)\]").unwrap();

        for line in content.lines() {
            let mut fixed_line = line.to_string();
            for cap in re.captures_iter(line).collect::<Vec<_>>().iter().rev() {
                let text = &cap[1];
                let url = &cap[2];
                let start = cap.get(0).unwrap().start();
                let end = cap.get(0).unwrap().end();
                fixed_line.replace_range(start..end, &format!("[{}]({})", text, url));
            }
            result.push_str(&fixed_line);
            result.push('\n');
        }

        if !content.ends_with('\n') {
            result.pop();
        }

        Ok(result)
    }
} 
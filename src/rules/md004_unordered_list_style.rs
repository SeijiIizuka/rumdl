use crate::rule::{Fix, LintError, LintResult, LintWarning, Rule, Severity};
use crate::utils::range_utils::LineIndex;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CODE_BLOCK_MARKER: Regex = Regex::new(r"^(```|~~~)").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnorderedListStyle {
    Asterisk,   // "*"
    Plus,       // "+"
    Dash,       // "-"
    Consistent, // Use the first marker in a file consistently
}

impl Default for UnorderedListStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

#[derive(Debug, Default)]
pub struct MD004UnorderedListStyle {
    pub style: UnorderedListStyle,
    pub after_marker: usize,
}

impl MD004UnorderedListStyle {
    pub fn new(style: UnorderedListStyle) -> Self {
        Self {
            style,
            after_marker: 1,
        }
    }

    fn get_marker_char(style: UnorderedListStyle) -> char {
        match style {
            UnorderedListStyle::Asterisk => '*',
            UnorderedListStyle::Plus => '+',
            UnorderedListStyle::Dash => '-',
            UnorderedListStyle::Consistent => '*', // Default, but will be overridden
        }
    }

    fn parse_list_marker(line: &str) -> Option<(usize, char)> {
        let indentation = line.len() - line.trim_start().len();
        let trimmed = line.trim_start();

        if let Some(c) = trimmed.chars().next() {
            if (c == '*' || c == '-' || c == '+')
                && trimmed.len() > 1
                && trimmed
                    .chars()
                    .nth(1)
                    .map_or(false, |next| next.is_whitespace())
            {
                return Some((indentation, c));
            }
        }

        None
    }

    fn is_in_code_block(code_blocks: &[bool], line_num: usize) -> bool {
        if line_num < code_blocks.len() {
            code_blocks[line_num]
        } else {
            false
        }
    }

    fn precompute_code_blocks(content: &str) -> Vec<bool> {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_code_block = false;
        let mut code_blocks = vec![false; lines.len()];

        for (i, line) in lines.iter().enumerate() {
            if CODE_BLOCK_MARKER.is_match(line.trim_start()) {
                in_code_block = !in_code_block;
            }
            code_blocks[i] = in_code_block;
        }

        code_blocks
    }
}

impl Rule for MD004UnorderedListStyle {
    fn name(&self) -> &'static str {
        "MD004"
    }

    fn description(&self) -> &'static str {
        "Use consistent style for unordered list markers"
    }

    fn check(&self, content: &str) -> LintResult {
        let line_index = LineIndex::new(content.to_string());
        let mut warnings = Vec::new();

        // Precompute code blocks
        let code_blocks = Self::precompute_code_blocks(content);

        // Track the first marker style for the "consistent" option
        let mut first_marker: Option<char> = None;

        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            // Skip lines in code blocks
            if Self::is_in_code_block(&code_blocks, line_num) {
                continue;
            }

            if let Some((indent, marker)) = Self::parse_list_marker(line) {
                // For consistent style, use the first marker encountered
                let target_style = match self.style {
                    UnorderedListStyle::Consistent => {
                        if first_marker.is_none() {
                            first_marker = Some(marker);
                        }
                        first_marker.unwrap()
                    }
                    specific_style => Self::get_marker_char(specific_style),
                };

                if marker != target_style {
                    warnings.push(LintWarning {
                        line: line_num + 1,
                        column: indent + 1,
                        severity: Severity::Warning,
                        message: format!(
                            "Unordered list item marker '{}' does not match style '{}'",
                            marker, target_style
                        ),
                        fix: Some(Fix {
                            range: line_index.line_col_to_byte_range(line_num + 1, indent + 1),
                            replacement: format!("{}{} ", " ".repeat(indent), target_style),
                        }),
                    });
                }
            }
        }

        Ok(warnings)
    }

    fn fix(&self, content: &str) -> Result<String, LintError> {
        let warnings = self.check(content)?;

        if warnings.is_empty() {
            return Ok(content.to_string());
        }

        let mut result = String::new();
        let lines: Vec<&str> = content.lines().collect();

        // Create a map of line numbers with fixes
        let mut line_fixes: std::collections::HashMap<usize, String> =
            std::collections::HashMap::new();
        for warning in &warnings {
            if let Some(fix) = &warning.fix {
                line_fixes.insert(warning.line, fix.replacement.clone());
            }
        }

        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;
            if let Some(fixed_marker) = line_fixes.get(&line_num) {
                // Replace just the marker, keeping the rest of the line
                if let Some((indent, _)) = Self::parse_list_marker(line) {
                    let rest_of_line = &line[indent + 1..];
                    let first_non_space = rest_of_line.trim_start().len();
                    let spaces = rest_of_line.len() - first_non_space;

                    result.push_str(&format!("{}{}", fixed_marker, &rest_of_line[spaces..]));
                } else {
                    // Shouldn't happen if warnings are accurate
                    result.push_str(line);
                }
            } else {
                result.push_str(line);
            }

            // Add newline for all lines except the last one, unless the original content ends with newline
            if i < lines.len() - 1 {
                result.push('\n');
            }
        }

        // Preserve trailing newline if present or add it
        if content.ends_with('\n') || !content.is_empty() {
            result.push('\n');
        }

        Ok(result)
    }
}

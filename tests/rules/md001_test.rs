use rustmark::rules::MD001HeadingIncrement;
use rustmark::rule::Rule;

#[test]
pub fn test_md001_valid() {
    let rule = MD001HeadingIncrement::default();
    let content = "# Heading 1\n## Heading 2\n### Heading 3\n";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
pub fn test_md001_invalid() {
    let rule = MD001HeadingIncrement::default();
    let content = "# Heading 1\n### Heading 3\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 2);
    assert_eq!(result[0].message, "Heading level should be 2 for this level");
}

#[test]
pub fn test_md001_multiple_violations() {
    let rule = MD001HeadingIncrement::default();
    let content = "# Heading 1\n### Heading 3\n#### Heading 4\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 2);
}

#[test]
pub fn test_md001_fix() {
    let rule = MD001HeadingIncrement::default();
    let content = "# Heading 1\n### Heading 3\n";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "# Heading 1\n## Heading 3\n");
}

#[test]
pub fn test_md001_no_headings() {
    let rule = MD001HeadingIncrement::default();
    let content = "This is a paragraph\nwith no headings.\n";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
pub fn test_md001_single_heading() {
    let rule = MD001HeadingIncrement::default();
    let content = "# Single Heading\n";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
pub fn test_md001_atx_and_setext() {
    let rule = MD001HeadingIncrement::default();
    let content = "# Heading 1\nHeading 2\n---------\n### Heading 3\n";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
} 
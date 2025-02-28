use rustmark::rules::MD003HeadingStyle;
use rustmark::rules::heading_utils::HeadingStyle;
use rustmark::rule::Rule;

#[test]
fn test_consistent_atx() {
    let rule = MD003HeadingStyle::default();
    let content = "# Heading 1\n## Heading 2\n### Heading 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_consistent_atx_closed() {
    let rule = MD003HeadingStyle::new(HeadingStyle::AtxClosed);
    let content = "# Heading 1 #\n## Heading 2 ##\n### Heading 3 ###";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_mixed_styles() {
    let rule = MD003HeadingStyle::default();
    let content = "# Heading 1\n## Heading 2 ##\n### Heading 3";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 2);
}

#[test]
fn test_fix_mixed_styles() {
    let rule = MD003HeadingStyle::default();
    let content = "# Heading 1\n## Heading 2 ##\n### Heading 3";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "# Heading 1\n## Heading 2\n### Heading 3");
}

#[test]
fn test_fix_to_atx_closed() {
    let rule = MD003HeadingStyle::new(HeadingStyle::AtxClosed);
    let content = "# Heading 1\n## Heading 2\n### Heading 3";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "# Heading 1 #\n## Heading 2 ##\n### Heading 3 ###");
}

#[test]
fn test_indented_headings() {
    let rule = MD003HeadingStyle::default();
    let content = "  # Heading 1\n  ## Heading 2\n  ### Heading 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_mixed_indentation() {
    let rule = MD003HeadingStyle::default();
    let content = "# Heading 1\n  ## Heading 2\n    ### Heading 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_preserve_content() {
    let rule = MD003HeadingStyle::default();
    let content = "# Heading with *emphasis* and **bold**\n## Another heading with [link](url)";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, content);
}

#[test]
fn test_empty_headings() {
    let rule = MD003HeadingStyle::default();
    let content = "#\n##\n###";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_heading_with_trailing_space() {
    let rule = MD003HeadingStyle::default();
    let content = "# Heading 1  \n## Heading 2  \n### Heading 3  ";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_consistent_setext() {
    let rule = MD003HeadingStyle::new(HeadingStyle::Setext1);
    let content = "Heading 1\n=========\n\nHeading 2\n---------";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_mixed_setext_atx() {
    let rule = MD003HeadingStyle::new(HeadingStyle::Setext1);
    let content = "Heading 1\n=========\n\n## Heading 2";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 4);
}

#[test]
fn test_fix_to_setext() {
    let rule = MD003HeadingStyle::new(HeadingStyle::Setext1);
    let content = "# Heading 1\n## Heading 2";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "Heading 1\n=========\n\nHeading 2\n---------");
}

#[test]
fn test_setext_with_formatting() {
    let rule = MD003HeadingStyle::new(HeadingStyle::Setext1);
    let content = "Heading with *emphasis*\n====================\n\nHeading with **bold**\n--------------------";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_fix_mixed_setext_atx() {
    let rule = MD003HeadingStyle::new(HeadingStyle::Setext1);
    let content = "Heading 1\n=========\n\n## Heading 2\n### Heading 3";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "Heading 1\n=========\n\nHeading 2\n---------\n\n### Heading 3");
}

#[test]
fn test_setext_with_indentation() {
    let rule = MD003HeadingStyle::new(HeadingStyle::Setext1);
    let content = "  Heading 1\n  =========\n\n  Heading 2\n  ---------";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
} 
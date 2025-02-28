use rustmark::rules::MD035HRStyle;
use rustmark::rule::Rule;

#[test]
fn test_valid_hr_style() {
    let rule = MD035HRStyle::default();
    let content = "Some text\n\n---\n\nMore text";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_invalid_hr_style() {
    let rule = MD035HRStyle::default();
    let content = "Some text\n\n***\n\nMore text";
    let result = rule.check(content).unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_mixed_hr_styles() {
    let rule = MD035HRStyle::default();
    let content = "Some text\n\n---\n\nMiddle text\n\n***\n\nMore text";
    let result = rule.check(content).unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_fix_hr_style() {
    let rule = MD035HRStyle::default();
    let content = "Some text\n\n***\n\nMore text";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "Some text\n\n---\n\nMore text");
} 
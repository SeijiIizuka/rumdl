use rumdl::rules::MD015NoMissingSpaceAfterListMarker;
use rumdl::rule::Rule;

#[test]
fn test_valid_unordered_list() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "* Item 1\n* Item 2\n* Item 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_valid_ordered_list() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "1. First\n2. Second\n3. Third";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_invalid_unordered_list() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "*Item 1\n*Item 2\n*Item 3";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].line, 1);
    assert_eq!(result[0].column, 1);
    assert_eq!(result[0].message, "Missing space after unordered list marker");
}

#[test]
fn test_invalid_ordered_list() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "1.First\n2.Second\n3.Third";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].line, 1);
    assert_eq!(result[0].column, 1);
    assert_eq!(result[0].message, "Missing space after ordered list marker");
}

#[test]
fn test_mixed_list_types() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "*Item 1\n1.First\n-Item 2\n2.Second";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 4);
}

#[test]
fn test_nested_lists() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "* Item 1\n  *Nested 1\n  *Nested 2\n* Item 2";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_code_block() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "```markdown\n*Item 1\n*Item 2\n```\n* Item 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_fix_unordered_list() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "*Item 1\n*Item 2\n*Item 3";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "* Item 1\n* Item 2\n* Item 3");
}

#[test]
fn test_fix_ordered_list() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "1.First\n2.Second\n3.Third";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "1. First\n2. Second\n3. Third");
}

#[test]
fn test_fix_mixed_list_types() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "*Item 1\n1.First\n-Item 2\n2.Second";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "* Item 1\n1. First\n- Item 2\n2. Second");
}

#[test]
fn test_fix_nested_lists() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "* Item 1\n  *Nested 1\n  *Nested 2\n* Item 2";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "* Item 1\n  * Nested 1\n  * Nested 2\n* Item 2");
}

#[test]
fn test_disabled_rule() {
    let rule = MD015NoMissingSpaceAfterListMarker::with_require_space(false);
    let content = "*Item 1\n*Item 2\n*Item 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_list_marker_variations() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "*Item\n-Item\n+Item";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 3);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "* Item\n- Item\n+ Item");
}

#[test]
fn test_preserve_indentation() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "  *Item 1\n    *Item 2\n      *Item 3";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "  * Item 1\n    * Item 2\n      * Item 3");
}

#[test]
fn test_preserve_code_blocks() {
    let rule = MD015NoMissingSpaceAfterListMarker::new();
    let content = "* Item 1\n```\n*Not a list\n```\n* Item 2";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "* Item 1\n```\n*Not a list\n```\n* Item 2");
}

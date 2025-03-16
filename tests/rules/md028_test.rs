use rumdl::rule::Rule;
use rumdl::rules::MD028NoBlanksBlockquote;

#[test]
fn test_md028_valid() {
    let rule = MD028NoBlanksBlockquote::default();
    let content = "> Quote\n> Another line\n\n> New quote\n> Another line\n";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_md028_invalid() {
    let rule = MD028NoBlanksBlockquote::default();
    let content = "> Quote\n> Another line\n>\n> Still same quote\n> Another line\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 3);
}

#[test]
fn test_md028_multiple_blanks() {
    let rule = MD028NoBlanksBlockquote::default();
    let content = "> Quote\n> Another line\n>\n>\n> Still same quote\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].line, 3);
    assert_eq!(result[1].line, 4);
}

#[test]
fn test_md028_fix() {
    let rule = MD028NoBlanksBlockquote::default();
    let content = "> Quote\n> Another line\n>\n> Still same quote\n> Another line\n";
    let result = rule.fix(content).unwrap();
    assert_eq!(
        result,
        "> Quote\n> Another line\n> \n> Still same quote\n> Another line\n"
    );
}

#[test]
fn test_md028_nested_blockquotes() {
    let rule = MD028NoBlanksBlockquote::default();
    let content = "> Outer quote\n>> Nested quote\n>>\n>> Still nested\n> Back to outer\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 3);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(
        fixed,
        "> Outer quote\n>> Nested quote\n>> \n>> Still nested\n> Back to outer\n"
    );
}

#[test]
fn test_md028_indented_blockquotes() {
    let rule = MD028NoBlanksBlockquote::default();
    let content = "  > Indented quote\n  > Another line\n  >\n  > Still same quote\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 3);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(
        fixed,
        "  > Indented quote\n  > Another line\n  > \n  > Still same quote\n"
    );
}

#[test]
fn test_md028_multi_blockquotes() {
    let rule = MD028NoBlanksBlockquote::default();
    let content = "> First quote\n> Another line\n\n> Second quote\n> Another line\n>\n> Still second quote\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 6);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "> First quote\n> Another line\n\n> Second quote\n> Another line\n> \n> Still second quote\n");
}

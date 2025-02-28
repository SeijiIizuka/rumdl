use rustmark::rules::MD011ReversedLink;
use rustmark::rule::Rule;

#[test]
fn test_md011_valid() {
    let rule = MD011ReversedLink::default();
    let content = "[text](link)\n[more text](another/link)\n";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_md011_invalid() {
    let rule = MD011ReversedLink::default();
    let content = "(text)[link]\n(more text)[another/link]\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].line, 1);
    assert_eq!(result[1].line, 2);
}

#[test]
fn test_md011_mixed() {
    let rule = MD011ReversedLink::default();
    let content = "[text](link)\n(reversed)[link]\n[text](link)\n";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 2);
}

#[test]
fn test_md011_fix() {
    let rule = MD011ReversedLink::default();
    let content = "(text)[link]\n(more text)[another/link]\n";
    let result = rule.fix(content).unwrap();
    assert_eq!(result, "[text](link)\n[more text](another/link)\n");
} 
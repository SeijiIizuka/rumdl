use rumdl::rule::Rule;
use rumdl::rules::MD006StartBullets;

#[test]
fn test_unicode_list_items() {
    let rule = MD006StartBullets;
    let content = "\
* Item with Unicode café
* Item with emoji 🔥
  * Nested item with Unicode 汉字
  * Nested item with mixed Unicode こんにちは
* Item with Arabic مرحبا";
    let result = rule.check(content).unwrap();
    assert!(
        result.is_empty(),
        "Valid Unicode list items should not trigger warnings"
    );
}

#[test]
fn test_unicode_indented_list() {
    let rule = MD006StartBullets;
    let content = "\
Some Unicode text here 汉字.

  * First item with Unicode café should not be indented
  * Second item with emoji 🔥 should not be indented
  * Third item with Unicode こんにちは should not be indented";
    let result = rule.check(content).unwrap();
    assert_eq!(
        result.len(),
        3,
        "Indented Unicode list items should trigger warnings"
    );
    let fixed = rule.fix(content).unwrap();
    assert_eq!(
        fixed,
        "\
Some Unicode text here 汉字.

* First item with Unicode café should not be indented
* Second item with emoji 🔥 should not be indented
* Third item with Unicode こんにちは should not be indented"
    );
}

#[test]
fn test_unicode_multiple_lists() {
    let rule = MD006StartBullets;
    let content = "\
* First Unicode list item café
* Second Unicode list item 汉字

Some Unicode text here こんにちは

  * Indented Unicode list 1 🔥
  * Indented Unicode list 2 مرحبا";
    let result = rule.check(content).unwrap();
    assert_eq!(
        result.len(),
        2,
        "Indented Unicode list items should trigger warnings"
    );
    let fixed = rule.fix(content).unwrap();
    assert_eq!(
        fixed,
        "\
* First Unicode list item café
* Second Unicode list item 汉字

Some Unicode text here こんにちは

* Indented Unicode list 1 🔥
* Indented Unicode list 2 مرحبا"
    );
}

#[test]
fn test_unicode_lists_with_blank_lines() {
    let rule = MD006StartBullets;
    let content = "\
* Unicode item 1 café

  * Nested Unicode item 汉字

* Unicode item 2 🔥";
    let result = rule.check(content).unwrap();
    assert!(
        result.is_empty(),
        "Valid Unicode list items with blank lines should not trigger warnings"
    );
}

#[test]
fn test_unicode_code_blocks() {
    let rule = MD006StartBullets;
    let content = "\
```markdown
  * This indented Unicode item café is inside a code block
  * These Unicode items 汉字 should be ignored
  * More Unicode emoji 🔥 in code block
```

* Regular Unicode item こんにちは outside code block";
    let result = rule.check(content).unwrap();
    assert!(
        result.is_empty(),
        "Unicode content in code blocks should be ignored"
    );
}

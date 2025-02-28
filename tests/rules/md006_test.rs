use rustmark::rules::MD006StartBullets;
use rustmark::rule::Rule;

#[test]
fn test_valid_unordered_list() {
    let rule = MD006StartBullets::default();
    let content = "\
* Item 1
* Item 2
  * Nested item
  * Another nested item
* Item 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_invalid_indented_list() {
    let rule = MD006StartBullets::default();
    let content = "\
* Item 1
  * Item 2
    * Nested item
  * Item 3";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 3);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "\
* Item 1
* Item 2
    * Nested item
* Item 3");
}

#[test]
fn test_mixed_list_styles() {
    let rule = MD006StartBullets::default();
    let content = "\
* Item 1
  * Nested item
* Item 2

- Another item
  - Nested item
- Final item";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_multiple_lists() {
    let rule = MD006StartBullets::default();
    let content = "\
* First list item
* Second list item

Some text here

  * Indented list 1
  * Indented list 2";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 2);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "\
* First list item
* Second list item

Some text here

* Indented list 1
* Indented list 2");
}

#[test]
fn test_empty_lines() {
    let rule = MD006StartBullets::default();
    let content = "\
* Item 1

  * Nested item

* Item 2";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_no_lists() {
    let rule = MD006StartBullets::default();
    let content = "\
Just some text
More text
Even more text";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
} 
use rustmark::rules::MD005ListIndent;
use rustmark::rule::Rule;

#[test]
fn test_valid_unordered_list() {
    let rule = MD005ListIndent::default();
    let content = "\
* Item 1
* Item 2
  * Nested 1
  * Nested 2
* Item 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_valid_ordered_list() {
    let rule = MD005ListIndent::default();
    let content = "\
1. Item 1
2. Item 2
  1. Nested 1
  2. Nested 2
3. Item 3";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_invalid_unordered_indent() {
    let rule = MD005ListIndent::default();
    let content = "\
* Item 1
 * Item 2
   * Nested 1";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 3);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "\
* Item 1
* Item 2
  * Nested 1");
}

#[test]
fn test_invalid_ordered_indent() {
    let rule = MD005ListIndent::default();
    let content = "\
1. Item 1
 2. Item 2
    1. Nested 1";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 3);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "\
1. Item 1
2. Item 2
  1. Nested 1");
}

#[test]
fn test_mixed_list_types() {
    let rule = MD005ListIndent::default();
    let content = "\
* Item 1
  1. Nested ordered
  * Nested unordered
* Item 2";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_multiple_levels() {
    let rule = MD005ListIndent::default();
    let content = "\
* Level 1
   * Level 2
      * Level 3";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 2);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "\
* Level 1
  * Level 2
    * Level 3");
}

#[test]
fn test_empty_lines() {
    let rule = MD005ListIndent::default();
    let content = "\
* Item 1

  * Nested 1

* Item 2";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_no_lists() {
    let rule = MD005ListIndent::default();
    let content = "\
Just some text
More text
Even more text";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_complex_nesting() {
    let rule = MD005ListIndent::default();
    let content = "\
* Level 1
  * Level 2
    * Level 3
  * Back to 2
    1. Ordered 3
    2. Still 3
* Back to 1";
    let result = rule.check(content).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_invalid_complex_nesting() {
    let rule = MD005ListIndent::default();
    let content = "\
* Level 1
   * Level 2
     * Level 3
   * Back to 2
      1. Ordered 3
     2. Still 3
* Back to 1";
    let result = rule.check(content).unwrap();
    assert_eq!(result.len(), 5);
    let fixed = rule.fix(content).unwrap();
    assert_eq!(fixed, "\
* Level 1
  * Level 2
    * Level 3
  * Back to 2
    1. Ordered 3
    2. Still 3
* Back to 1");
}

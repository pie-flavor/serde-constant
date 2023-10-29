# `serde-constant`

This crate provides a type that can represent a single-const serde value. For example, asserting that a particular bool is always true.
You can use this for disambiguation in `serde(untagged)` structures, or just for validation.

# Examples

```rust
use serde_constant::ConstBool;
#[derive(Deserialize)]
struct Foo {
    bar: String,
    baz: ConstBool<true>,
}
assert!(serde_json::from_value::<Foo>(json!({ "bar": "quux", "baz": true })).is_ok());
assert!(serde_json::from_value::<Foo>(json!({ "bar": "quux", "baz": false })).is_err());
```

```rust
use serde_constant::ConstI64;
// int tags? No problem!
#[derive(Deserialize)]
#[serde(untagged)]
enum Foo {
    Bar {
        tag: ConstI64<1>,
    },
    Baz {
        tag: ConstI64<2>,
        x: Option<String>,
    },
}
assert!(matches!(
    serde_json::from_value(json!({ "tag": 2, "x": null }))?,
    // would have been Bar if `tag` were just `i64`
    Foo::Baz { x: None, .. },
));
```

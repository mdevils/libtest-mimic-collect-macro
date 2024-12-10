# libtest-mimic-collect

Automatically collect tests marked `#[test]` attribute from different modules.

This library is useful when you want to use `libtest-mimic` library, but want to
keep the convenience of using `#[test]` attribute.

Tests are collected using `libtest-mimic-collect` library and can then be run. 

## Installation

Add these modules to the dependencies:

* `libtest-mimic`
* `libtest-mimic-collect`
* `libtest-mimic-collect-macro`
* `ctor`

## Example

Specify your test target in `Cargo.toml`:

```toml
[[test]]
name = "test"
harness = false
path = "lib/test.rs"
```

Create a test module:

```rust
mod my_mod1;
mod my_mod2;
// ...

#[macro_use]
extern crate libtest_mimic_collect_macro;

#[test]
fn test_success() {
    ()
}

#[test]
fn test_failure() -> Result<(), String> {
    Err("Something went wrong".into())
}

#[test]
fn test_assert() {
    assert_eq!(1, 2);
}

pub fn main() {
    libtest_mimic_collect::TestCollection::run();
}
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](../libtest-mimic-collect/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](../libtest-mimic-collect/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

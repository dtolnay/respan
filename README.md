Respan
======

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/respan-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/respan)
[<img alt="crates.io" src="https://img.shields.io/crates/v/respan.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/respan)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-respan-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/respan)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/respan/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/respan/actions?query=branch%3Amaster)

Macros to erase scope information from tokens.

## Example

```rust
#![forbid(unsafe_code)]

fn main() {
    let ptr = 1337 as *const i32;
    let value = respan::call_site! {
        unsafe { *ptr }
    };
    println!("{}", value);
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

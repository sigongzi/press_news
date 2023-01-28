## run example
```shell
cargo run --example fake-real
```

## summary
create a new crate named \<trait_name\>_derive (relatively canonical)

add

```toml
[lib]
proc-macro = true
```
in `Cargo.toml`

And re-export the `proc_macro_derive` trait in current lib

for example

in lib.rs (this crate)

```rs
pub use press_news_derive::{self, *};
```
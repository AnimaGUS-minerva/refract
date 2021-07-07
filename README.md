# refract

Facilitate exposing some constituent files of a crate under the 'target' directory of its "consumer" crate.

Note: This functionality would go against [the generally recommended practice](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script):

> Build scripts may save any output files in the directory specified in the OUT_DIR environment variable. Scripts should not modify any files outside of that directory.

So, use this **at your own risk in a very specific situation** (in our case, a custom Rust build/test framework) where this sort of spec is absolutely required.

## Examples

### `foo` crate

```
foo/
  Cargo.toml
  build.rs
  src/
    one.rs
    two.rs
```

Add `minerva_refract` in 'foo/Cargo.toml' as

```toml
[build-dependencies]
minerva_refract = { git = "https://github.com/AnimaGUS-minerva/refract.git" }
```

and use it from within 'foo/build.rs'.

```rust
extern crate minerva_refract;
use minerva_refract::expose_under_target;

fn main() {
    println!("cargo:rerun-if-changed=src/one.rs");
    println!("cargo:rerun-if-changed=src/two.rs");

    // Expose 'src/one.rs' as 'target/refract_foo/one.rs'
    let dir_opt = Some("refract_foo");
    expose_under_target("src/one.rs", dir_opt, "one.rs").unwrap();

    // Expose 'src/two.rs' as 'target/two.rs'
    expose_under_target("src/two.rs", None, "two.rs").unwrap();
}
```

### `bar` crate (consumer)

'bar/Cargo.toml'

```
[dependencies]
foo = 0.1
```

Now, upon `cargo test`, we can access the raw files (i.e. 'one.rs' and 'two.rs') exposed by the `foo` crate.

```
bar/
  Cargo.toml
  src/
    lib.rs
  target/
    refract_foo/
      one.rs
    two.rs
```

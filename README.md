winp.rs
===

Read outputs of the process for Windows

## Usage

Cargo.toml

```toml
[dependencies.winp]
git = "https://github.com/bnuts/winp.rs.git"
version = "*"
```

```rust
extern crate winp;

fn main() {
    let output = winp::Winp::new("cat").arg("--version").output().unwrap();
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
}
```

# Hello Rust Depencencies

Follow the previous readme with ``Hello Rust``

``cargo add ferris-says``

updated ``Cargo.toml`` and ``Cargo.lock`` file with ``ferris-says`` depencencies

Replace main.rs with:

```rust
use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    let out = "Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
```

``cargo run```

The output will be:

```text
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```
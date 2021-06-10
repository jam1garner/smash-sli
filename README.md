# smash-sli

A Rust library for working with `soundlabelinfo.sli` files from Smash Ultimate.

### Example Usage

```rust
use sli::SliFile;

let mut file = SliFile::open("soundlabelinfo.sli")?;

for entry in file.entries() {
    println!("tone_id: {:#X}", entry.tone_id);
}

for (i, entry) in file.entries_mut().enumerate() {
    entry.nus3bank_id = 5000;
}

file.save("soundlabelinfo.sli")?;
```

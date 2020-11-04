# smash-bgm-property

A Rust library for working with `bgm_property.bin` files from Smash Ultimate.

### Example Usage

```rust
use bgm_property::BgmPropertyFile;

let mut file = BgmPropertyFile::open("bgm_property.bin")?;

for entry in file.entries() {
    println!("name_id: {:#X}", entry.name_id);
}

for entry in file.entries_mut() {
    entry.loop_start_sample = 0;
}

file.save("bgm_property.bin")?;
```

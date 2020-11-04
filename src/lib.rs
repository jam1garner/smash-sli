//! A Rust library for working with `bgm_property.bin` files from Smash Ultimate. This allows for
//! modifying various properties associated with  background music.
//! 
/// ```rust
/// # fn main() -> binread::BinResult<()> {
/// use bgm_property::BgmPropertyFile;
/// 
/// let mut file = BgmPropertyFile::open("bgm_property.bin")?;
/// 
/// for entry in file.entries() {
///     println!("name_id: {:#X}", entry.name_id);
/// }
/// 
/// for entry in file.entries_mut() {
///     entry.loop_start_sample = 0;
/// }
/// 
/// file.save("bgm_property_out.bin")?;
/// # Ok(())
/// # }
/// ```

use binread::{BinRead, BinReaderExt, derive_binread};
use binwrite::{BinWrite, WriterOption};

use std::fs::File;
use std::path::Path;
use std::io::{self, Write, BufReader, BufWriter};

#[cfg(feature = "derive_serde")]
use serde::{Serialize, Deserialize};

mod hash40;

/// Type alias for Hash40
pub type Hash40 = u64;

pub use binread::{BinResult as Result, Error};

/// ```rust
/// # fn main() -> binread::BinResult<()> {
/// use bgm_property::BgmPropertyFile;
/// 
/// let mut file = BgmPropertyFile::open("bgm_property.bin")?;
/// 
/// for entry in file.entries() {
///     println!("name_id: {:#X}", entry.name_id);
/// }
/// 
/// for entry in file.entries_mut() {
///     entry.loop_start_sample = 0;
/// }
/// 
/// file.save("bgm_property_out.bin")?;
/// # Ok(())
/// # }
/// ```
#[derive_binread]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
#[br(magic = b"PMGB")]
pub struct BgmPropertyFile (
    #[br(temp)]
    u32,

    #[br(count = self_0)]
    Vec<Entry>,
);

impl BinWrite for BgmPropertyFile {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            "PMGB",
            self.0.len() as u32,
            &self.0
        ).write_options(writer, options)
    }
}

/// An entry representing a single nus3audio background music file
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite, Debug)]
pub struct Entry {
    #[serde(with = "serde_hash40")]
    pub name_id: Hash40,
    pub unk: u32,
    pub loop_start_sample: u32,
    pub unk_sample: u32,
    pub loop_end_sample: u32,
    pub unk2: u32,
    
    #[br(pad_after = 4)]
    #[binwrite(pad_after(0x4))]
    pub total_samples: u32,
}

#[cfg(feature = "derive_serde")]
pub fn set_labels<P: AsRef<Path>>(path: P) -> Result<()> {
    fn inner(path: &Path) -> Result<()> {
        let contents = std::fs::read_to_string(path)?;
        let labels = contents.split("\n")
            .map(|string| (hash40::hash40(string.trim()), string.to_owned()))
            .collect();

        *serde_hash40::LABELS.lock().unwrap() = labels;

        Ok(())
    }

    inner(path.as_ref())
}

#[cfg(feature = "derive_serde")]
mod serde_hash40 {
    use std::{
        sync::Mutex,
        collections::HashMap,
    };

    lazy_static::lazy_static! {
        pub static ref LABELS: Mutex<HashMap<Hash40, String>> = Mutex::new(HashMap::new());
    }

    use super::{hash40::hash40, Hash40};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D, E>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        if s.starts_with("0x") {
            u64::from_str_radix(s.trim_start_matches("0x"), 16)
                .map_err(|_| D::Error::custom(format!("{} is an invalid Hash40", s)))
        } else {
            Ok(hash40(&s))
        }
    }

    pub fn serialize<S>(hash40: &Hash40, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer,
    {
        match LABELS.lock().unwrap().get(hash40) {
            Some(label) => {
                serializer.serialize_str(&label)
            }
            None => {
                serializer.serialize_str(&format!("{:#x}", hash40))
            }
        }
    }
}


impl BgmPropertyFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        BufReader::new(File::open(path)?).read_le()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.write(&mut BufWriter::new(File::create(path)?))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.write_options(writer, &binwrite::writer_option_new!(endian: binwrite::Endian::Little))
            .map_err(Into::into)
    }

    pub fn new(entries: Vec<Entry>) -> Self {
        BgmPropertyFile(entries)
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut Vec<Entry> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = std::fs::read("bgm_property.bin").unwrap();
        let bgm_property = BgmPropertyFile::open("bgm_property.bin").unwrap();

        println!("{:#X?}", bgm_property);

        let mut round_trip = Vec::new();
        bgm_property.write(&mut round_trip).unwrap();

        assert_eq!(original, round_trip);
        //bgm_property.save("bgm_property_out.bin").unwrap();
    }
}

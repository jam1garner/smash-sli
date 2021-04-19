//! A Rust library for working with `soundlabelinfo.sli` files from Smash Ultimate. This allows for
//! modifying various properties associated with  background music.

use binread::{BinRead, BinReaderExt, derive_binread};
use binwrite::{BinWrite, WriterOption};

use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Seek, Write, BufReader, BufWriter};

#[cfg(feature = "derive_serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "derive_serde")]
mod hash40;

/// Type alias for Hash40
pub type Hash40 = u64;

pub use binread::{BinResult as Result, Error};

#[derive_binread]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
#[br(magic = b"SLI\0\x01\0\0\0")]
pub struct SliFile (
    #[br(temp)]
    u32,

    #[br(count = self_0)]
    Vec<Entry>,
);

impl BinWrite for SliFile {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        let mut entries = self.0.clone();
        entries.sort_unstable_by(|a, b| a.tone_name.cmp(&b.tone_name));

        (
            "SLI\0\x01\0\0\0",
            self.0.len() as u32,
            entries
        ).write_options(writer, options)
    }
}

/// An entry representing a single nus3audio background music file
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite, Debug, Clone)]
pub struct Entry {
    #[cfg_attr(feature = "derive_serde", serde(with = "serde_hash40"))]
    pub tone_name: Hash40,
    pub nus3bank_id: u32,
    pub tone_id: u32,
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


impl SliFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        reader.read_le()
    }

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
        SliFile(entries)
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
        let original = std::fs::read("soundlabelinfo.sli").unwrap();
        let bgm_property = SliFile::open("soundlabelinfo.sli").unwrap();

        println!("{:#X?}", bgm_property);

        let mut round_trip = Vec::new();
        bgm_property.write(&mut round_trip).unwrap();

        assert_eq!(original, round_trip);
        //bgm_property.save("bgm_property_out.bin").unwrap();
    }
}

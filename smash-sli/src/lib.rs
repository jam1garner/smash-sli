//! # sli_lib
//!
//! sli_lib is a library for reading and writing `soundlabelinfo.sli` files from Super Smash Bros. Ultimate.
use std::{
    fs,
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

use binrw::{binrw, BinReaderExt, BinResult, BinWrite};
pub use hash40::Hash40;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The container type for sound entries.
#[binrw]
#[brw(magic = b"SLI\0", little)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct SliFile {
    #[br(temp)]
    #[bw(calc = 1u32)]
    unk1: u32,

    #[br(temp)]
    #[bw(calc = entries.len() as u32)]
    entry_count: u32,

    #[br(count = entry_count)]
    #[bw(map = |e| {
        let mut entries = e.clone();
        entries.sort_unstable_by(|a, b| a.tone_name.cmp(&b.tone_name));
        entries
    })]
    pub entries: Vec<SliEntry>,
}

impl SliFile {
    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let sli = reader.read_le::<Self>()?;

        Ok(sli)
    }

    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut file = Cursor::new(fs::read(path)?);
        let sli = file.read_le::<Self>()?;

        Ok(sli)
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.write_le(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_le(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }
}

/// A group of sound identification parameters involved with sound lookups.
#[binrw]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct SliEntry {
    /// Hashed name of the sound.
    pub tone_name: Hash40,

    /// ID of the associated NUS3BANK file.
    pub nus3bank_id: u32,

    /// ID of the sound in the NUS3AUDIO, NUS3BANK, and TONELABEL files.
    pub tone_id: u32,
}

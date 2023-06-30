use anyhow::bail;
use byteorder::{LittleEndian, ReadBytesExt};
use lz4_flex::decompress;

use crate::prelude::*;

pub async fn unpack_dvpl(mut dvpl: Vec<u8>) -> Result<Vec<u8>> {
    let footer = Footer::try_from(dvpl.as_slice())?;
    dvpl.truncate(footer.compressed_size);
    match footer.compression_type {
        CompressionType::None => Ok(dvpl),
        CompressionType::Lz4 | CompressionType::Lz4Hc => {
            decompress(&dvpl, footer.uncompressed_size).context("failed to decompress LZ4")
        }
        CompressionType::Rfc1951 => unimplemented!("RFC1951 is not implemented"),
    }
}

struct Footer {
    uncompressed_size: usize,
    compressed_size: usize,
    compression_type: CompressionType,
}

impl TryFrom<&[u8]> for Footer {
    type Error = Error;

    fn try_from(dvpl: &[u8]) -> std::result::Result<Self, Self::Error> {
        let (body, mut footer) = dvpl.split_at(dvpl.len() - 20);
        let uncompressed_size = footer.read_u32::<LittleEndian>()? as usize;
        let compressed_size = footer.read_u32::<LittleEndian>()? as usize;
        ensure!(compressed_size == body.len(), "incorrect compressed size ({compressed_size})");
        let crc32 = footer.read_u32::<LittleEndian>()?;
        ensure!(crc32 == crc32fast::hash(body), "incorrect CRC32");
        let compression_type = CompressionType::try_from(footer.read_u32::<LittleEndian>()?)?;
        let magic = footer.read_u32::<LittleEndian>()?;
        ensure!(magic == MAGIC, "incorrect magic number (expected {MAGIC:x}, got {magic:x})");
        Ok(Self {
            uncompressed_size,
            compressed_size,
            compression_type,
        })
    }
}

enum CompressionType {
    None,
    Lz4,
    Lz4Hc,
    Rfc1951,
}

impl TryFrom<u32> for CompressionType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CompressionType::None),
            1 => Ok(CompressionType::Lz4),
            2 => Ok(CompressionType::Lz4Hc),
            3 => Ok(CompressionType::Rfc1951),
            _ => bail!("incorrect compression type ({value})"),
        }
    }
}

/// Magic number at the end of DVPL file, this is just a low-endian for «DVPL».
const MAGIC: u32 = 0x4C505644;

#[cfg(test)]
mod tests {
    use std::{fs::read, path::Path};

    use super::*;

    #[tokio::test]
    async fn unpack_list_ok() -> Result {
        let dvpl = read(Path::new("src").join("tankopedia").join("tests").join("list.xml.dvpl"))?;
        unpack_dvpl(dvpl).await?;
        Ok(())
    }
}

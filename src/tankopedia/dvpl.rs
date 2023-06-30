use std::{
    io::{Cursor, Read},
    path::Path,
};

use anyhow::bail;
use lz4_flex::decompress;
use tokio::{fs::read, io::AsyncReadExt, task::spawn_blocking};

use crate::prelude::*;

/// Wrapper for `*.dvpl` file contents.
#[derive(derive_more::From)]
pub struct Dvpl(pub Vec<u8>);

impl Dvpl {
    /// Read the DVPL contents into the structure. The DVPL does **not** get parsed at this moment.
    ///
    /// This function exists mainly for developer's happiness.
    #[inline]
    pub async fn read(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(read(path).await?))
    }

    /// Parse the DVPL and return the unpacked contents as a vector.
    pub async fn into_vec(mut self) -> Result<Vec<u8>> {
        let footer = Footer::try_from(self.0.as_slice()).await?;

        // Remove the raw footer.
        self.0.truncate(footer.compressed_size);

        match footer.compression_type {
            CompressionType::None => Ok(self.0),
            CompressionType::Lz4 | CompressionType::Lz4Hc => {
                spawn_blocking(move || decompress(&self.0, footer.uncompressed_size))
                    .await?
                    .context("failed to decompress LZ4")
            }
            CompressionType::Rfc1951 => unimplemented!("RFC1951 is not implemented"),
        }
    }

    /// Parse the DVPL and return the unpacked contents as a reader.
    #[inline]
    pub async fn into_reader(self) -> Result<impl Read> {
        Ok(Cursor::new(self.into_vec().await?))
    }
}

/// DVPL footer.
///
/// Funny, usually people put this kind of information in the beginning, so that one
/// would know how to read the file beforehand.
///
/// These smart guys put it at the end.
struct Footer {
    uncompressed_size: usize,
    compressed_size: usize,
    compression_type: CompressionType,
}

impl Footer {
    pub async fn try_from(dvpl: &[u8]) -> Result<Self> {
        let (body, mut footer) = dvpl.split_at(dvpl.len() - 20);
        let uncompressed_size = footer.read_u32_le().await? as usize;
        let compressed_size = footer.read_u32_le().await? as usize;
        ensure!(compressed_size == body.len(), "incorrect compressed size ({compressed_size})");
        let crc32 = footer.read_u32_le().await?;
        ensure!(crc32 == crc32fast::hash(body), "incorrect CRC32");
        let compression_type = CompressionType::try_from(footer.read_u32_le().await?)?;
        let magic = footer.read_u32_le().await?;
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
            _ => bail!("unexpected compression type ({value})"),
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
        Dvpl(dvpl).into_vec().await?;
        Ok(())
    }
}

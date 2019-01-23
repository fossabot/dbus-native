use std::io;
use byteorder::{WriteBytesExt, ByteOrder};
use crate::type_system::{ObjectPath, Signature};

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait DbusWrite {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>) -> Result<()>
        where T1: io::Write,
              T2: ByteOrder;
}

pub struct DbusWriter<T: io::Write> {
    writer: T,
}

impl<T: io::Write> DbusWriter<T> {
    pub fn new(writer: T) -> DbusWriter<T> {
        DbusWriter {
            writer
        }
    }

    pub fn write_invalid(&self) -> Result<()> {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "HeaderField::Invalid can not be marshaled!"))
    }

    /// A single 8-bit byte.
    pub fn write_u8(&mut self, n: u8) -> Result<()> {
        self.writer.write_u8(n)
    }

    /// As for UINT32, but only 0 and 1 are valid values.
    pub fn write_boolean<T1: ByteOrder>(&mut self, b: bool) -> Result<()> {
        self.writer.write_u32::<T1>(b as u32)
    }

    /// 16-bit signed integer in the message's byte order.
    pub fn write_i16<T1: ByteOrder>(&mut self, i: i16) -> Result<()> {
        self.writer.write_i16::<T1>(i)
    }

    /// 16-bit unsigned integer in the message's byte order.
    pub fn write_u16<T1: ByteOrder>(&mut self, u: u16) -> Result<()> {
        self.writer.write_u16::<T1>(u)
    }

    /// 32-bit signed integer in the message's byte order.
    pub fn write_i32<T1: ByteOrder>(&mut self, i: i32) -> Result<()> {
        self.writer.write_i32::<T1>(i)
    }

    /// 32-bit unsigned integer in the message's byte order.
    pub fn write_u32<T1: ByteOrder>(&mut self, u: u32) -> Result<()> {
        self.writer.write_u32::<T1>(u)
    }

    /// 64-bit signed integer in the message's byte order.
    pub fn write_i64<T1: ByteOrder>(&mut self, i: i64) -> Result<()> {
        self.writer.write_i64::<T1>(i)
    }

    /// 64-bit unsigned integer in the message's byte order.
    pub fn write_u64<T1: ByteOrder>(&mut self, u: u64) -> Result<()> {
        self.writer.write_u64::<T1>(u)
    }

    /// A UINT32 indicating the string's length in bytes excluding its terminating nul,
    /// followed by non-nul string data of the given length, followed by a terminating nul byte.
    pub fn write_string<T1: ByteOrder>(&mut self, s: &str) -> Result<()> {
        self.writer.write_u32::<T1>(s.len() as u32)?;
        self.writer.write_all(s.as_bytes())?;
        self.writer.write_u8(b'\n')?;
        Ok(())
    }

    /// Exactly the same as STRING except the content must be a valid object path (see above).
    pub fn write_object_path<T1: ByteOrder>(&mut self, object_path: ObjectPath) -> Result<()> {
        self.write_string::<T1>(&object_path.0)
    }

    /// The same as STRING except the length is a single byte (thus signatures
    /// have a maximum length of 255) and the content must be a valid signature (see above).
    pub fn write_signature<T1: ByteOrder>(&mut self, signature: Signature) -> Result<()> {
        self.write_string::<T1>(&signature.0)
    }

    /// A UINT32 giving the length of the array data in bytes, followed by alignment
    /// padding to the alignment boundary of the array element type, followed by each array element.
    pub fn write_array<T1: ByteOrder, T2: DbusWrite>(&mut self, a: &[T2]) -> Result<()> {
        self.writer.write_u32::<T1>(a.len() as u32)?;
        for x in a {
            x.write::<_, T1>(self)?;
        }
        Ok(())
    }
}


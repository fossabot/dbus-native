use std::io;
use byteorder::{WriteBytesExt, ByteOrder};

use crate::message::{Signature, ObjectPath};

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait DbusWrite {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1, T2>) -> Result<()>
        where T1: ByteOrder,
              T2 : io::Write;
}


pub struct DbusWriter<T1: ByteOrder, T2: io::Write> {
    writer: T2,
}

// impl<T1, T2> DbusWriteInternal<T1, T2> for DbusWriter<T1, T2> {}

impl<T1: ByteOrder, T2: io::Write> DbusWriter<T1, T2> {
    pub fn new(writer: &mut T2) -> DbusWriter<T1, T2> {
        DbusWriter {
            writer
        }
    }
// }
// pub trait DbusWriteInternal<T1: ByteOrder, T2: io::Write> {

    fn write_invalid(&self) -> Result<()> {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "HeaderField::Invalid can not be marshaled!"))
    }

    /// A single 8-bit byte.
    fn write_u8(&self, writer: &mut T2, n: u8) -> Result<()> {
        writer.write_u8(n)
    }

    /// As for UINT32, but only 0 and 1 are valid values.
    fn write_boolean(&self, writer: &mut T2, b: bool) -> Result<()> {
        writer.write_u32::<T1>(b as u32)
    }

    /// 16-bit signed integer in the message's byte order.
    fn write_i16(&self, writer: &mut T2, i: i16) -> Result<()> {
        writer.write_i16::<T1>(i)
    }

    /// 16-bit unsigned integer in the message's byte order.
    fn write_u16(&self, writer: &mut T2, u: u16) -> Result<()> {
        writer.write_u16::<T1>(u)
    }

    /// 32-bit signed integer in the message's byte order.
    fn write_i32(&self, writer: &mut T2, i: i32) -> Result<()> {
        writer.write_i32::<T1>(i)
    }

    /// 32-bit unsigned integer in the message's byte order.
    fn write_u32(&self, writer: &mut T2, u: u32) -> Result<()> {
        writer.write_u32::<T1>(u)
    }

    /// 64-bit signed integer in the message's byte order.
    fn write_i64(&self, writer: &mut T2, i: i64) -> Result<()> {
        writer.write_i32::<T1>(i)
    }

    /// 64-bit unsigned integer in the message's byte order.
    fn write_u64(&self, writer: &mut T2, u: u64) -> Result<()> {
        writer.write_u64::<T1>(u)
    }

    /// A UINT32 indicating the string's length in bytes excluding its terminating nul,
    /// followed by non-nul string data of the given length, followed by a terminating nul byte.
    fn write_string(&self, writer: &mut T2, s: String) -> Result<()> {
        writer.write_u32::<T1>(s.len() as u32)?;
        writer.write::<T1>(s)?;
        writer.write_u8('\n' as u8)?;
        Ok(())
    }

    /// Exactly the same as STRING except the content must be a valid object path (see above).
    fn write_object_path(&self, writer: &mut T2, object_path: ObjectPath) -> Result<()> {
        self.write_string(writer, object_path.0)
    }

    /// The same as STRING except the length is a single byte (thus signatures
    /// have a maximum length of 255) and the content must be a valid signature (see above).
    fn write_signature(&self, writer: &mut T2, signature: Signature) -> Result<()> {
        self.write_string(writer, signature.0)
    }

    /// A UINT32 giving the length of the array data in bytes, followed by alignment
    /// padding to the alignment boundary of the array element type, followed by each array element.
    fn write_array<T>(&self, writer: &mut T2, a: &[T]) -> Result<()> {
        writer.write_u32::<T1>(a.len())?;
        for x in a {
            a.write(writer, x)?;
        }
        Ok(())
    }
}


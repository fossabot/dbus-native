use std::io;
use byteorder::ByteOrder;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait DbusWriter<T1: ByteOrder, T2: io::Write> {

    fn write(&self, writer: &mut T2) -> Result<()>;

    fn write_invalid(&self) -> Result<()> {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "HeaderField::Invalid can not be marshaled!"))
    }

    /// A single 8-bit byte.
    fn write_byte((&self, writer: &mut T2, n: u8) -> Result<()> {
        writer.write_u8(n)
    }

    /// As for UINT32, but only 0 and 1 are valid values.
    fn write_boolean((&self, writer: &mut T2, b: bool) -> Result<()> {
        writer.write_u32::<T1>(b as u32)
    }

    /// 16-bit signed integer in the message's byte order.
    fn write_i16((&self, writer: &mut T2, i: i16) -> Result<()> {
        writer.write_i16::<T1>(i)
    }

    /// 16-bit unsigned integer in the message's byte order.
    fn write_u16((&self, writer: &mut T2, u: u16) -> Result<()> {
        writer.write_u16::<T1>(u)
    }

    /// 32-bit signed integer in the message's byte order.
    fn write_i32((&self, writer: &mut T2, i: i32) -> Result<()> {
        writer.write_i32::<T1>(i)
    }

    /// 32-bit unsigned integer in the message's byte order.
    fn write_u32((&self, writer: &mut T2, u: u32) -> Result<()> {
        writer.write_u32::<T1>(u)
    }

    /// 64-bit signed integer in the message's byte order.
    fn write_i64((&self, writer: &mut T2, i: i64) -> Result<()> {
        writer.write_i32::<T1>(i)
    }

    /// 64-bit unsigned integer in the message's byte order.
    fn write_u64((&self, writer: &mut T2, u: u64) -> Result<()> {
        writer.write_u64::<T1>(u)
    }

    /// A UINT32 indicating the string's length in bytes excluding its terminating nul,
    /// followed by non-nul string data of the given length, followed by a terminating nul byte.
    fn write_string(&self, writer: &mut T2, s: String) -> Result<()> {
        writer.write_u32::<T1>(s.len())?;
        writer.write::<T1>(s)?;
        writer.write_u8('\n')?;
        Ok(())
    }

    /// Exactly the same as STRING except the content must be a valid object path (see above).
    fn write_object_path(&self, writer: &mut T2, object_path: ObjectPath) -> Result<()> {
        self.write_string::<T2>(writer, object_path)
    }

    /// The same as STRING except the length is a single byte (thus signatures
    /// have a maximum length of 255) and the content must be a valid signature (see above).
    fn write_signature(&self, writer: &mut T2, signature: Signature) -> Result<()> {
        self.write_string::<T2>(writer, signature)
    }

    /// A UINT32 giving the length of the array data in bytes, followed by alignment
    /// padding to the alignment boundary of the array element type, followed by each array element.
    fn write_array<T: DbusWriter> (&self, writer: &mut T2, a: &[T]) -> Result<()> {
        writer.write_u32::<T1>(a.len())?;
        for x in a {
            a.write(writer, x)?;
        }
        Ok(())
    }
}
//! https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling
use byteorder::{BigEndian, ReadBytesExt, ByteOrder, WriteBytesExt};

use crate::names::{BusName, InterfaceName, ErrorName, MemberName};
use crate::dbus_writer::DbusWriter;
use std::io;

/// The maximum length of a message, including header, header alignment padding,
/// and body is 2 to the 27th power or 134217728 (128 MiB).
/// Implementations must not send or accept messages exceeding this size.
const MAX_MESSAGE_SIZE: u32 = 2^27;

/// A message consists of a header and a body. If you think of a message as a package,
/// the header is the address, and the body contains the package contents.
/// Both header and body use the D-Bus [type system](https://dbus.freedesktop.org/doc/dbus-specification.html#type-system) and format for serializing data.
struct Message {
    /// The message delivery system uses the header information to figure out
    /// where to send the message and how to interpret it.
    header: Header,
    /// The body of the message is made up of zero or more arguments,
    /// which are typed values, such as an integer or a byte array.
    body: Body,
}

impl Message<T> {
    pub fn write<T1: io::Write>(&self, writer: &mut T1) -> Result<(), std::io::Error> {
        writer.write_u32::<T>(self.message_id)?;

        Ok(())
    }
}

/// Endianness flag; ASCII 'l' for little-endian or ASCII 'B' for big-endian.
/// Both header and body are in this endianness.
#[repr(u8)]
enum EndianessFlag {
    LittleEndian,
    BigEndian,
}

/// Message type. Unknown types must be ignored.
#[repr(u8)]
enum MessageType {
    /// This is an invalid type.
    Invalid = 0,
    /// Method call. This message type may prompt a reply.
    MethodCall = 1,
    /// Method reply with returned data.
    MethodReturn = 2,
    /// Error reply. If the first argument exists
    /// and is a string, it is an error message.
    Error = 3,
    /// Signal emission.
    Signal = 4,
}

bitflags! {
    struct HeaderFlags: u8 {
        /// This message does not expect method return replies or error replies,
        /// even if it is of a type that can have a reply; the reply should be omitted.
        const NO_REPLY_EXPECTED = 0x1;

        /// The bus must not launch an owner for the destination name in response to this message.
        const NO_AUTO_START = 0x1;

        /// This flag may be set on a method call message to inform the receiving side that the caller
        /// is prepared to wait for interactive authorization, which might take a considerable time to complete.
        const ALLOW_INTERACTIVE_AUTHORIZATION = 0x4;
    }
}

/// Major protocol version of the sending application.
/// If the major protocol version of the receiving application does not match,
/// the applications will not be able to communicate and the D-Bus connection must be disconnected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MajorProtocolVersion(u8);

/// The serial of this message, used as a cookie by the sender to identify
/// the reply corresponding to this request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Serial(u32);

// TODO
/// Exactly the same as STRING except the content must be a valid object path (see above).
type ObjectPath = ();

// TODO
/// The same as STRING except the length is a single byte
/// (thus signatures have a maximum length of 255) and the
/// content must be a valid signature (see above).
type Signature = ();

/// The array at the end of the header contains header fields,
/// where each field is a 1-byte field code followed by a field value.
/// A header must contain the required header fields for its message type,
/// and zero or more of any optional header fields.
#[repr(u8)]
enum HeaderFieldCode {
    /// Not a valid field name (error if it appears in a message)
    Invalid = 0,
    /// The object to send a call to, or the object a signal is emitted from.
    /// Required in `MessageType::MethodCall` and `MessageType::Signal`.
    Path = 1,
    /// The interface to invoke a method call on, or that a signal is emitted from.
    /// Required in `MessageType::Signal`.
    Interface = 2,
    /// The member, either the method name or signal name.
    /// This header field is controlled by the message sender.
    /// Required in `MessageType::MethodCall` and `MessageType::Signal`.
    Member = 3,
    /// The name of the error that occurred, for errors.
    /// Required in `MessageType::Error`.
    ErrorName = 4,
    /// The serial number of the message this message is a reply to.
    /// Required in `MessageType::Error` and `MessageType::MethodReturn`.
    ReplySerial = 5,
    /// The name of the connection this message is intended for.
    /// Optional.
    Destination = 6,
    /// Unique name of the sending connection. This field is usually only meaningful
    /// in combination with the message bus, but other servers may define their own meanings for it.
    /// Optional.
    Sender = 7,
    /// The signature of the message body. If omitted, it is assumed to be the empty signature "".
    /// Optional.
    Signature = 8,
    /// The number of Unix file descriptors that accompany the message.
    /// If omitted, it is assumed that no Unix file descriptors accompany the message.
    UnixFds = 9,
}

/// The array at the end of the header contains header fields,
/// where each field is a 1-byte field code followed by a field value.
/// A header must contain the required header fields for its message type,
/// and zero or more of any optional header fields.
#[repr(u8)]
enum HeaderField {
    /// Not a valid field name (error if it appears in a message)
    Invalid,
    /// The object to send a call to, or the object a signal is emitted from.
    /// Required in `MessageType::MethodCall` and `MessageType::Signal`.
    Path(ObjectPath),
    /// The interface to invoke a method call on, or that a signal is emitted from.
    /// Required in `MessageType::Signal`.
    Interface(InterfaceName),
    /// The member, either the method name or signal name.
    /// This header field is controlled by the message sender.
    /// Required in `MessageType::MethodCall` and `MessageType::Signal`.
    Member(MemberName),
    /// The name of the error that occurred, for errors.
    /// Required in `MessageType::Error`.
    ErrorName(ErrorName),
    /// The serial number of the message this message is a reply to.
    /// Required in `MessageType::Error` and `MessageType::MethodReturn`.
    ReplySerial(Serial),
    /// The name of the connection this message is intended for.
    /// Optional.
    Destination(String),
    /// Unique name of the sending connection. This field is usually only meaningful
    /// in combination with the message bus, but other servers may define their own meanings for it.
    /// Optional.
    Sender(String),
    /// The signature of the message body. If omitted, it is assumed to be the empty signature "".
    /// Optional.
    Signature(Signature),
    /// The number of Unix file descriptors that accompany the message.
    /// If omitted, it is assumed that no Unix file descriptors accompany the message.
    UnixFds(u32),
}

impl HeaderField {
    pub fn write<T1: ByteOrder, T2: io::Write>(&self, writer: &mut T2) -> Result<(), std::io::Error> {

        match self {
            Invalid => return Err(io::Error::new(io::ErrorKind::InvalidInput, "HeaderField::Invalid can not be marshaled!")),
            Path(object_path) => ,
            Interface(interface_name),
            Member(member_name),
            ErrorName(error_name),
            ReplySerial(serial),
            Destination(destination),
            Sender(sender),
            Signature(signature),
            UnixFds(fd) => writer.write_u32::<T1>(fd),
        };
        Ok(())
    }
}


/// The length of the header must be a multiple of 8, allowing the body to begin on
/// an 8-byte boundary when storing the entire message in a single buffer.
/// If the header does not naturally end on an 8-byte boundary up to 7 bytes of
/// nul-initialized alignment padding must be added.
/// https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields
struct Header {
    endianess_flag: EndianessFlag,
    /// Message type. Unknown types must be ignored.
    message_type: MessageType,
    /// Bitwise OR of flags. Unknown flags must be ignored.
    flags: HeaderFlags,
    /// Major protocol version of the sending application.
    /// If the major protocol version of the receiving application does not match,
    /// the applications will not be able to communicate and the D-Bus connection must be disconnected.
    major_protocol_version: MajorProtocolVersion,
    /// Length in bytes of the message body, starting from the end of the header.
    /// The header ends after its alignment padding to an 8-boundary.
    length_message_body: u32,
    /// The serial of this message, used as a cookie by the sender to identify
    /// the reply corresponding to this request. This must not be zero.
    serial: Serial,
    /// An array of zero or more header fields where the byte is the field code,
    /// and the variant is the field value. The message type determines which fields are required.
    header_fields: Vec<(HeaderFieldCode, HeaderField)>,
}

impl<T1: ByteOrder, T2: io::Write> DbusWriter<T1, T2> for Header {
    fn write(&self, writer: &mut T2) -> Result<(), std::io::Error> {

         self.write_byte(self.endianess_flag as u8)?;
         self.write_byte(self.message_type as u8)?;
         self.write_byte(self.flags.bits())?;
         self.write_byte(self.major_protocol_version.0)?;

         self.write_u32::<T1>(self.length_message_body)?;
         self.write_u32::<T1>(self.serial.0)?;

         for (code, field) in self.header_fields {
              self.write_byte(code as u8)?;
              field.write(writer);
         }
         Ok(())
    }
}


struct Body {

}
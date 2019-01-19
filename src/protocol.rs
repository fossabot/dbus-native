
//! https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling
//! https://git.devuan.org/CenturionDan/dbus/blob/debian-upstream/doc/dbus-specification.xml

/// https://dbus.freedesktop.org/doc/dbus-specification.html#auth-mechanisms
enum AuthMechanism {
    /// The EXTERNAL mechanism is defined in RFC 4422 "Simple Authentication and Security Layer (SASL)",
    /// appendix A "The SASL EXTERNAL Mechanism". This is the recommended authentication mechanism
    /// on platforms where credentials can be transferred out-of-band,
    /// in particular Unix platforms that can perform credentials-passing over the unix: transport.
    External,
    /// DBUS_COOKIE_SHA1 is a D-Bus-specific SASL mechanism.
    /// Its reference implementation is part of the reference implementation of D-Bus.
    DbusCookieSha1,
    /// The ANONYMOUS mechanism is defined in RFC 4505
    /// "Anonymous Simple Authentication and Security Layer (SASL) Mechanism".
    /// It does not perform any authentication at all, and should not be accepted by message buses.
    /// However, it might sometimes be useful for non-message-bus uses of D-Bus.
    Anonymous,
};

enum Protocol {
    /// The AUTH command is sent by the client to the server. The server replies with DATA, OK or REJECTED.
    /// If an AUTH command has no arguments, it is a request to list available mechanisms.
    /// The server must respond with a REJECTED command listing the mechanisms it understands, or with an error.
    Auth {
        mechanism: AuthMechanism
    },
    /// The CANCEL command is sent by the client to the server.
    /// The server replies with REJECTED.
    /// At any time up to sending the BEGIN command, the client may send a CANCEL command.
    /// On receiving the CANCEL command, the server must send a REJECTED command and abort the current authentication exchange.
    Cancel,
    /// The BEGIN command is sent by the client to the server. The server does not reply.
    Begin,
    /// The DATA command may come from either client or server, and simply contains a hex-encoded block of data to be interpreted
    /// according to the SASL mechanism in use. If sent by the client, the server replies with DATA, OK or REJECTED.
    Data {
        /// data in hex encoding
        data: u8,
    },
    Error {
        /// human-readable error explanation
        error_explanation: String,
    },
    /// The NEGOTIATE_UNIX_FD command is sent by the client to the server. The server replies with AGREE_UNIX_FD or ERROR.
    /// The NEGOTIATE_UNIX_FD command indicates that the client supports Unix file descriptor passing.
    NegotiateUnixFd,

    Rejected {
        /// space-separated list of mechanism names
        mechanism: [AuthMechanism]
    },
    ///  The OK command is sent by the server to the client.
    /// The OK command indicates that the client has been authenticated. The client may now proceed with negotiating Unix file descriptor passing.
    Ok {
        /// GUID in hex
        guid: Guid
    },
    AgreeUnixFd,
}

enum AuthResponse {
    Error(Protocol::Error),
    Data,
    Ok,
    Rejected,
}

enum AuthListResponse {
    Error(Protocol::Error),
    Rejected,
}

struct DBusProtocol {}

impl DBusProtocol {

    /// https://dbus.freedesktop.org/doc/dbus-specification.html#auth-protocol
    pub fn auth(mechanism: AuthMechanism) -> AuthResponse {

    }

    pub fn list_auth() -> AuthListResponse {

    }
}
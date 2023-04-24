use std::num::NonZeroU16;

/// 0 means next available port
pub type Port = NonZeroU16;

pub enum Protocol {
    TCP,
    UDP,
    HTTP1,
    HTTP2,
    HTTP3,
}

pub enum TransportLayerProtocol {
    TCP,
    UDP,
    DCCP,
    SCTP,
    RSVP,
    QUIC,
}

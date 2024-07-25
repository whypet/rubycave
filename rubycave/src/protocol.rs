mod protocol_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/protocol_capnp.rs"));
}

mod client_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/client_capnp.rs"));
}

mod server_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/server_capnp.rs"));
}

pub use client_capnp::client;
pub use server_capnp::server;

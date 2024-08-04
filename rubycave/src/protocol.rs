use regex::Regex;
use rkyv::{Archive, Deserialize, Serialize};

pub mod client;
pub mod server;

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum Packet {
    Client(client::Packet),
    Server(server::Packet),
}

pub struct PacketValidator {
    crate_version: String,
    username_regex: Regex,
}

impl PacketValidator {
    pub fn new(crate_version: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            crate_version: crate_version.to_owned(),
            username_regex: Regex::new(r"[^a-zA-Z0-9_]")?,
        })
    }

    fn check_username(&self, s: &str) -> bool {
        (1..=16).contains(&s.len()) && !self.username_regex.is_match(s)
    }

    pub fn check_client(&self, packet: &client::Packet) -> Result<(), server::PacketError> {
        match packet {
            client::Packet::Handshake { version, username } => {
                (self.crate_version == *version)
                    .then_some(())
                    .ok_or(server::PacketError::Version)?;
                self.check_username(username)
                    .then_some(())
                    .ok_or(server::PacketError::Username)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn check_server(&self, packet: &server::Packet) -> Result<(), client::PacketError> {
        match packet {
            server::Packet::Handshake { version } => {
                (self.crate_version == *version)
                    .then_some(())
                    .ok_or(client::PacketError::Version)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

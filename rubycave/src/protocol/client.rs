use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive_attr(derive(Debug))]
pub struct KeepAlive {
    epoch: u64,
}

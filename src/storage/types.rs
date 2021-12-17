use crate::protocol::Protocol;

// 所有"写"操作的命令
pub static WRITE_CMD: [&str; 2] = ["SET", "HSET"];

// 所有"删"操作的命令
pub static DELETE_CMD: [&str; 2] = ["DEL", "HDEL"];

// redis的五种数据类型
#[derive(Clone, Debug, PartialEq)]
pub enum KeyType {
    String,
    Hash,
    List,
    Set,
    ZSet,
}

impl From<Protocol> for KeyType {
    fn from(msg: Protocol) -> Self {
        match msg {
            Protocol::Set { typ, .. } => typ,
            Protocol::Get { typ, .. } => typ,
            Protocol::HSet { typ, .. } => typ,
            Protocol::HGet { typ, .. } => typ,
            Protocol::HDel { typ, .. } => typ,
            Protocol::Del { typ, .. } => typ,

            // 根本不会执行到这里，所以这个panic也根本就不会执行！
            _ => panic!(""),
        }
    }
}

// 通道中传递的是这个数据
#[derive(Debug, Clone)]
pub struct Message {
    pub protocol: Protocol,
    pub key: String,
    pub cmd: String,
}

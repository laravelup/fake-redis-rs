use crate::protocol::Protocol;
use crate::storage::hash::THash;
use crate::storage::string::TString;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

// 这里存储的运行时保存的数据
// 目前没有做持久化，进程结束后，数据消失
#[derive(Debug)]
pub struct DB {
    pub inner: Arc<Mutex<Inner>>,
    // pub inner: Inner,
}

#[derive(Debug)]
pub struct Inner {
    pub t_string: TString,
    pub t_hash: THash,
}

impl DB {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                t_string: TString::new(),
                t_hash: THash::new(),
            })),
        }
    }

    pub async fn inner(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().await
    }

    // 处理传过来的数据
    pub async fn handle(&self, message: Protocol) -> Result<String, ()> {
        let mut inner = self.inner().await;

        return match message {
            // set命令
            Protocol::Set {
                key,
                value,
                ttl,
                lock,
            } => {
                let x = inner.t_string.set(key, value, ttl, lock);

                x
            }

            // get命令
            Protocol::Get { .. } => Ok("+OK\r\n".to_string()),
            _ => Ok("+OK\r\n".to_string()),
        };
    }
}

impl Clone for DB {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            t_hash: self.t_hash.clone(),
            t_string: self.t_string.clone(),
        }
    }
}

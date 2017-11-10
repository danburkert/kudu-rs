use std::collections::{
    HashMap,
    hash_map,
};
use std::sync::Arc;

use krpc;
use parking_lot::Mutex;

use HostPort;
use Options;

#[derive(Clone)]
pub struct ProxyCache {
    inner: Arc<Inner>,
}

struct Inner {
    options: Options,
    cache: Mutex<HashMap<Box<[HostPort]>, krpc::Proxy>>,
}

impl ProxyCache {

    pub fn new(options: Options) -> ProxyCache {
        ProxyCache {
            inner: Arc::new(Inner {
                options,
                cache: Default::default(),
            }),
        }
    }

    pub fn get(&self, mut rpc_addrs: Box<[HostPort]>) -> krpc::Proxy {
        rpc_addrs.sort();
        match self.inner.cache.lock().entry(rpc_addrs) {
            hash_map::Entry::Occupied(entry) => entry.get().clone(),
            hash_map::Entry::Vacant(vacant) => {
                let proxy = krpc::Proxy::spawn(vacant.key().clone(),
                                               self.inner.options.rpc.clone(),
                                               self.inner.options.threadpool.clone(),
                                               &self.inner.options.remote);
                vacant.insert(proxy).clone()
            },
        }
    }
}

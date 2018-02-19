use std::collections::{
    HashMap,
    hash_map,
};
use std::sync::Arc;

use krpc;
use parking_lot::{
    Mutex,
    RwLock,
};

use Error;
use HostPort;
use Options;
use TabletServerId;
use pb::master::TsInfoPb;

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

#[derive(Clone)]
pub struct TabletServer(Arc<(TabletServerId, RwLock<(Box<[HostPort]>, krpc::Proxy)>)>);

impl TabletServer {

    /// Creates a new `TabletServer`.
    pub(crate) fn new(options: &Options, info: TsInfoPb) -> Result<TabletServer, Error> {
        let id = TabletServerId::parse_bytes(&info.permanent_uuid)?;
        let rpc_addrs = info.rpc_addresses
                            .into_iter()
                            .map(HostPort::from)
                            .collect::<Vec<_>>()
                            .into_boxed_slice();

        let proxy = krpc::Proxy::spawn(rpc_addrs.clone(),
                                       options.rpc.clone(),
                                       options.threadpool.clone(),
                                       &options.remote);

        Ok(TabletServer(Arc::new((id, RwLock::new((rpc_addrs, proxy))))))
    }

    /// Returns the tablet server ID.
    pub fn id(&self) -> TabletServerId {
        (self.0).0
    }

    /// Returns the tablet server RPC addresses.
    pub fn rpc_addrs(&self) -> Box<[HostPort]> {
        (self.0).1.read().0.clone()
    }

    /// Returns the tablet server KRPC proxy.
    pub(crate) fn proxy(&self) -> krpc::Proxy {
        (self.0).1.read().1.clone()
    }

    /// Updates the `InnerTabletServer` with refreshed information.
    pub(crate) fn refresh(&self, options: &Options, info: TsInfoPb) {
        debug_assert_eq!(self.id(), TabletServerId::parse_bytes(&info.permanent_uuid).unwrap());

        let rpc_addrs = info.rpc_addresses.into_iter().map(HostPort::from).collect::<Vec<_>>().into_boxed_slice();


        let mut state = (self.0).1.write();
        if rpc_addrs != state.0 {
            state.0 = rpc_addrs;
            unimplemented!("TabletServer::refresh");
        }
    }
}

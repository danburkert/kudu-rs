use std::collections::{
    HashMap,
    hash_map,
};
use std::sync::Arc;
use std::time::Duration;

use krpc;
use parking_lot::{
    Mutex,
    RwLock,
};

use HostPort;
use Options;
use TabletServerId;
use pb::master::TsInfoPb;
use Error;

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

struct TabletServerProxy {
    inner: Arc<RwLock<InnerTabletServer>>,
}


struct InnerTabletServer {
    id: TabletServerId,
    rpc_addrs: Box<[HostPort]>,
    proxy: krpc::Proxy,
}

impl TabletServerProxy {

    /// Creates a new `TabletServerProxy`.
    fn new(options: &Options, info: TsInfoPb) -> Result<TabletServerProxy, Error> {
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

        Ok(TabletServerProxy {
            inner: Arc::new(RwLock::new(InnerTabletServer { id, rpc_addrs, proxy }))
        })
    }

    /// Returns the tablet server ID.
    fn id(&self) -> TabletServerId {
        self.inner.read().id
    }

    /// Returns the tablet server RPC addresses.
    fn rpc_addrs(&self) -> Box<[HostPort]> {
        self.inner.read().rpc_addrs.clone()
    }

    /// Returns the tablet server KRPC proxy.
    fn proxy(&self) -> krpc::Proxy {
        self.inner.read().proxy.clone()
    }

    /// Updates the `InnerTabletServer` with refreshed information.
    fn update(&mut self, info: TsInfoPb) {
        let mut rpc_addrs = info.rpc_addresses.into_iter().map(HostPort::from).collect::<Vec<_>>().into_boxed_slice();
        let mut inner = self.inner.write();
        debug_assert_eq!(inner.id, TabletServerId::parse_bytes(&info.permanent_uuid).unwrap());
        if rpc_addrs != inner.rpc_addrs {
            inner.rpc_addrs = rpc_addrs;
            unimplemented!("update proxy");
        }
    }
}

impl InnerTabletServer {
}

struct RemoteTablet {
}

struct RemoteReplica {
    tserver: TabletServerProxy,
}

use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::any::Any;

use kudu_pb::master::{
    //AlterTableRequestPB,
    //AlterTableResponsePB,
    //CreateTableRequestPB,
    //CreateTableResponsePB,
    //DeleteTableRequestPB,
    //DeleteTableResponsePB,
    //GetMasterRegistrationRequestPB,
    //GetMasterRegistrationResponsePB,
    //GetTableLocationsRequestPB,
    //GetTableLocationsResponsePB,
    //GetTableSchemaRequestPB,
    //GetTableSchemaResponsePB,
    GetTabletLocationsRequestPB,
    GetTabletLocationsResponsePB,
    //IsAlterTableDoneRequestPB,
    //IsAlterTableDoneResponsePB,
    IsCreateTableDoneRequestPB,
    IsCreateTableDoneResponsePB,
    //ListMastersRequestPB,
    //ListMastersResponsePB,
    //ListTablesRequestPB,
    //ListTablesResponsePB,
    ListTabletServersRequestPB,
    ListTabletServersResponsePB,
    PingRequestPB,
    PingResponsePB,
};
use rpc::messenger::Messenger;
use rpc::{Response, RpcError};

use eventual::{Async, AsyncError, Complete, Future};
use protobuf::{Message, RepeatedField};
use parking_lot::Mutex;

const SERVICE_NAME: &'static str = "kudu.master.MasterService";

pub struct MasterProxy {
    leader: Arc<Mutex<Option<SocketAddr>>>,
    addrs: Vec<SocketAddr>,
    messenger: Arc<Messenger>,
}

impl MasterProxy {

    pub fn new(addrs: Vec<SocketAddr>,
               messenger: Arc<Messenger>)
               -> MasterProxy {
        MasterProxy {
            leader: Arc::new(Mutex::new(None)),
            addrs: addrs,
            messenger: messenger,
        }
    }

    fn send<Req, Resp>(&self,
                       method_name: &'static str,
                       timeout: Duration,
                       required_feature_flags: Vec<u32>,
                       request: Box<Req>,
                       response: Box<Resp>)
                       -> Future<Response, RpcError>
    where Req: Message + Clone, Resp: Message + Clone {

        if let Some(leader_addr) = self.leader_addr() {
            self.messenger.send(leader_addr,
                                SERVICE_NAME,
                                method_name,
                                timeout,
                                required_feature_flags,
                                request,
                                response)
        } else {
            let (complete, future) = Future::pair();

            let master_count = self.addrs.len();
            let state: Arc<Mutex<(Vec<AsyncError<RpcError>>,
                                  Option<Complete<Response, RpcError>>)>> =
                Arc::new(Mutex::new((Vec::with_capacity(master_count),
                                     Some(complete))));

            for &addr in &self.addrs {
                let state = state.clone();
                self.messenger.send(addr,
                                    SERVICE_NAME,
                                    method_name,
                                    timeout,
                                    required_feature_flags.clone(),
                                    request.clone(),
                                    response.clone())
                    .receive(move |result| {
                        match result {
                            Ok(result) => {
                                trace!("multi-master RPC success: {:?}", result);
                                if let Some(complete) = state.lock().1.take() {
                                    complete.complete(result);
                                };
                            },
                            Err(error) => {
                                trace!("multi-master RPC failure: {:?}", error);

                                let mut state = state.lock();
                                state.0.push(error);
                                if state.0.len() == master_count {
                                    warn!("all multi-master RPCs failed");
                                    state.1.take().unwrap().fail(state.0.pop().unwrap().unwrap());
                                }
                            },
                        }
                    });
            };
            future
        }
    }

    fn leader_addr(&self) -> Option<SocketAddr> {
        *self.leader.lock()
    }

    pub fn get_tablet_locations(&self, timeout: Duration, tablet_ids: Vec<Vec<u8>>) -> Future<GetTabletLocationsResponsePB, RpcError> {
        let mut request = Box::new(GetTabletLocationsRequestPB::new());
        request.set_tablet_ids(RepeatedField::from_vec(tablet_ids));
        self.send("GetTabletLocations", timeout, Vec::new(),
                  request, Box::new(GetTabletLocationsResponsePB::new()))
            .map(|response| *response.response_message.into_any().downcast::<GetTabletLocationsResponsePB>().unwrap())
    }

    // rpc CreateTable(CreateTableRequestPB) (CreateTableResponsePB);

    fn is_create_table_done(&self, timeout: Duration, request: Box<IsCreateTableDoneRequestPB>) -> Future<IsCreateTableDoneResponsePB, RpcError> {
        self.send("IsCreateTableDone", timeout, Vec::new(),
                  request, Box::new(IsCreateTableDoneResponsePB::new()))
            .map(|response| *response.response_message.into_any().downcast::<IsCreateTableDoneResponsePB>().unwrap())
    }

    pub fn is_create_table_done_by_name(&self, timeout: Duration, name: String) -> Future<IsCreateTableDoneResponsePB, RpcError> {
        let mut request = Box::new(IsCreateTableDoneRequestPB::new());
        request.mut_table().set_table_name(name);
        self.is_create_table_done(timeout, request)
    }

    pub fn is_create_table_done_by_id(&self, timeout: Duration, id: Vec<u8>) -> Future<IsCreateTableDoneResponsePB, RpcError> {
        let mut request = Box::new(IsCreateTableDoneRequestPB::new());
        request.mut_table().set_table_id(id);
        self.is_create_table_done(timeout, request)
    }

    pub fn ping(&self, timeout: Duration) -> Future<(), RpcError> {
        self.send("Ping", timeout, Vec::new(),
                  Box::new(PingRequestPB::new()),
                  Box::new(PingResponsePB::new()))
            .map(|_| ())
    }

    pub fn list_tablet_servers(&self, timeout: Duration) -> Future<ListTabletServersResponsePB, RpcError> {
        self.send("ListTabletServers", timeout, Vec::new(),
                  Box::new(ListTabletServersRequestPB::new()),
                  Box::new(ListTabletServersResponsePB::new()))
            .map(|response| *response.response_message.into_any().downcast::<ListTabletServersResponsePB>().unwrap())
    }

    /*


  rpc IsCreateTableDone(IsCreateTableDoneRequestPB) (IsCreateTableDoneResponsePB);
  rpc DeleteTable(DeleteTableRequestPB) (DeleteTableResponsePB);
  rpc AlterTable(AlterTableRequestPB) (AlterTableResponsePB);
  rpc IsAlterTableDone(IsAlterTableDoneRequestPB) (IsAlterTableDoneResponsePB);
  rpc ListTables(ListTablesRequestPB) (ListTablesResponsePB);
  rpc GetTableLocations(GetTableLocationsRequestPB) (GetTableLocationsResponsePB);
  rpc GetTableSchema(GetTableSchemaRequestPB) (GetTableSchemaResponsePB);

  rpc ListTabletServers(ListTabletServersRequestPB) (ListTabletServersResponsePB);
  rpc ListMasters(ListMastersRequestPB) (ListMastersResponsePB);
  rpc GetMasterRegistration(GetMasterRegistrationRequestPB) returns (GetMasterRegistrationResponsePB);


    */

}

trait T: Any {}

impl T for u32 {}

impl fmt::Debug for MasterProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MasterProxy {{ leader: {:?}, addrs: {:?} }}", self.leader_addr(), self.addrs)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use rpc::messenger::Messenger;
    use std::time::{Duration, Instant};
    use env_logger;
    use kudu_pb;
    use eventual::Async;
    use std::thread;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn test_ping() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default());

        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(),
                                     Arc::new(Messenger::new().unwrap()));

        let future = proxy.ping(Duration::from_millis(1000));
        let response = future.await().unwrap();
    }

    #[test]
    fn test_ping_multi_master() {
        let _ = env_logger::init();
        let mut mini_cluster_config = MiniClusterConfig::default();
        mini_cluster_config.num_masters = 3;
        let cluster = MiniCluster::new(mini_cluster_config);

        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(),
                                     Arc::new(Messenger::new().unwrap()));

        let future = proxy.ping(Duration::from_millis(1000));
        let response = future.await().unwrap();
    }
}

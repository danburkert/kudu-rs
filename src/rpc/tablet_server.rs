use std::net::SocketAddr;
use std::time::Instant;

use kudu_pb::tserver::{
    PingRequestPB, PingResponsePB,
    WriteRequestPB, WriteResponsePB,
    ScanRequestPB, ScanResponsePB,
    ScannerKeepAliveRequestPB, ScannerKeepAliveResponsePB,
    ListTabletsRequestPB, ListTabletsResponsePB,
};
use kudu_pb::tserver_service::{
    ChecksumRequestPB, ChecksumResponsePB,
};
use rpc::Rpc;

const SERVICE_NAME: &'static str = "kudu.tserver.TabletServerService";

// When macros in type position and concat_idents! land the 3rd and 4th param can be dropped.
// If/when Rust gets a snake -> camel ident converter the 2nd param can be dropped.
macro_rules! rpc {
    ($fn_name:ident, $rpc_name:ident, $request_type:ident, $response_type:ident) => {
        pub fn $fn_name(addr: SocketAddr, deadline: Instant, request: $request_type) -> Rpc {
            Rpc {
                addr: addr,
                service_name: SERVICE_NAME,
                method_name: stringify!($rpc_name),
                deadline: deadline,
                required_feature_flags: Vec::new(),
                request: Box::new(request),
                response: Box::new($response_type::new()),
                sidecars: Vec::new(),
                callback: None,
                cancel: None,
            }
        }
    };
}

rpc!(ping, Ping, PingRequestPB, PingResponsePB);
rpc!(write, Write, WriteRequestPB, WriteResponsePB);
rpc!(scan, Scan, ScanRequestPB, ScanResponsePB);
rpc!(scanner_keep_alive, ScannerKeepAlive, ScannerKeepAliveRequestPB, ScannerKeepAliveResponsePB);
rpc!(list_tablets, ListTablets, ListTabletsRequestPB, ListTabletsResponsePB);
rpc!(checksum, Checksum, ChecksumRequestPB, ChecksumResponsePB);

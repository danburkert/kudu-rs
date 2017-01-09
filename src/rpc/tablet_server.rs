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
use protobuf::Message;

use rpc::Rpc;

const SERVICE_NAME: &'static str = "kudu.tserver.TabletServerService";

// When macros in type position and concat_idents! land the 3rd and 4th param can be dropped.
// If/when Rust gets a snake -> camel ident converter the 2nd param can be dropped.
macro_rules! rpc {
    ($fn_name:ident, $rpc_name:ident, $request_type:ident, $response_type:ident) => {
        pub fn $fn_name<S>(deadline: Instant, request: $request_type) -> Rpc<S> {
            fn response() -> Box<Message> {
                Box::new($response_type::new())
            }
            Rpc::new(SERVICE_NAME,
                     stringify!($rpc_name),
                     &[],
                     response,
                     deadline,
                     Box::new(request))
        }
    };
}

rpc!(ping, Ping, PingRequestPB, PingResponsePB);
rpc!(write, Write, WriteRequestPB, WriteResponsePB);
rpc!(scan, Scan, ScanRequestPB, ScanResponsePB);
rpc!(scanner_keep_alive, ScannerKeepAlive, ScannerKeepAliveRequestPB, ScannerKeepAliveResponsePB);
rpc!(list_tablets, ListTablets, ListTabletsRequestPB, ListTabletsResponsePB);
rpc!(checksum, Checksum, ChecksumRequestPB, ChecksumResponsePB);

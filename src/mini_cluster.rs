use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;
use std::io::{
    Read,
    Write,
    BufRead,
    BufReader,
};
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::process::{Command, Child, Stdio};
use std::thread;
use std::time::Duration;

use byteorder::{
    BigEndian,
    ReadBytesExt,
    WriteBytesExt,
};
use bytes::{BufMut, BytesMut};
use itertools::Itertools;
use prost::Message;
use tempdir::TempDir;

use HostPort;
use pb::tools::{
    ControlShellRequestPb,
    ControlShellResponsePb,
    CreateClusterRequestPb,
    GetMastersRequestPb,
    GetMastersResponsePb,
    GetTServersRequestPb,
    GetTServersResponsePb,
    StartClusterRequestPb,
    StopClusterRequestPb,
};
use pb::tools::control_shell_request_pb::Request;
use pb::tools::control_shell_response_pb::Response;

pub struct MiniCluster {
    data_root: TempDir,
    process: Child,
    logger: Option<thread::JoinHandle<()>>,
}

impl MiniCluster {
    pub fn new(conf: &MiniClusterConfig) -> MiniCluster {
        let data_root = TempDir::new("kudu-mini-cluster").unwrap();
        let mut process = Command::new(&get_executable_path("kudu"))
                                  .arg("test")
                                  .arg("mini_cluster")
                                  .arg("--serialization=pb")
                                  .stdin(Stdio::piped())
                                  .stdout(Stdio::piped())
                                  .stderr(Stdio::piped())
                                  .spawn()
                                  .unwrap();

        let stderr = process.stderr.take().unwrap();
        let logger = thread::spawn(move || {
            let stderr = BufReader::new(stderr);
            for line in stderr.lines() {
                debug!("{}", line.unwrap());
            }
        });

        let mut mini_cluster = MiniCluster {
            data_root,
            process,
            logger: Some(logger),
        };

        let mut pb = conf.pb.clone();
        pb.data_root = Some(mini_cluster.data_root.path().to_str().unwrap().to_string());
        mini_cluster.send_request(Request::CreateCluster(pb));
        mini_cluster.start_cluster();

        mini_cluster
    }

    pub fn stop_cluster(&mut self) {
        self.send_request(Request::StopCluster(StopClusterRequestPb::default()));
    }

    pub fn start_cluster(&mut self) {
        self.send_request(Request::StartCluster(StartClusterRequestPb::default()));
    }

    pub fn master_addrs(&mut self) -> Vec<HostPort> {
        let response = self.send_request(Request::GetMasters(GetMastersRequestPb::default()));

        if let Some(Response::GetMasters(GetMastersResponsePb { masters })) = response {
            masters.into_iter().map(|master| master.bound_rpc_address.unwrap().into()).collect()
        } else {
            panic!("unexpected response: {:?}", response)
        }
    }

    pub fn stop_node(&mut self, addr: SocketAddr) {
        //self.nodes.get_mut(&addr).expect(&format!("no node with address {}", addr)).stop();
        unimplemented!()
    }

    pub fn start_node(&mut self, addr: SocketAddr) {
        //self.nodes.get_mut(&addr).expect(&format!("no node with address {}", addr)).start();
        unimplemented!()
    }

    fn send_request(&mut self, request: Request) -> Option<Response> {
        let stdin = self.process.stdin.as_mut().expect("stdin");
        let stdout = self.process.stdout.as_mut().expect("stdout");

        let request = ControlShellRequestPb { request: Some(request) };
        let len = request.encoded_len();

        let mut buf = Vec::with_capacity(len);
        request.encode(&mut buf).expect("encode request");

        stdin.write_u32::<BigEndian>(len as u32).expect("write len");
        stdin.write_all(&buf).expect("write request");

        buf.clear();
        let len = stdout.read_u32::<BigEndian>().expect("read len") as usize;
        buf.resize(len, 0);
        stdout.read_exact(&mut buf).expect("read response");

        let response = ControlShellResponsePb::decode(&buf).expect("decode response");
        if let Some(error) = response.error {
            panic!("mini cluster request failed: {:?}", error);
        }
        response.response
    }
}

impl Drop for MiniCluster {
    fn drop(&mut self) {
        self.process.wait().expect("wait");
        self.logger.take().unwrap().join().expect("logger");
    }
}


impl Default for MiniCluster {
    fn default() -> MiniCluster {
        MiniCluster::new(&MiniClusterConfig::default())
    }
}

/// Attempts to get the path of a Kudu executable (`kudu-master` or `kudu-tserver`). If the
/// `KUDU_HOME` environment variable is set, then that will be used, otherwise it will be searched
/// for on the path. If the executable can not be found, panic.
fn get_executable_path(executable: &str) -> PathBuf {
    if let Ok(kudu_home) = env::var("KUDU_HOME") {
        let mut bin = PathBuf::from(&kudu_home);
        bin.push("build");
        bin.push("latest");
        bin.push("bin");
        bin.push(executable);
        return bin;
    }

    let path_bytes = Command::new("which")
                             .arg(executable)
                             .output()
                             .unwrap()
                             .stdout;
    let path = CString::new(path_bytes).unwrap().into_string().unwrap();

    PathBuf::from(path.lines().next().expect(
            &format!("unable to find the {} executable. Set $KUDU_HOME or add it to $PATH",
                     executable)))
}

/// Mini cluster configuration options. Unless otherwise specified, the defaults match the master
/// and tablet server defaults.
#[derive(Default, Debug)]
pub struct MiniClusterConfig {
    pb: CreateClusterRequestPb,
}

impl MiniClusterConfig {

    pub fn num_masters(&mut self, num_masters: u32) -> &mut MiniClusterConfig {
        self.pb.num_masters = Some(num_masters as i32);
        self
    }

    pub fn num_tservers(&mut self, num_tservers: u32) -> &mut MiniClusterConfig {
        self.pb.num_tservers = Some(num_tservers as i32);
        self
    }

    pub fn enable_kerberos(&mut self) -> &mut MiniClusterConfig {
        self.pb.enable_kerberos = Some(true);
        self
    }

    pub fn log_rpc_negotiation_trace(&mut self) -> &mut MiniClusterConfig {
        self.push_flag("--rpc-trace-negotiation=true".to_string())
    }

    pub fn log_rpc_trace(&mut self) -> &mut MiniClusterConfig {
        self.push_flag("--rpc-dump-all-traces=true".to_string())
    }

    pub fn rpc_negotiation_delay(&mut self, millis: i32) -> &mut MiniClusterConfig {
        self.push_flag(format!("--rpc-negotiation-inject-delay-ms={}", millis))
    }

    pub fn get_table_locations_delay(&mut self, millis: i32) -> &mut MiniClusterConfig {
        self.pb.extra_master_flags.push(format!("--master-inject-latency-on-tablet-lookup-ms={}",
                                                millis));
        self
    }

    pub fn disable_leader_failure_detection(&mut self) -> &mut MiniClusterConfig {
        self.push_flag("--enable-leader-failure-detection=false".to_string())
    }

    pub fn disable_data_block_fsync(&mut self) -> &mut MiniClusterConfig {
        self.push_flag("--enable-data-block-fsync=false".to_string())
    }

    fn push_flag(&mut self, flag: String) -> &mut MiniClusterConfig {
        self.pb.extra_master_flags.push(flag.clone());
        self.pb.extra_tserver_flags.push(flag);
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Info = 0,
    Warning = 1,
    Error = 2,
    Fatal = 3,
}

#[cfg(test)]
mod tests {

    use env_logger;

    use super::*;

    #[test]
    fn test_spawn_mini_cluster() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::default();

        cluster.stop_cluster();
        cluster.start_cluster();
    }
}

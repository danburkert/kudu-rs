use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;
use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::process::{Command, Child, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use Client;
use ClientConfig;
use Error;
use backoff::Backoff;
use tempdir::TempDir;

/// A Kudu server node (either master or tablet server).
struct Node {
    name: String,
    bin: PathBuf,
    args: Vec<(&'static str, String)>,
    process: Option<Child>,
}

impl Node {
    fn new(name: String, bin: PathBuf, args: Vec<(&'static str, String)>) -> Node {
        Node {
            name: name,
            bin: bin,
            args: args,
            process: None,
        }
    }

    fn stop(&mut self) {
        let _ = self.process.take().expect(&format!("{} not running", &self.name)).kill();
    }

    fn start(&mut self) {
        assert!(self.process.is_none(), "{} already started", &self.name);

        let mut command = Command::new(&self.bin);

        for &(k, ref v) in &self.args {
            command.arg(format!("--{}={}", k, v));
        }

        println!("binary: {:?}", self.bin);

        command.stderr(Stdio::piped());
        let mut process = command.spawn().unwrap();
        let stderr = process.stderr.take().unwrap();

        let name = self.name.clone();
        thread::spawn(move || {
            let stderr = BufReader::new(stderr);
            for line in stderr.lines() {
                debug!("({}): {}", name, line.unwrap());
            }

        });
        self.process = Some(process);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let _ = self.process.take().map(|mut process| process.kill());
    }
}

pub struct MiniCluster {
    dir: TempDir,
    master_addrs: Vec<SocketAddr>,
    nodes: HashMap<SocketAddr, Node>,
}

impl MiniCluster {
    pub fn new(conf: &MiniClusterConfig) -> MiniCluster {
        let master_bin = get_executable_path("kudu-master");
        let tserver_bin = get_executable_path("kudu-tserver");
        let dir = TempDir::new("kudu-rs-mini-cluster").expect("unable to create temp dir");

        let num_nodes = (conf.num_masters + conf.num_tservers) as usize;
        let mut nodes = HashMap::with_capacity(num_nodes);

        let addrs = get_unbound_addresses(num_nodes);
        let (master_addrs, tserver_addrs) = addrs.split_at(conf.num_masters as usize);

        let master_addresses = master_addrs.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
        let conf_args = conf.args();

        for (i, addr) in master_addrs.iter().enumerate() {
            let path = dir.path().join(format!("master-{}", i));
            let logs = dir.path().join(format!("master-{}-logs", i));
            fs::create_dir(&logs).expect("unable to create master log directory");

            let name = format!("master:{}", addr.port());

            let mut args = conf_args.clone();
            args.push(("fs_wal_dir", path.to_string_lossy().into_owned()));
            args.push(("fs_data_dirs", path.to_string_lossy().into_owned()));
            args.push(("log_dir", logs.to_string_lossy().into_owned()));
            args.push(("rpc_bind_addresses", addr.to_string()));
            args.push(("webserver_port", 0.to_string()));
            args.push(("logtostderr", true.to_string()));
            args.push(("unlock_unsafe_flags", true.to_string()));
            args.push(("unlock_experimental_flags", true.to_string()));

            if conf.num_masters > 1 {
                args.push(("master_addresses", master_addresses.clone()));
            }

            let mut node = Node::new(name, master_bin.clone(), args);
            node.start();
            nodes.insert(*addr, node);
        }

        for (i, addr) in tserver_addrs.iter().enumerate() {
            let path = dir.path().join(format!("tserver-{}", i));
            let logs = dir.path().join(format!("tserver-{}-logs", i));
            fs::create_dir(&logs).expect("unable to create tserver log directory");

            let name = format!("tserver:{}", addr.port());

            let mut args = conf_args.clone();
            args.push(("fs_wal_dir", path.to_string_lossy().into_owned()));
            args.push(("fs_data_dirs", path.to_string_lossy().into_owned()));
            args.push(("log_dir", logs.to_string_lossy().into_owned()));
            args.push(("rpc_bind_addresses", addr.to_string()));
            args.push(("webserver_port", 0.to_string()));
            args.push(("logtostderr", true.to_string()));
            args.push(("tserver_master_addrs", master_addresses.clone()));
            args.push(("unlock_unsafe_flags", true.to_string()));
            args.push(("unlock_experimental_flags", true.to_string()));

            let mut node = Node::new(name, tserver_bin.clone(), args);
            node.start();
            nodes.insert(*addr, node);
        }

        if conf.wait_for_startup {
            let deadline = Instant::now() + Duration::from_secs(10);
            let mut backoff = Backoff::with_duration_range(50, 1000);

            let client = Client::new(ClientConfig::new(master_addrs.to_owned()));
            loop {
                match client.list_tablet_servers(deadline) {
                    Ok(ref tservers) if tservers.len() == conf.num_tservers as usize => break,
                    Ok(_) => (),
                    Err(Error::TimedOut) => panic!("timed out waiting for tablet servers to start"),
                    Err(error) => warn!("error while waiting for tservers: {:?}", error),
                }
                let backoff_ms = backoff.next_backoff_ms();
                trace!("waiting {}ms for {} tablet servers to register",
                       backoff_ms, conf.num_tservers);
                thread::sleep(Duration::from_millis(backoff_ms));
            }
        }

        MiniCluster {
            dir: dir,
            master_addrs: master_addrs.to_owned(),
            nodes: nodes,
        }
    }

    pub fn master_addrs(&self) -> &[SocketAddr] {
        &self.master_addrs
    }

    pub fn stop_node(&mut self, addr: SocketAddr) {
        self.nodes.get_mut(&addr).expect(&format!("no node with address {}", addr)).stop();
    }

    pub fn start_node(&mut self, addr: SocketAddr) {
        self.nodes.get_mut(&addr).expect(&format!("no node with address {}", addr)).start();
    }
}

impl Default for MiniCluster {
    fn default() -> MiniCluster {
        MiniCluster::new(&MiniClusterConfig::default())
    }
}

pub fn get_unbound_address() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap()
}

/// Attempts to get local unbound socket addresses for testing.
fn get_unbound_addresses(count: usize) -> Vec<SocketAddr> {
    let mut listeners = Vec::with_capacity(count);
    let mut addrs = Vec::with_capacity(count);

    for _ in 0..count {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        listeners.push(listener);
        addrs.push(addr);
    }
    addrs
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
pub struct MiniClusterConfig {
    /// The number of masters in the mini cluster. Default: 1.
    num_masters: u32,
    /// The number of tablet servers in the mini cluster. Default: 1.
    num_tservers: u32,

    wait_for_startup: bool,

    // Logging Options

    /// Minimum logging level.
    min_log_level: Option<LogLevel>,

    /// Show all `VLOG(m)` messages for `m < this`. Overridden by `vmodule`.
    vlog: Option<i32>,

    /// Per-module `VLOG` level. List of (<module name>, <log level>) pairs. <module name> is a
    /// glob pattern, matched against the filename base (that is, name ignoring
    /// `.cc`/`.h`/`-inl.h`). <log level> overrides any value given by `vlog`.
    vmodule: Vec<(String, i32)>,

    /// If enabled, dump traces of all RPC negotiations to the log at `INFO` level.
    log_rpc_negotiation_trace: Option<bool>,

    /// If enabled, dump traces of all RPCs to the log at `INFO` level.
    log_rpc_trace: Option<bool>,

    // Latency Injection

    /// Amount of latency in milliseconds to inject into the RPC negotiation process.
    rpc_negotiation_delay: Option<i32>,

    /// Amount of latency in milliseconds to inject in GetTableLocations RPCs.
    get_table_locations_delay: Option<i32>,

    // Miscellaneous

    /// Whether to enable failure detection of tablet leaders. If enabled, attempts will be made to
    /// elect a follower as a new leader when the leader is detected to have failed.
    leader_failure_detection: Option<bool>,

    /// Whether to enable fsync() of data blocks, metadata, and their parent directories. Disabling
    /// this flag may cause data loss in the event of a system crash. Default: false.
    data_block_fsync: bool,
}

impl MiniClusterConfig {

    pub fn num_masters(&mut self, num_masters: u32) -> &mut MiniClusterConfig {
        self.num_masters = num_masters;
        self
    }

    pub fn num_tservers(&mut self, num_tservers: u32) -> &mut MiniClusterConfig {
        self.num_tservers = num_tservers;
        self
    }

    pub fn wait_for_startup(&mut self, wait_for_startup: bool) -> &mut MiniClusterConfig {
        self.wait_for_startup = wait_for_startup;
        self
    }

    pub fn min_log_level(&mut self, log_level: LogLevel) -> &mut MiniClusterConfig {
        self.min_log_level = Some(log_level);
        self
    }

    pub fn vlog(&mut self, vlog_level: i32) -> &mut MiniClusterConfig {
        assert!(vlog_level >= 0);
        self.vlog = Some(vlog_level);
        self
    }

    pub fn vmodule(&mut self, module: String, vlog_level: i32) -> &mut MiniClusterConfig {
        assert!(vlog_level >= 0);
        self.vmodule.push((module, vlog_level));
        self
    }

    pub fn log_rpc_negotiation_trace(&mut self, log_rpc_negotiation_trace: bool) -> &mut MiniClusterConfig {
        self.log_rpc_negotiation_trace = Some(log_rpc_negotiation_trace);
        self
    }

    pub fn log_rpc_trace(&mut self, log_rpc_trace: bool) -> &mut MiniClusterConfig {
        self.log_rpc_trace = Some(log_rpc_trace);
        self
    }

    pub fn rpc_negotiation_delay(&mut self, millis: i32) -> &mut MiniClusterConfig {
        self.rpc_negotiation_delay = Some(millis);
        self
    }

    pub fn get_table_locations_delay(&mut self, millis: i32) -> &mut MiniClusterConfig {
        self.get_table_locations_delay = Some(millis);
        self
    }

    pub fn leader_failure_detection(&mut self, leader_failure_detection: bool) -> &mut MiniClusterConfig {
        self.leader_failure_detection = Some(leader_failure_detection);
        self
    }

    pub fn data_block_fsync(&mut self, data_block_fsync: bool) -> &mut MiniClusterConfig {
        self.data_block_fsync = data_block_fsync;
        self
    }

    fn args(&self) -> Vec<(&'static str, String)> {
        let mut args = Vec::new();

        if let Some(log_level) = self.min_log_level {
            args.push(("minloglevel", (log_level as usize).to_string()));
        }

        if let Some(vlog_level) = self.vlog {
            args.push(("v", vlog_level.to_string()));
        }

        if !self.vmodule.is_empty() {
            let modules = self.vmodule
                              .iter()
                              .map(|&(ref module, ref level)| format!("{}={}", module, level))
                              .collect::<Vec<_>>();
            args.push(("vmodule", modules.join(",")));
        }

        if let Some(log_rpc_negotiation_trace) = self.log_rpc_negotiation_trace {
            args.push(("rpc_trace_negotiation", log_rpc_negotiation_trace.to_string()));
        }

        if let Some(log_rpc_trace) = self.log_rpc_trace {
            args.push(("rpc_dump_all_traces", log_rpc_trace.to_string()));
        }

        if let Some(rpc_negotiation_delay) = self.rpc_negotiation_delay {
            args.push(("rpc_negotiation_inject_delay_ms", rpc_negotiation_delay.to_string()));
        }

        if let Some(get_table_locations_delay) = self.get_table_locations_delay {
            args.push(("master_inject_latency_on_tablet_lookup_ms",
                       get_table_locations_delay.to_string()));
        }

        if let Some(leader_failure_detection) = self.leader_failure_detection {
            args.push(("enable_leader_failure_detection",
                       leader_failure_detection.to_string()));
        }

        args.push(("enable_data_block_fsync",
                   self.data_block_fsync.to_string()));

        args
    }
}

impl Default for MiniClusterConfig {
    fn default() -> MiniClusterConfig {
        MiniClusterConfig {
            num_masters: 1,
            num_tservers: 1,
            wait_for_startup: true,
            min_log_level: None,
            vlog: None,
            vmodule: Vec::new(),
            log_rpc_negotiation_trace: None,
            log_rpc_trace: None,
            rpc_negotiation_delay: None,
            get_table_locations_delay: None,
            leader_failure_detection: None,
            data_block_fsync: false,
        }
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
        let _cluster = MiniCluster::default();
    }
}

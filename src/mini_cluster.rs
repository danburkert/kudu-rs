use std::env;
use std::fs;
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::process::{Command, Child, Stdio};
use std::thread;
use std::io::{BufRead, BufReader};

use tempdir::TempDir;

/// A process handle that will kill the process when it goes out of scope.
pub struct ProcessHandle(Child);
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

#[allow(dead_code)]
pub struct MiniCluster {
    dir: TempDir,
    master_addrs: Vec<SocketAddr>,
    master_procs: Vec<ProcessHandle>,
    tserver_procs: Vec<ProcessHandle>,
}

impl MiniCluster {
    pub fn new(conf: &MiniClusterConfig) -> MiniCluster {
        let kudu_home = env::var("KUDU_HOME").expect("KUDU_HOME environment variable must be set");
        let mut bin_dir = PathBuf::from(&kudu_home);
        bin_dir.push("build");
        bin_dir.push("latest");
        bin_dir.push("bin");

        let master_bin = bin_dir.join("kudu-master");
        let tserver_bin = bin_dir.join("kudu-tserver");
        let dir = TempDir::new("kudu-rs-mini-cluster").expect("unable to create temp dir");

        let mut master_procs = Vec::with_capacity(conf.num_masters as usize);
        let mut master_addrs = Vec::with_capacity(conf.num_masters as usize);

        for i in 0..conf.num_masters {
            let addr = get_unbound_address();

            let path = dir.path().join(format!("master-{}", i));
            let logs = dir.path().join(format!("master-{}-logs", i));
            fs::create_dir(&logs).expect("unable to create master log directory");
            let mut command = Command::new(&master_bin);
            command.arg(format!("--fs_wal_dir={}", path.to_str().unwrap()))
                   .arg(format!("--fs_data_dirs={}", path.to_str().unwrap()))
                   .arg(format!("--log_dir={}", logs.to_str().unwrap()))
                   .arg(format!("--rpc_bind_addresses={}", addr))
                   .arg("--webserver_port=0")
                   .arg("--logtostderr");

            command.stderr(Stdio::piped());
            conf.configure(&mut command);

            let mut process = command.spawn().expect("unable to start master");
            let stderr = process.stderr.take().unwrap();
            let port = addr.port();

            thread::spawn(move || {
                let stderr = BufReader::new(stderr);
                for line in stderr.lines() {
                    debug!("(master:{}): {}", port, line.unwrap());
                }

            });

            master_procs.push(ProcessHandle(process));
            master_addrs.push(addr);
        }

        let mut tserver_procs = Vec::with_capacity(conf.num_tservers as usize);
        let masters = master_addrs.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
        for i in 0..conf.num_tservers {
            let addr = get_unbound_address();
            let path = dir.path().join(format!("tserver-{}", i));
            let logs = dir.path().join(format!("tserver-{}-logs", i));
            fs::create_dir(&logs).expect("unable to create tserver log directory");

            let mut command = Command::new(&tserver_bin);
            command.arg(format!("--fs_wal_dir={}", path.to_str().unwrap()))
                   .arg(format!("--fs_data_dirs={}", path.to_str().unwrap()))
                   .arg(format!("--log_dir={}", logs.to_str().unwrap()))
                   .arg(format!("--rpc_bind_addresses={}", addr))
                   .arg("--webserver_port=0")
                   .arg(format!("--tserver_master_addrs={}", masters))
                   .arg("--logtostderr");

            command.stderr(Stdio::piped());
            conf.configure(&mut command);

            let mut process = command.spawn().expect("unable to start master");
            let stderr = process.stderr.take().unwrap();
            let port = addr.port();

            thread::spawn(move || {
                let stderr = BufReader::new(stderr);
                for line in stderr.lines() {
                    debug!("(tserver:{}): {}", port, line.unwrap());
                }
            });

            tserver_procs.push(ProcessHandle(process));
        }

        MiniCluster {
            dir: dir,
            master_addrs: master_addrs,
            master_procs: master_procs,
            tserver_procs: tserver_procs,
        }
    }

    #[allow(dead_code)]
    pub fn master_addrs(&self) -> &[SocketAddr] {
        &self.master_addrs
    }
}

impl Default for MiniCluster {
    fn default() -> MiniCluster {
        MiniCluster::new(&MiniClusterConfig::default())
    }
}

/// Attempts to get a local unbound socket address for testing.
fn get_unbound_address() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap()
}

/// Mini cluster configuration options. Unless otherwise specified, the defaults match the master
/// and tablet server defaults.
pub struct MiniClusterConfig {
    /// The number of masters in the mini cluster. Default: 1.
    num_masters: i32,
    /// The number of tablet servers in the mini cluster. Default: 1.
    num_tservers: i32,

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

    pub fn num_masters(&mut self, num_masters: i32) -> &mut MiniClusterConfig {
        self.num_masters = num_masters;
        self
    }

    pub fn num_tservers(&mut self, num_tservers: i32) -> &mut MiniClusterConfig {
        self.num_tservers = num_tservers;
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

    fn configure(&self, command: &mut Command) {

        if let Some(log_level) = self.min_log_level {
            command.arg(format!("--minloglevel={}", log_level as usize));
        }

        if let Some(vlog_level) = self.vlog {
            command.arg(format!("--v={}", vlog_level));
        }

        if !self.vmodule.is_empty() {
            let modules = self.vmodule
                              .iter()
                              .map(|&(ref module, ref level)| format!("{}={}", module, level))
                              .collect::<Vec<_>>();
            command.arg(format!("--vmodule={}", modules.join(",")));
        }

        if let Some(log_rpc_negotiation_trace) = self.log_rpc_negotiation_trace {
            command.arg(format!("--rpc_trace_negotiation={}", log_rpc_negotiation_trace));
        }

        if let Some(log_rpc_trace) = self.log_rpc_trace {
            command.arg(format!("--rpc_dump_all_traces={}", log_rpc_trace));
        }

        if let Some(rpc_negotiation_delay) = self.rpc_negotiation_delay {
            command.arg(format!("--rpc_negotiation_inject_delay_ms={}",
                                rpc_negotiation_delay));
        }

        if let Some(get_table_locations_delay) = self.get_table_locations_delay {
            command.arg(format!("--master_inject_latency_on_tablet_lookups_ms={}",
                                get_table_locations_delay));
        }

        if let Some(leader_failure_detection) = self.leader_failure_detection {
            command.arg(format!("--enable_leader_failure_detection={}",
                                leader_failure_detection));
        }

        command.arg(format!("--enable_data_block_fsync={}", self.data_block_fsync));
    }
}

impl Default for MiniClusterConfig {
    fn default() -> MiniClusterConfig {
        MiniClusterConfig {
            num_masters: 1,
            num_tservers: 1,
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

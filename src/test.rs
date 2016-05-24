use std::env;
use std::fs;
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use std::sync::{StaticMutex, MUTEX_INIT};

use tempdir::TempDir;

static LOCK: StaticMutex = MUTEX_INIT;

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
    master_procs: Vec<ProcessHandle>,
    master_addrs: Vec<SocketAddr>,
    tserver_procs: Vec<ProcessHandle>,
}

impl MiniCluster {
    pub fn new(conf: MiniClusterConfig) -> MiniCluster {
        // TODO: this prevents a deadlock which seamingly originates in TCMalloc. Figure out why
        // this is necessary.
        let _lock = LOCK.lock().unwrap();

        let kudu_home = env::var("KUDU_HOME").expect("KUDU_HOME environment variable must be set");
        let mut bin_dir = PathBuf::from(&kudu_home);
        bin_dir.push("build");
        bin_dir.push("latest");
        bin_dir.push("bin");

        let master_bin = bin_dir.join("kudu-master");
        let tserver_bin = bin_dir.join("kudu-tserver");
        let dir = TempDir::new("kudu-rs-mini-cluster").expect("unable to create temp dir");

        let mut master_procs = Vec::with_capacity(conf.num_masters);
        let mut master_addrs = Vec::with_capacity(conf.num_masters);

        for i in 0..conf.num_masters {
            let addr = get_unbound_address();

            let path = dir.path().join(format!("master-{}", i));
            let logs = dir.path().join(format!("master-{}-logs", i));
            fs::create_dir(&logs).expect("unable to create master log directory");
            let process = Command::new(&master_bin).arg("--webserver_port=0")
                                                  .arg("--minloglevel=0")
                                                  .arg("--v=2")
                                                  .arg("--enable_data_block_fsync=false")
                                                  .arg("--enable_leader_failure_detection=true")
                                                  .arg(format!("--fs_wal_dir={}", path.to_str().unwrap()))
                                                  .arg(format!("--fs_data_dirs={}", path.to_str().unwrap()))
                                                  .arg(format!("--log_dir={}", logs.to_str().unwrap()))
                                                  .arg(format!("--rpc_bind_addresses={}", addr))
                                                  .spawn().expect("unable to start master");
            master_procs.push(ProcessHandle(process));
            master_addrs.push(addr);
        }

        let mut tserver_procs = Vec::with_capacity(conf.num_tservers);
        let masters = master_addrs.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
        for i in 0..conf.num_tservers {
            let addr = get_unbound_address();
            let path = dir.path().join(format!("tserver-{}", i));
            let logs = dir.path().join(format!("tserver-{}-logs", i));
            fs::create_dir(&logs).expect("unable to create tserver log directory");
            let process = Command::new(&tserver_bin).arg("--webserver_port=0")
                                                   .arg("--minloglevel=0")
                                                   .arg("--v=2")
                                                   .arg("--enable_data_block_fsync=false")
                                                   .arg("--enable_leader_failure_detection=true")
                                                   .arg(format!("--fs_wal_dir={}", path.to_str().unwrap()))
                                                   .arg(format!("--fs_data_dirs={}", path.to_str().unwrap()))
                                                   .arg(format!("--log_dir={}", logs.to_str().unwrap()))
                                                   .arg(format!("--rpc_bind_addresses={}", addr))
                                                   .arg(format!("--tserver_master_addrs={}", masters))
                                                   .spawn().expect("unable to start tablet server");
            tserver_procs.push(ProcessHandle(process));
        }

        MiniCluster {
            dir: dir,
            master_procs: master_procs,
            master_addrs: master_addrs,
            tserver_procs: tserver_procs,
        }
    }

    #[allow(dead_code)]
    pub fn master_addrs(&self) -> &[SocketAddr] {
        &self.master_addrs
    }
}

/// Attempts to get a local unbound socket address for testing.
fn get_unbound_address() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap()
}

pub struct MiniClusterConfig {
    pub num_masters: usize,
    pub num_tservers: usize,
}

impl Default for MiniClusterConfig {
    fn default() -> MiniClusterConfig {
        MiniClusterConfig {
            num_masters: 1,
            num_tservers: 1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_spawn_mini_cluster() {
        MiniCluster::new(MiniClusterConfig::default());
    }
}

use std::env;
use std::fs;
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::process::{Command, Child};

use tempdir::TempDir;

pub struct MiniCluster {
    _dir: TempDir,
    master_procs: Vec<Child>,
    master_addrs: Vec<SocketAddr>,
    tserver_procs: Vec<Child>,
}

impl MiniCluster {
    pub fn new(conf: MiniClusterConfig) -> MiniCluster {
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
            master_procs.push(process);
            master_addrs.push(addr);
        }

        let mut tserver_procs = Vec::with_capacity(conf.num_tservers);
        let masters = master_addrs.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
        for i in 0..conf.num_masters {
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
                                                   .spawn().expect("unable to start master");
            tserver_procs.push(process);
        }

        MiniCluster {
            _dir: dir,
            master_procs: master_procs,
            master_addrs: master_addrs,
            tserver_procs: tserver_procs,
        }

    }

    pub fn master_addrs(&self) -> &[SocketAddr] {
        &self.master_addrs
    }
}

/// Attempts to get a local unbound socket address for testing.
fn get_unbound_address() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap()
}

impl Drop for MiniCluster {
    fn drop(&mut self) {
        for mut tserver in &mut self.tserver_procs {
            tserver.kill().expect("unable to shutdown tablet server");
            tserver.wait().expect("unable to wait for tablet server shutdown");
        }

        for mut master in &mut self.master_procs {
            master.kill().expect("unable to shutdown master server");
            master.wait().expect("unable to wait for master server shutdown");
        }
    }
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

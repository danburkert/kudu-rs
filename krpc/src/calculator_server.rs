use std::env;
use std::ffi::CString;
use std::net::{
    SocketAddr,
    TcpListener,
};
use std::path::PathBuf;
use std::process::{
    Child,
    Command,
    Stdio,
};

#[must_use]
struct CalculatorServer {
    process: Child,
}

impl CalculatorServer {

    fn start() -> CalculatorServer {
        let bin = get_executable_path("calculator-server");

        let mut command = Command::new(get_executable_path(&"calculator-server"));
        command.arg("--addr").arg(&get_unbound_address().to_string());

        command.stderr(Stdio::piped());
        let mut process = command.spawn().expect("spawn");

        CalculatorServer {
            process: process,
        }
    }
}

/// Attempts to get the path of a Kudu executable. If the `KUDU_HOME` environment variable is set,
/// then that will be used, otherwise it will be searched for on the path. If the executable can
/// not be found, panic.
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
                             .expect("which")
                             .stdout;
    let path = CString::new(path_bytes).expect("cstring").into_string().expect("into_string");

    PathBuf::from(path.lines().next().expect(
            &format!("unable to find the {} executable. Set $KUDU_HOME or add it to $PATH",
                     executable)))
}

/// Attempts to get local unbound socket address for testing.
fn get_unbound_address() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0").expect("bind").local_addr().expect("local_addr")
}


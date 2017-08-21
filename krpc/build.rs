extern crate curl;
extern crate prost_build;
extern crate tempdir;

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use curl::easy::Easy;
use tempdir::TempDir;

const VERSION: &'static str = "1.4.0";
const URL_BASE: &'static str = "https://raw.githubusercontent.com/apache/kudu";

const PROTOS: &[&str] = &[
    "rpc/rpc_header.proto",
    "security/token.proto",
    "util/pb_util.proto",
];

fn main() {
    let kudu_home = match env::var("KUDU_HOME") {
        Ok(kudu_home) => PathBuf::from(kudu_home),
        Err(_) => {
            let kudu_home =
                PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                        .join(format!("apache-kudu-{}", VERSION));

            if !kudu_home.exists() {
                download_protos(&kudu_home);
            }
            kudu_home
        },
    };

    let protos = PROTOS.iter()
                       .map(|proto| kudu_home.join("src").join("kudu").join(proto))
                       .collect::<Vec<_>>();

    prost_build::compile_protos(&protos, &[kudu_home.join("src")]).unwrap();
}

fn download_protos(target: &Path) {
    let tempdir = TempDir::new_in(target.parent().unwrap(), "proto-download").unwrap();
    let mut path = tempdir.path().to_owned();
    path.push("src");
    fs::create_dir(&path).unwrap();
    path.push("kudu");
    fs::create_dir(&path).unwrap();

    let mut data = Vec::new();
    for proto in PROTOS {
        data.clear();
        let proto_path = path.join(proto);
        fs::create_dir_all(proto_path.parent().unwrap()).unwrap();

        let mut handle = Easy::new();

        handle.url(&format!("{url_base}/{version}/src/kudu/{proto}",
                            url_base=URL_BASE,
                            version=VERSION,
                            proto=proto))
              .expect("failed to configure Kudu URL");
        handle.follow_location(true)
              .expect("failed to configure follow location");
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).expect("failed to write download data");
            transfer.perform().expect("failed to download Kudu source tarball");
        }

        fs::File::create(proto_path).unwrap().write_all(&data).unwrap();
    }

    fs::rename(&tempdir.into_path(), target).expect("unable to move temporary directory");
}

extern crate curl;
extern crate env_logger;
extern crate flate2;
extern crate krpc_build;
extern crate prost_build;
extern crate tar;

use std::env;
use std::io::Cursor;
use std::path::PathBuf;

use curl::easy::Easy;
use flate2::bufread::GzDecoder;
use tar::Archive;

const VERSION: &'static str = "1.7.1";

fn main() {
    env_logger::init();
    let target = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"));
    let dir = target.join(format!("kudu-{}", VERSION));

    // Download the Kudu source tarball.
    if !dir.exists() {
        let mut data = Vec::new();
        let mut handle = Easy::new();

        handle.url(&format!("https://github.com/apache/kudu/archive/{}.tar.gz", VERSION))
              .expect("failed to configure Kudu tarball URL");
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

        Archive::new(GzDecoder::new(Cursor::new(data)))
                .unpack(target).expect("failed to unpack Kudu source tarball");
    }

    prost_build::Config::new()
                        .service_generator(Box::new(krpc_build::KrpcServiceGenerator))
                        .compile_protos(&[dir.join("src/kudu/client/client.proto"),
                                          dir.join("src/kudu/consensus/metadata.proto"),
                                          dir.join("src/kudu/master/master.proto"),
                                          dir.join("src/kudu/rpc/rpc_header.proto"),
                                          dir.join("src/kudu/tools/tool.proto"),
                                          dir.join("src/kudu/tserver/tserver_service.proto")],
                                        &[dir.join("src")]).unwrap();
}

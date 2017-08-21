extern crate prost_build;

use std::env;
use std::path::PathBuf;

fn main() {
    let kudu_home = PathBuf::from(env::var("KUDU_HOME").expect("KUDU_HOME must be set"));

    let rtest = kudu_home.join("src").join("kudu").join("rpc").join("rtest.proto");
    let src = kudu_home.join("src");

    prost_build::Config::new()
                        .type_substitutions(vec![(".kudu.rpc.UserInformationPB",
                                                  "::krpc::pb::rpc::UserInformationPb")])
                        .compile_protos(&[rtest], &[src])
                        .unwrap();
}

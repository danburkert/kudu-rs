extern crate curl;
extern crate env_logger;
extern crate flate2;
extern crate prost_build;
extern crate tar;

use std::env;
use std::io::Cursor;
use std::path::PathBuf;

use curl::easy::Easy;
use flate2::bufread::GzDecoder;
use tar::Archive;

const VERSION: &'static str = "master";

fn main() {
    env_logger::init().unwrap();
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

        Archive::new(GzDecoder::new(Cursor::new(data)).expect("failed to create gzip decoder"))
                .unpack(target).expect("failed to unpack Kudu source tarball");
    }

    prost_build::Config::new()
                        .service_generator(Box::new(KuduServiceGenerator))
                        .compile_protos(&[dir.join("src/kudu/client/client.proto"),
                                          dir.join("src/kudu/consensus/metadata.proto"),
                                          dir.join("src/kudu/master/master.proto"),
                                          dir.join("src/kudu/rpc/rpc_header.proto"),
                                          dir.join("src/kudu/tools/tool.proto"),
                                          dir.join("src/kudu/tserver/tserver_service.proto")],
                                        &[dir.join("src")]).unwrap();
}

#[derive(Debug)]
pub struct KuduServiceGenerator;

impl prost_build::ServiceGenerator for KuduServiceGenerator {
    fn generate(&self, service: prost_build::Service, buf: &mut String) {
            // Generate a trait for the service.
            service.comments.append_with_indent(0, buf);
            buf.push_str(&format!("pub struct {};\n", &service.name));
            buf.push_str(&format!("impl {} {{\n", &service.name));

            // Generate the method implementations.
            for method in &service.methods {
                method.comments.append_with_indent(1, buf);
                buf.push_str(&format!("    pub fn {}(\n", method.name));
                buf.push_str(&format!("    request: ::std::sync::Arc<{}>,\n", method.input_type));
                buf.push_str("        deadline: ::std::time::Instant,\n");
                buf.push_str(&format!("    ) -> ::krpc::Call<{}, {}> {{\n", method.input_type, method.output_type));

                buf.push_str("        ::krpc::Call::new(\n");
                buf.push_str(&format!("            \"{}.{}\",\n", service.package, service.proto_name));
                buf.push_str(&format!("            \"{}\",\n", method.proto_name));
                buf.push_str("            request,\n");
                buf.push_str("            deadline,\n");
                buf.push_str("        )\n");
                buf.push_str("    }\n");
            }

            // Close out the struct.
            buf.push_str("}\n");
    }
}

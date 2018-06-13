extern crate prost_build;

#[derive(Debug)]
pub struct KrpcServiceGenerator;

impl prost_build::ServiceGenerator for KrpcServiceGenerator {
    fn generate(&mut self, service: prost_build::Service, buf: &mut String) {
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

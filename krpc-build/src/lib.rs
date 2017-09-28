extern crate prost_build;

#[derive(Debug)]
pub struct KrpcServiceGenerator;

impl prost_build::ServiceGenerator for KrpcServiceGenerator {
    fn generate(&self, service: prost_build::Service, buf: &mut String) {
            // Generate a trait for the service.
            service.comments.append_with_indent(0, buf);
            buf.push_str(&format!("pub trait {} {{\n", &service.name));

            // Generate the service methods.
            for method in &service.methods {
                method.comments.append_with_indent(1, buf);
                buf.push_str(&format!("    fn {}(\n", method.name));
                buf.push_str("        &mut self,\n");
                buf.push_str(&format!("        request: ::std::boxed::Box<{}>,\n", method.input_type));
                buf.push_str("        deadline: ::std::time::Instant,\n");
                buf.push_str("        required_feature_flags: &'static [u32],\n");
                buf.push_str(&format!("    ) -> ::krpc::Response<{}>;\n", method.output_type));
            }

            // Close out the trait.
            buf.push_str("}\n");

            // Impl the trait for Proxy.
            buf.push_str(&format!("impl {} for ::krpc::Proxy {{\n", &service.name));

            // Generate the method implementations.
            for method in &service.methods {
                method.comments.append_with_indent(1, buf);
                buf.push_str(&format!("    fn {}(\n", method.name));
                buf.push_str("        &mut self,\n");
                buf.push_str(&format!("        request: ::std::boxed::Box<{}>,\n", method.input_type));
                buf.push_str("        deadline: ::std::time::Instant,\n");
                buf.push_str("        required_feature_flags: &'static [u32],\n");
                buf.push_str(&format!("    ) -> ::krpc::Response<{}> {{\n", method.output_type));

                buf.push_str("        let request = ::krpc::Request {\n");
                buf.push_str(&format!("            service: \"{}.{}\",\n", service.package, service.proto_name));
                buf.push_str(&format!("            method: \"{}\",\n", method.proto_name));
                buf.push_str("            required_feature_flags,\n");
                buf.push_str("            body: request,\n");
                buf.push_str("            timestamp: ::std::time::Instant::now(),\n");
                buf.push_str("            deadline,\n");
                buf.push_str("        };\n");
                buf.push_str("        self.send(request)\n");
                buf.push_str("    }\n");
            }

            // Close out the impl block.
            buf.push_str("}\n");
    }
}

extern crate prost_build;

#[derive(Debug)]
pub struct KrpcServiceGenerator;

impl prost_build::ServiceGenerator for KrpcServiceGenerator {
    fn generate(&self, service: prost_build::Service, buf: &mut String) {
            // Generate a trait for the service.
            service.comments.append_with_indent(0, buf);
            buf.push_str(&format!("pub trait {} {{\n", &service.name));

            // Generate the service methods.
            for method in service.methods {
                method.comments.append_with_indent(1, buf);
                buf.push_str(&format!("    fn {}({}) -> {};\n",
                                      method.name,
                                      method.input_type,
                                      method.output_type));
            }

            // Close out the trait.
            buf.push_str("}\n");
    }
}

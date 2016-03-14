use std::env;

fn main() {
    if let Ok(path) = env::var("KUDU_HOME") {
        println!("cargo:rustc-link-search=native={}/build/latest/lib", path);
    }
}

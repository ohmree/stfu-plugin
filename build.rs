use std::env;

pub fn main() {
    if env::var("PROFILE").as_ref().map(String::as_str) == Ok("debug") {
        println!(r#"cargo:rustc-cfg=feature="debug""#);
    }
}

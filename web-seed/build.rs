use std::env;

fn main() {
    let api_url = env::var("API_URL").unwrap();
    println!("cargo:rustc-env=API_URL={}", api_url);
}

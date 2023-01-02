fn main() {
    if std::env::consts::OS == "windows" {
        println!("cargo:rustc-link-arg=.res");
    }
}
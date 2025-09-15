fn main() {
    println!("cargo::rustc-link-lib=c");
    println!("cargo::rustc-link-lib=dragon");
    println!("cargo::rustc-link-lib=dragonsys");
}

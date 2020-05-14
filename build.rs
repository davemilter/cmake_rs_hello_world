use std::path::Path;

fn main() {
    let dst = cmake::Config::new(Path::new("cpp_proj"))
        .build_target("cpp_proj")
        .build()
        .join("build");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=cpp_proj");
}

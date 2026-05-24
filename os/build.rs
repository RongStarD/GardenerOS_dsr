use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // 将链接脚本复制到输出目录
    fs::copy("src/linker.ld", out.join("linker.ld")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rerun-if-changed=src/linker.ld");
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=dl");

    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=dl");

    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}

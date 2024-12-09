fn main() {
    println!("cargo:rustc-env=TAURI_ICON=none");
    tauri_build::build()
}

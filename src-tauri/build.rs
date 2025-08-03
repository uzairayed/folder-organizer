fn main() {
    // Skip tauri-build for now to avoid icon requirements
    // tauri_build::build()
    
    // Just set some basic build flags
    println!("cargo:rerun-if-changed=tauri.conf.json");
    println!("cargo:rustc-cfg=desktop");
} 
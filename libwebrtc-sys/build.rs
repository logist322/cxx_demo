fn main() {
    println!("cargo:rustc-link-search=native=../libwebrtc/");
    println!("cargo:rustc-link-lib=static=webrtc");

    cxx_build::bridge("src/lib.rs")
        .file("src/bridge.cc")
        .compile("libwebrtc-sys");

    println!("cargo:rerun-if-changed=src/lib.rs.rs");
    println!("cargo:rerun-if-changed=src/bridge.cc");
    println!("cargo:rerun-if-changed=../libwebrtc/libwebrtc.cc");
    println!("cargo:rerun-if-changed=src/bridge.h");
}

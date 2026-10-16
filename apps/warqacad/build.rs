//! Windows only: embed the app icon and version info into `warqacad.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/warqacad.ico");
    println!("cargo:rerun-if-env-changed=WARQACAD_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=WARQACAD_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/warqacad.ico").exists() {
        res.set_icon("../../assets/app-icon/warqacad.ico");
    }
    res.set("ProductName", "WarqaCAD")
        .set("FileDescription", "WarqaCAD computer-aided design")
        .set("CompanyName", "Abdul Bari")
        .set("LegalCopyright", "Copyright (c) 2026 Abdul Bari. Based on CADCraft (c) 2026 ArtCraft Team and the CADCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "warqacad.exe")
        .set("InternalName", "warqacad");
    if let Err(e) = res.compile() {
        if std::env::var_os("WARQACAD_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=warqacad.exe built without icon/version resources: {e}");
    }
}

//! Embeds every bundled SunSpec model definition (`models/model_<id>.json`), so adding
//! a model only needs its file.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let models =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir")).join("models");
    println!("cargo:rerun-if-changed={}", models.display());

    let mut files: Vec<String> = fs::read_dir(&models)
        .expect("read models directory")
        .map(|entry| {
            entry
                .expect("models directory entry")
                .file_name()
                .into_string()
                .expect("UTF-8 file name")
        })
        .filter(|name| name.starts_with("model_") && name.ends_with(".json"))
        .collect();
    files.sort();

    let mut sources = String::from("&[\n");
    for name in &files {
        let path = models.join(name);
        println!("cargo:rerun-if-changed={}", path.display());
        sources.push_str(&format!(
            "    include_str!({:?}),\n",
            path.display().to_string()
        ));
    }
    sources.push_str("]\n");

    let out = PathBuf::from(env::var("OUT_DIR").expect("out dir")).join("model_sources.rs");
    fs::write(out, sources).expect("write model_sources.rs");
}

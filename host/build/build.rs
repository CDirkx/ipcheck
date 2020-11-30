mod rust;
mod dotnet;

fn main() {
    if !std::path::Path::new("../artifacts").exists() {
        std::fs::create_dir("../artifacts").expect("failed to create artifacts dir");
    }

    let mut impls = std::fs::File::create("../artifacts/.impls").expect("failed to create .impls file");

    output_impl(&mut impls, "Rust", rust::build());
    output_impl(&mut impls, ".NET", dotnet::build());

    rerun_if_changed("build");
    rerun_if_changed("../impls");
}

fn output_impl(file: &mut std::fs::File, lang: &str, result: std::io::Result<()>) {
    use std::io::Write;

    match result {
        Ok(()) => { writeln!(file, "{}", lang).expect("failed to write .impls file") }
        Err(err) => {
            println!("cargo:warning=Failed to build {} impl: {}", lang, err)
        }
    }
}

fn rerun_if_changed(dir: &str) {
    for entry in walkdir::WalkDir::new(dir) {
        if let Ok(entry) = entry {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
}
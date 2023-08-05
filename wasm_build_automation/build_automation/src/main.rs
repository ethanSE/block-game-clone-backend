use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

fn main() {
    build_pkg();
    generate_ts_types();
    generate_index_file_for_ts_types();
    add_types_to_package_json();
}

fn build_pkg() {
    let output = Command::new("wasm-pack")
        .current_dir("../..")
        .args(["build", "--target", "web"])
        .output()
        .expect("failed to build with wasm-pack");
    println!("{:?}", output)
}

fn generate_ts_types() {
    let output = Command::new("cargo")
        .current_dir("../..")
        .args(["test", "-q"])
        .output()
        .expect("failed to build with wasm-pack");
    println!("{:?}", output)
}

fn generate_index_file_for_ts_types() {
    println!("generating index file");
    let exports: Vec<_> = fs::read_dir("../../pkg/types")
        .expect("failed to open pkg/types")
        .filter_map(Result::ok)
        .filter_map(|p| {
            p.path()
                .file_stem()
                .and_then(OsStr::to_str)
                .map(str::to_owned)
        })
        .filter(|f| f != "index")
        .map(|f| format!("export * from \"./{}\"", f))
        .collect();

    let mut file =
        File::create("../../pkg/types/index.ts").expect("failed to create /pkg/types/index.ts");
    file.write_all(exports.join("\n").as_bytes())
        .expect("failed to write to /pkg/types/index.ts");
}

fn add_types_to_package_json() {
    let path = Path::new("../../pkg/package.json");
    let contents = fs::read(path).expect("failed to read to bytes vec");

    let mut pkg_json: serde_json::Value = serde_json::from_slice(&contents)
        .expect("failed to read package.json bytes to serde_json::Value");

    pkg_json
        .get_mut("files")
        .expect("failed to get files in Value")
        .as_array_mut()
        .unwrap()
        .push(serde_json::Value::String("types".to_string()));

    fs::write(
        path,
        serde_json::to_string_pretty(&pkg_json).unwrap().as_bytes(),
    )
    .expect("failed to write to package.json")
}

use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    panic,
    path::Path,
    process::Command,
};

const BUILD_PROJECT: &str = "./build_wasm_ts_module";
const GAME_LOGIC: &str = "./game_logic";

fn main() {
    let result = panic::catch_unwind(|| {
        build_pkg();
        generate_ts_types();
        generate_index_file_for_ts_types();
        add_types_to_package_json();
        move_dir_to_build_wasm_bundle_project();
    });

    match result {
        Ok(_) => {
            println!(
                "✅ Build automation suceeded! Package available at: {}/pkg",
                BUILD_PROJECT
            )
        }
        Err(e) => println!("{:?}", e),
    }
}

fn build_pkg() {
    Command::new("wasm-pack")
        .current_dir(BUILD_PROJECT)
        .args(["build", "--target", "web"])
        .output()
        .expect("failed to build with wasm-pack");
}

fn generate_ts_types() {
    Command::new("cargo")
        .current_dir(GAME_LOGIC)
        .args(["test", "-q"])
        .output()
        .expect("failed to test / generate ts types");
}

fn generate_index_file_for_ts_types() {
    println!("generating index file");

    let exports: Vec<_> = fs::read_dir(GAME_LOGIC.to_string() + "/pkg/types")
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

    let mut file = File::create(GAME_LOGIC.to_string() + "/pkg/types/index.ts")
        .expect("failed to create /pkg/types/index.ts");
    file.write_all(exports.join("\n").as_bytes())
        .expect("failed to write to /pkg/types/index.ts");
}

fn add_types_to_package_json() {
    let a = format!("{}/pkg/package.json", BUILD_PROJECT);
    let path = Path::new(&a);
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

fn move_dir_to_build_wasm_bundle_project() {
    if let Ok(true) = fs::exists(BUILD_PROJECT.to_string() + "/pkg/types") {
        fs::remove_dir_all(BUILD_PROJECT.to_string() + "/pkg/types")
            .expect(format!("failed to delete existing dir:  {}/pkg", BUILD_PROJECT).as_str());
    }
    fs::rename(
        GAME_LOGIC.to_string() + "/pkg/types",
        BUILD_PROJECT.to_string() + "/pkg/types",
    )
    .expect("failed to move pkg directory")
}

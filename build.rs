use std::{
    env::{self, var},
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let dir = var("CARGO_MANIFEST_DIR").unwrap();
    //println!("cargo:rustc-link-lib=dylib=AiNav");
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("lib").display()
    );

    copy_dynamic_libraries();
}

fn find_cargo_target_dir() -> PathBuf {
    // Infer the top level cargo target dir from the OUT_DIR by searching
    // upwards until we get to $CARGO_TARGET_DIR/build/ (which is always one
    // level up from the deepest directory containing our package name)
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let mut out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    loop {
        {
            let final_path_segment = out_dir.file_name().unwrap();
            if final_path_segment.to_string_lossy().contains(&pkg_name) {
                break;
            }
        }
        if !out_dir.pop() {
            panic!("Malformed build path: {}", out_dir.to_string_lossy());
        }
    }
    out_dir.pop();
    out_dir.pop();
    out_dir
}

fn copy_library_file(src_path: &Path, target_path: &Path) {
    // Copy the shared libs to:
    //  * target dir: as a product ship product of the build,
    //  * deps directory: as comment example testing doesn't pick up the library search path
    //    otherwise and fails.
    let deps_path = target_path.join("deps");
    for path in &[target_path, &deps_path] {
        let dst_path = path.join(src_path.file_name().expect("Path missing filename"));

        fs::copy(&src_path, &dst_path).expect(&format!(
            "Failed to copy dynamic library from {} to {}",
            src_path.to_string_lossy(),
            dst_path.to_string_lossy()
        ));
    }
}

fn copy_dynamic_libraries() {
    let src_path = Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("lib");
    let target_path = find_cargo_target_dir();

    for entry in std::fs::read_dir(&src_path).unwrap_or_else(|_| panic!("Couldn't readdir lib")) {
        let entry = entry.expect("Error looking at lib dir");
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_file() {
                copy_library_file(&entry.path(), &target_path)
            }
        }
    }
}

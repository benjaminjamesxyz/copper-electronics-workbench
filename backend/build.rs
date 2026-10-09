use std::{env, fs, path::PathBuf};
fn main() {
    println!("cargo:rerun-if-changed=catalog");
    let mut files: Vec<_> = fs::read_dir("catalog").into_iter().flatten().flatten()
        .map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "json")).collect();
    files.sort();
    let mut code = String::from("pub const CATALOGS: &[&str] = &[\n");
    for path in files {
        let path = fs::canonicalize(path).expect("catalog path");
        code.push_str(&format!("include_str!({:?}),\n", path));
    }
    code.push_str("];\n");
    fs::write(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("catalogs.rs"), code).unwrap();
}

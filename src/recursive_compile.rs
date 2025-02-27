use crate::ignore_paths::IgnorePaths;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct CompilationOptions {
    pub compiler: String,
    pub ar: String,
    pub optimization_level: String,
}

pub fn compile_to_static_libs<const SIZE: usize>(lib_dir_path: &str, output_dir_path: &str,
                                                 compilation_options: &CompilationOptions,
                                                 ignore_paths: &IgnorePaths<SIZE>) {
    let module_name: String = Path::new(lib_dir_path)
        .file_name()
        .expect(&format!(
            "Can't fetch library name form path {}",
            lib_dir_path
        ))
        .to_str()
        .expect("Can't convert library name to str")
        .to_lowercase();
    let module_name: &str = module_name.as_str();

    let output_dir_path: String = format!("{output_dir_path}/{module_name}");
    let output_dir_path: &str = output_dir_path.as_str();

    println!("module_name: {:?}", module_name);
    println!("module_dir_path: {:?}", lib_dir_path);
    println!("output_dir_path: {:?}", output_dir_path);
    println!();

    println!("Creating output directory: {:?}", output_dir_path);
    fs::create_dir_all(output_dir_path).expect(&format!(
        "Can't create output directory {}",
        output_dir_path
    ));

    let children: fs::ReadDir = fs::read_dir(lib_dir_path)
        .expect(&format!("Can't read library directory {}", lib_dir_path));

    let headers_recursive = children
        .map(|entry| entry.expect("Could not read dir entry").path())
        .filter(|entry| (entry.is_dir() || entry.extension().unwrap_or_default() == "c") && !ignore_paths.is_ignored(entry.as_path().to_str().unwrap()));

    for child_path in headers_recursive {
        if child_path.is_dir() {
            let child_path: &str = child_path
                .to_str()
                .expect("Can't convert child path to str");

            compile_to_static_libs(
                child_path,
                &format!("{output_dir_path}").as_str(),
                compilation_options,
                ignore_paths
            );

            continue;
        }

        let file_stem: &str = child_path
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .expect(&format!("Can't get stem from child file {:?}", child_path));

        let file_path = child_path
            .as_path()
            .to_str()
            .expect(&format!("Can't get path from child file {:?}", child_path));

        println!("file_path: {:?}", file_path);
        println!("file_stem: {:?}", file_stem);

        println!("Compile to {output_dir_path}/{file_stem}.o");

        Command::new(&compilation_options.compiler)
            .args(&["-c", file_path, "-fPIC", "-o", &format!("{output_dir_path}/{file_stem}.o"), "-Wall", "-Wformat=0"])
            .status()
            .unwrap();

        Command::new(&compilation_options.ar)
            .args(&["rcs", &format!("{output_dir_path}/lib{file_stem}.a"), &format!("{output_dir_path}/{file_stem}.o")])
            .status()
            .unwrap();
    }
}


use bindgen::Bindings;
use std::fs;
use std::io::Write;
use std::path::Path;

/**
* Recursively generates C module bindings for Rust and returns module name.
*/
pub fn generate_module_bindings(module_dir_path: &str, output_dir_path: &str) -> String {
    let module_name: String = Path::new(module_dir_path)
        .file_name()
        .expect(&format!(
            "Can't fetch library name form path {}",
            module_dir_path
        ))
        .to_str()
        .expect("Can't convert library name to str")
        .to_lowercase();
    let module_name: &str = module_name.as_str();

    let output_dir_path: String = format!("{output_dir_path}/{module_name}");
    let output_dir_path: &str = output_dir_path.as_str();

    let module_file_path: String = format!("{output_dir_path}.rs");
    let module_file_path: &str = module_file_path.as_str();

    println!("module_name: {:?}", module_name);
    println!("module_dir_path: {:?}", module_dir_path);
    println!("output_dir_path: {:?}", output_dir_path);
    println!();

    println!("Creating output directory: {:?}", output_dir_path);
    fs::create_dir_all(output_dir_path).expect(&format!(
        "Can't create output directory {}",
        output_dir_path
    ));

    println!("Creating module file: {:?}", module_file_path);
    println!();

    let mut module_file: fs::File = fs::File::create(module_file_path)
        .expect(&format!("Can't create module file {}", module_file_path));

    let children: fs::ReadDir = fs::read_dir(module_dir_path)
        .expect(&format!("Can't read library directory {}", module_dir_path));

    let headers_recursive = children
        .map(|entry| entry.expect("Could not read dir entry").path())
        .filter(|entry| entry.is_dir() || entry.extension().unwrap_or_default() == "h");

    for child_path in headers_recursive {
        if child_path.is_dir() {
            let child_path: &str = child_path
                .to_str()
                .expect("Can't convert child path to str");

            let child_module_name: String =
                generate_module_bindings(child_path, &format!("{output_dir_path}").as_str());

            import_module(&mut module_file, &module_name, &child_module_name);
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

        let result: Bindings = bindgen::Builder::default()
            .header(file_path)
            .generate()
            .expect(&format!(
                "Can't generate bindings for library file {file_path}"
            ));

        let output_file_path: String = format!("{output_dir_path}/{file_stem}.rs");
        println!("output_file_path: {:?}", output_file_path);
        println!();

        import_module(&mut module_file, module_name, file_stem);

        result.write_to_file(&output_file_path).expect(&format!(
            "Can't write bindings to file {}",
            output_file_path
        ));
    }

    module_name.to_string()
}

fn import_module(module_file: &mut fs::File, parent_module_name: &str, module_name: &str) {
    let child_module_import_row: String = format!(
        r#"
#[path = "{parent_module_name}/{module_name}.rs"]
pub(crate) mod {module_name};
"#
    );

    module_file
        .write_all(child_module_import_row.as_bytes())
        .expect("Can't add child module to module file.");
}

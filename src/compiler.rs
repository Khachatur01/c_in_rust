use std::env::set_var;
use std::fs;
use std::path::Path;

pub struct CompilationOptions {
    pub target: String,
    pub compiler: String,
    pub ar: String,
    pub optimization_level: String,
    pub host_target: String,
}

pub fn compile_to_static_libs(
    lib_dir_path: &str,
    output_dir_path: &str,
    compilation_options: &CompilationOptions,
) {
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

    set_var("OUT_DIR", output_dir_path);
    set_var("TARGET", &compilation_options.target);
    set_var(
        format!("CC_{}", &compilation_options.target),
        &compilation_options.compiler,
    );
    set_var(
        format!("AR_{}", &compilation_options.target),
        &compilation_options.ar,
    );
    set_var("CRATE_CC_NO_DEFAULTS", "true");
    set_var("OPT_LEVEL", &compilation_options.optimization_level);
    set_var("HOST", &compilation_options.host_target);

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
        .filter(|entry| entry.is_dir() || entry.extension().unwrap_or_default() == "c");

    for child_path in headers_recursive {
        if child_path.is_dir() {
            let child_path: &str = child_path
                .to_str()
                .expect("Can't convert child path to str");

            compile_to_static_libs(
                child_path,
                &format!("{output_dir_path}").as_str(),
                compilation_options,
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

        println!("Compile to {output_dir_path}/{file_stem}");

        cc::Build::new()
            .file(file_path)
            .out_dir(output_dir_path)
            .compile(file_stem);
    }
}

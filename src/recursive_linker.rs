use std::fs;

#[derive(Debug)]
pub struct StaticLibrary {
    pub search_path: String,
    pub static_libraries: Vec<String>,
}

pub fn get_static_libraries(lib_dir_path: &str) -> Vec<StaticLibrary> {
    let mut static_library: StaticLibrary = StaticLibrary {
        search_path: lib_dir_path.to_string(),
        static_libraries: vec![],
    };
    let mut static_libraries: Vec<StaticLibrary> = vec![];

    let children: fs::ReadDir = fs::read_dir(lib_dir_path)
        .expect(&format!("Can't read library directory {}", lib_dir_path));

    let static_libs_recursive = children
        .map(|entry| entry.expect("Could not read dir entry").path())
        .filter(|entry| entry.is_dir() || entry.extension().unwrap_or_default() == "a");

    for child_path in static_libs_recursive {
        if child_path.is_dir() {
            let child_path: &str = child_path
                .to_str()
                .expect("Can't convert child path to str");

            let mut child_library = get_static_libraries(child_path);
            static_libraries.append(&mut child_library);

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

        let static_lib_name: String = file_stem
            .strip_prefix("lib")
            .expect(&format!("Can't strip lib prefix from file {:?}", child_path))
            .to_string();

        println!("file_path: {:?}", file_path);
        println!("file_stem: {:?}", file_stem);

        static_library.static_libraries.push(static_lib_name);
    }

    static_libraries.push(static_library);
    static_libraries
}

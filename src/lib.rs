mod recursive_bindgen;
mod recursive_compile;
mod recursive_linker;
pub mod command_line_arguments;
mod ignore_paths;

pub use crate::recursive_bindgen::generate_module_bindings;
pub use crate::recursive_compile::{compile_to_static_libs, CompilationOptions};
pub use crate::recursive_linker::{get_static_libraries};
pub use bindgen::Builder as BindgenBuilder;

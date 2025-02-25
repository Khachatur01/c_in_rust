mod recursive_bindgen;
mod recursive_compile;
pub mod command_line_arguments;

pub use crate::recursive_bindgen::generate_module_bindings;
pub use crate::recursive_compile::{compile_to_static_libs, CompilationOptions};
pub use bindgen::Builder as BindgenBuilder;

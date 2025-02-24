mod bindings_generator;
mod compiler;

pub use crate::bindings_generator::generate_module_bindings;
pub use crate::compiler::{compile_to_static_libs, CompilationOptions};

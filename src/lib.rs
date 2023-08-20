//! SSA generation library.

mod ssa;
pub use ssa::builder::ModuleBuilder;
pub use ssa::module::Module;
pub mod vcode;

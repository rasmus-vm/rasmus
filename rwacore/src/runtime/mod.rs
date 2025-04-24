//! Rasmus WebAssembly Runtime

pub mod exec_res;
pub mod values;
pub mod trap;
pub mod stack;
pub mod store;
pub mod instances;

mod frame;

pub use frame::*;
pub use exec_res::*;

mod numeric;
mod reference;
mod parametric;
mod variable;
mod table;
mod memory;
mod control;

pub use control::*;
pub use memory::*;
pub use table::*;
pub use variable::*;
pub use parametric::*;
pub use numeric::*;
pub use reference::*;

pub enum Instr {}

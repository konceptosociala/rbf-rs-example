pub mod logger;
pub mod macros;
pub mod traits;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size(pub u32, pub u32);
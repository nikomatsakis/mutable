#![feature(marker_trait_attr)]

mod pure_clone;
mod mutbl;

pub use pure_clone::PureClone;
pub use mutbl::Mut;

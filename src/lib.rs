#![feature(marker_trait_attr)]

mod pure_clone;
mod mutbl;
mod mutbl_vec;

pub use pure_clone::PureClone;
pub use mutbl::Mut;

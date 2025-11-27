mod boxplot;
mod contour;
mod histogram;
mod kde;
mod linear_system;

pub use boxplot::boxplot;
pub use contour::contour;
pub use histogram::histogram;
pub use kde::{kde, KdeResult};
pub use linear_system::thomas_algorithm;

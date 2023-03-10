/// Shorthand for `mod module; pub use module::*;`.
#[macro_export]
macro_rules! glob_export {
    ($module: ident) => {
        mod $module;
        pub use $module::*;
    };
}

#[cfg(test)]
mod test;

mod error;
mod extensions;
mod traits;
mod vector;

pub use error::*;
pub use extensions::*;
pub use traits::*;
pub use vector::*;

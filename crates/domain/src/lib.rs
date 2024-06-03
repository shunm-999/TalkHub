#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(async_fn_traits)]

pub mod crates;
pub mod error;
pub mod error_type;
pub mod input_data;
pub mod result;
pub mod usecase;

#[cfg(test)]
mod tests {
    use super::*;
}

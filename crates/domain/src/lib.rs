#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(async_fn_traits)]

pub mod crates;
pub mod error;
pub mod input_data;
pub mod usecase;
pub mod result;
pub mod error_type;

#[cfg(test)]
mod tests {
    use super::*;
}

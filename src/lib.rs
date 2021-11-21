//! # Modified
//! Simple library which allows you to track changes in your variables.
//! ```
//! use modified::Modified;
//! 
//! let mut var = Modified::new(15);
//! *var = 20;
//! // Or
//! // var.set(20);
//! assert!(var.is_modified());
//! ```
mod reset;
pub use reset::*;

mod modify;
pub use modify::*;

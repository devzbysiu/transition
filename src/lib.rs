#![deny(missing_docs)]

//! This crate allows to programmatically control LED light - blink(1).
//! The purpose is to be able to show the status of your task by "wrapping" all it's
//! execution between calls to this library.
//!
//! While your code is executing, the LED transition between two specified colors
//! indicating "pending" state. After your code finishes execution, you can notify the
//! LED about the status of your task, changing it's light to one of two
//! colors - depending on the status of your code.
//!
//! # Example
//!
//! ```rust
//! use transition::Transition;
//! use std::error::Error;
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let notifier = Transition::new(&["blue", "white"]) // pending state
//!         .on_success("green")
//!         .on_failure("red")
//!         .start()?;
//!
//!     // your code here, e.g.:
//!     thread::sleep(Duration::from_secs(2));
//!
//!     notifier.notify_success(); // or notifier.notify_failure();
//!     // LED changes color to the one specified for success
//!     Ok(())
//! }
//! ```

#[cfg(test)]
mod testutils;

mod error;
mod msg;
mod notifier;
mod task;
mod transition;

pub use crate::transition::Transition;
pub use error::TransitionErr;
pub use notifier::Notifier;

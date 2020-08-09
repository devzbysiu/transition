#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/transition/0.1.0")]

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
//! use transition::{Transition, Led};
//! use std::error::Error;
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let notifier = Transition::new(&[Led::Blue, Led::Blank]) // pending state
//!         .on_success(&Led::Green)
//!         .on_failure(&Led::Red)
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

mod color;
mod error;
mod msg;
mod notifier;
mod task;
mod transition;

pub use crate::color::Led;
pub use crate::transition::Transition;
use doc_comment::doctest;
pub use error::TransitionErr;
pub use notifier::Notifier;

doctest!("../README.md");

#[cfg(test)]
mod test {
    use crate::testutils::utils::init_logging;
    use crate::Transition;
    use crate::TransitionErr;
    use log::debug;
    use std::time::Duration;

    #[test]
    fn test_clone_of_transition() -> Result<(), TransitionErr> {
        init_logging();
        let transition = Transition::default();
        let other_transition = transition.clone();
        let notifier = transition.start()?;
        std::thread::sleep(Duration::from_millis(1000));
        notifier.notify_failure()?;

        let notifier = other_transition.start()?;
        std::thread::sleep(Duration::from_millis(1000));
        notifier.notify_success()?;

        Ok(())
    }

    #[test]
    fn test_debug_of_transition() -> Result<(), TransitionErr> {
        init_logging();
        let transition = Transition::default();
        debug!("testing Debug of transition: {:#?}", transition);

        Ok(())
    }

    #[test]
    fn test_debug_of_notifier() -> Result<(), TransitionErr> {
        init_logging();
        let notifier = Transition::default().start()?;
        debug!("testing Debug of notifier: {:#?}", notifier);

        Ok(())
    }
}

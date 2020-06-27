use crate::error::TransitionErr;
use crossbeam_channel::Sender;
use log::debug;
use std::thread::JoinHandle;

/// Allows to control blinking of the LED after the transition starts.
///
/// After you starts the blinking via [start()](./transition/struct.Transition.html#method.start),
/// you can end the blinking process via this struct.
///
/// # Example:
///
/// ```
/// use transition::{Transition, Notifier};
/// use std::{error::Error, time::Duration, thread};
///
/// let notifier: Notifier = Transition::new(&["blue", "white"]).start()?;
/// // blinks using colors blue - white interchangeably
/// thread::sleep(Duration::from_secs(1));
/// notifier.notify_failure();
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub struct Notifier {
    sender: Sender<MsgType>,
    handle: JoinHandle<Result<(), TransitionErr>>,
}

impl Notifier {
    pub(crate) fn new(
        sender: Sender<MsgType>,
        handle: JoinHandle<Result<(), TransitionErr>>,
    ) -> Self {
        Self { sender, handle }
    }

    /// Finishes the transition with success.
    ///
    /// Changes the color of the LED, to the one set with
    /// [on_success](./transition/struct.Transition.html#method.on_success). If not set, the
    /// default is set to *green*.
    ///
    /// Stops the thread which is responsible for blinking of the LED.
    pub fn notify_success(self) -> Result<(), TransitionErr> {
        debug!("notifying about success");
        self.sender.send(MsgType::Success)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }

    /// Finishes the transition with failure.
    ///
    /// Changes the color of the LED, to the one set with
    /// [on_failure](./transition/struct.Transition.html#method.on_failure). If not set, the
    /// default is set to *red*.
    ///
    /// Stops the thread which is responsible for blinking of the LED.
    pub fn notify_failure(self) -> Result<(), TransitionErr> {
        debug!("notifying about failure");
        self.sender.send(MsgType::Failure)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }
}

/// Messages interchanged between main thread and the thread which is responsible for blinking the
/// LED.
#[derive(Debug)]
pub enum MsgType {
    /// Send when [notify_success](./transition/struct.Transition.html#method.notify_success) is
    /// called.
    Success,

    /// Send when [notify_failure](./transition/struct.Transition.html#method.notify_failure) is
    /// called.
    Failure,
}

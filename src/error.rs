use crate::notifier::MsgType;
use thiserror::Error;

/// Error descriping issue with the transition.
#[derive(Debug, Error)]
pub enum TransitionErr {
    // #[error("error while executing task in transition")]
    // TaskExecution,
    /// Describes issue with the connection to the blink(1) device.
    ///
    /// Make sure that your blink is connected and that your user have correct rights to access
    /// blink device.
    #[error("cannot contact blink(1) device")]
    BlinkConnection(#[from] blinkrs::BlinkError),

    /// Describes issue with sending a message via a crossbeam_channel to inform blinking thread to
    /// stop execution.
    #[error("cannot notify second thread")]
    Notification(#[from] crossbeam_channel::SendError<MsgType>),
}

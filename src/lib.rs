use thiserror::Error;

mod msg;
pub mod notifier;
mod task;
pub mod transition;

#[cfg(test)]
mod testutils;

#[derive(Debug, Error)]
pub enum TransitionError {
    #[error("error while executing task in transition")]
    TaskExecution,

    #[error("cannot contact blink(1) device")]
    BlinkConnection(#[from] blinkrs::BlinkError),

    #[error("cannot notify second thread")]
    Notification(#[from] crossbeam_channel::SendError<MsgType>),
}

#[derive(Debug)]
pub enum MsgType {
    Success,
    Failure,
}

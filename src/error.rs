use crate::notifier::MsgType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransitionErr {
    #[error("error while executing task in transition")]
    TaskExecution,

    #[error("cannot contact blink(1) device")]
    BlinkConnection(#[from] blinkrs::BlinkError),

    #[error("cannot notify second thread")]
    Notification(#[from] crossbeam_channel::SendError<MsgType>),
}

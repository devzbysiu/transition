use crossbeam_channel::Sender;
use log::debug;
use std::thread::JoinHandle;
use thiserror::Error;

mod msg;
mod task;
mod transition;

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

pub struct Transmitter {
    sender: Sender<MsgType>,
    handle: JoinHandle<Result<(), TransitionError>>,
}

impl Transmitter {
    pub fn notify_success(self) -> Result<(), TransitionError> {
        debug!("notifying about success");
        self.sender.send(MsgType::Success)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }

    pub fn notify_failure(self) -> Result<(), TransitionError> {
        debug!("notifying about failure");
        self.sender.send(MsgType::Failure)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum MsgType {
    Success,
    Failure,
}

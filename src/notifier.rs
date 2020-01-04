use crate::MsgType;
use crate::TransitionError;
use crossbeam_channel::Sender;
use log::debug;
use std::thread::JoinHandle;

pub struct Notifier {
    sender: Sender<MsgType>,
    handle: JoinHandle<Result<(), TransitionError>>,
}

impl Notifier {
    pub(crate) fn new(
        sender: Sender<MsgType>,
        handle: JoinHandle<Result<(), TransitionError>>,
    ) -> Self {
        Self { sender, handle }
    }

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

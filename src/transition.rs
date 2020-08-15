use crate::color::Led;
use crate::error::TransitionErr;
use crate::msg::ColorMessage;
use crate::msg::Message;
use crate::notifier::MsgType;
use crate::notifier::Notifier;
use crate::task::BlinkTask;
use crate::task::Task;
use crossbeam_channel::unbounded;
use log::debug;
use log::info;
use std::sync::Arc;
use std::thread;

/// Main structure. Represents colors of task state (pending, successfull, failed). Allows to start the transition.
#[derive(Debug, Clone)]
pub struct Transition {
    task: Arc<dyn Task>,
    failure_msg: Arc<dyn Message>,
    success_msg: Arc<dyn Message>,
}

impl Transition {
    /// Creates new instance of `Transition` with specified colors for "pending" state.
    ///
    /// Passed colors will be used between calls of
    /// [`start`](struct.Transition.html#method.start) and
    /// [`notify_success`](../struct.Notifier.html#method.notify_success) (or
    /// [`notify_failure`](../struct.Notifier.html#method.notify_failure)) to visualise pendig task
    /// execution.
    /// The failure color is set to *red* and the success color is set to *green*. You can override
    /// success and failure colors using [`on_success`](struct.Transition.html#method.on_success) and
    /// [`on_failure`](struct.Transition.html#method.on_failure) accordingly.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// use crate::transition::{Transition, Led};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let transition = Transition::new(&[Led::Blue, Led::Blank]);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn new(colors: &[Led]) -> Result<Self, TransitionErr> {
        Ok(Self {
            task: Arc::new(BlinkTask::new(colors)?),
            failure_msg: Arc::new(ColorMessage::new(&Led::Red)),
            success_msg: Arc::new(ColorMessage::new(&Led::Green)),
        })
    }

    /// Starts the transition.
    ///
    /// The transition is started in a separate thread. As a result, you get
    /// [Notifier](../struct.Notifier) struct.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// use crate::transition::{Transition, Led};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let notifier = Transition::new(&[Led::Blue, Led::Blank])?.start()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// If any error related to Blink(1) device will be thrown, this method results with
    /// [`TransitionErr`](enum.TransitionErr.html).
    pub fn start(self) -> Result<Notifier, TransitionErr> {
        debug!("starting transition");
        let (sender, receiver) = unbounded();
        debug!("starting thread with task to execute");
        let handle = thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(MsgType::Success) => break self.send_success_msg(),
                Ok(MsgType::Failure) => break self.send_failure_msg(),
                Err(_) => info!("no message received"),
            };
            self.execute_task_if_present()?;
        });
        Ok(Notifier::new(sender, handle))
    }

    fn send_success_msg(&self) -> Result<(), TransitionErr> {
        self.send_if_present(&MsgType::Success)?;
        Ok(())
    }

    fn send_if_present(&self, msg: &MsgType) -> Result<(), TransitionErr> {
        let message = match msg {
            MsgType::Success => self.success_msg.as_ref(),
            MsgType::Failure => self.failure_msg.as_ref(),
        };
        debug!("sending {:?} message", msg);
        message.send()?;
        Ok(())
    }

    fn send_failure_msg(&self) -> Result<(), TransitionErr> {
        self.send_if_present(&MsgType::Failure)?;
        Ok(())
    }

    fn execute_task_if_present(&self) -> Result<(), TransitionErr> {
        debug!("executing task");
        self.task.execute()?;
        Ok(())
    }

    /// Allows to override success color.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// use crate::transition::{Transition, Led};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let transition = Transition::default().on_success(&Led::Orange);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn on_success(mut self, color: &Led) -> Self {
        self.success_msg = Arc::new(ColorMessage::new(color));
        self
    }

    /// Allows to override failure color.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// use crate::transition::{Transition, Led};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let transition = Transition::default().on_failure(&Led::Cyan);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn on_failure(mut self, color: &Led) -> Self {
        self.failure_msg = Arc::new(ColorMessage::new(color));
        self
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            task: Arc::new(
                BlinkTask::new(&[Led::Blue, Led::Blank]).expect("failed to create blink task"),
            ),
            failure_msg: Arc::new(ColorMessage::new(&Led::Red)),
            success_msg: Arc::new(ColorMessage::new(&Led::Green)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::TransitionErr;
    use crate::testutils::utils::init_logging;
    use crate::testutils::utils::MessageSpy;
    use crate::testutils::utils::TaskSpy;
    use crate::transition::Transition;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_task_not_executed_when_transition_not_started() -> Result<(), TransitionErr> {
        init_logging();
        let (_, task, _, _) = transition_with_spies();

        assert_eq!(false, task.executed(), "Test task was executed");
        Ok(())
    }

    #[test]
    fn test_task_was_executed_after_transition_start() -> Result<(), TransitionErr> {
        init_logging();
        let (transition, task, _, _) = transition_with_spies();

        transition.start()?;
        std::thread::sleep(Duration::from_millis(1000)); // allow transition to execute

        assert_eq!(true, task.executed(), "Test task was executed");
        Ok(())
    }

    #[test]
    fn test_failure_msg_was_sent_when_failure_notified() -> Result<(), TransitionErr> {
        init_logging();
        let (transition, task, failure_msg, success_msg) = transition_with_spies();

        let tx = transition.start()?;
        std::thread::sleep(Duration::from_millis(1000)); // allow transition to execute
        tx.notify_failure()?;

        assert_eq!(true, task.executed(), "Test task was executed");
        assert_eq!(true, failure_msg.msg_sent(), "Test failure WAS sent");
        assert_eq!(false, success_msg.msg_sent(), "Test success NOT sent");
        Ok(())
    }

    #[test]
    fn test_success_msg_was_sent_when_success_notified() -> Result<(), TransitionErr> {
        init_logging();
        let (transition, task, failure_msg, success_msg) = transition_with_spies();

        let tx = transition.start()?;
        std::thread::sleep(Duration::from_millis(1000)); // allow transition to execute
        tx.notify_success()?;

        assert_eq!(true, task.executed(), "Test task was executed");
        assert_eq!(false, failure_msg.msg_sent(), "Test failure NOT sent");
        assert_eq!(true, success_msg.msg_sent(), "Test success WAS sent");
        Ok(())
    }

    fn transition_with_spies() -> (Transition, Arc<TaskSpy>, Arc<MessageSpy>, Arc<MessageSpy>) {
        let task = Arc::new(TaskSpy::new());
        let failure_msg = Arc::new(MessageSpy::new());
        let success_msg = Arc::new(MessageSpy::new());
        let transition = Transition {
            task: task.clone(),
            failure_msg: failure_msg.clone(),
            success_msg: success_msg.clone(),
        };
        (transition, task, failure_msg, success_msg)
    }
}

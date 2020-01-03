use crate::msg::Message;
use crate::task::Task;
use anyhow::Result;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use log::debug;
use log::info;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

mod msg;
mod task;

#[cfg(test)]
mod testutils;

pub struct Transition {
    task: Arc<dyn Task>,
    failure_msg: Arc<dyn Message>,
    success_msg: Arc<dyn Message>,
}

impl Transition {
    pub fn new<A: AsRef<str>>(colors: &[A]) -> Self {
        Self {
            task: Arc::new(task::Simple::new(colors)),
            failure_msg: Arc::new(msg::Simple::new("red")),
            success_msg: Arc::new(msg::Simple::new("green")),
        }
    }

    pub fn start(self) -> Result<Transmitter> {
        debug!("starting transition");
        let (sender, receiver) = unbounded();
        debug!("starting thread with task to execute");
        let handle = thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(Msg::Success) => break self.send_success_msg(),
                Ok(Msg::Failure) => break self.send_failure_msg(),
                Err(_) => info!("no message received"),
            };
            self.execute_task_if_present()?;
        });
        Ok(Transmitter { sender, handle })
    }

    fn send_success_msg(&self) -> Result<()> {
        self.send_if_present(&Msg::Success)?;
        Ok(())
    }

    fn send_if_present(&self, msg: &Msg) -> Result<()> {
        let message = match msg {
            Msg::Success => self.success_msg.as_ref(),
            Msg::Failure => self.failure_msg.as_ref(),
        };
        debug!("sending {:?} message", msg);
        message.send()?;
        Ok(())
    }

    fn send_failure_msg(&self) -> Result<()> {
        self.send_if_present(&Msg::Failure)?;
        Ok(())
    }

    fn execute_task_if_present(&self) -> Result<()> {
        debug!("executing task");
        self.task.execute()?;
        Ok(())
    }

    #[must_use]
    pub fn on_success(mut self, color: &str) -> Self {
        self.success_msg = Arc::new(msg::Simple::new(color));
        self
    }

    #[must_use]
    pub fn on_failure(mut self, color: &str) -> Self {
        self.failure_msg = Arc::new(msg::Simple::new(color));
        self
    }
}

#[derive(Debug)]
enum Msg {
    Success,
    Failure,
}

pub struct Transmitter {
    sender: Sender<Msg>,
    handle: JoinHandle<Result<()>>,
}

impl Transmitter {
    pub fn notify_success(self) -> Result<()> {
        debug!("notifying about success");
        self.sender.send(Msg::Success)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }

    pub fn notify_failure(self) -> Result<()> {
        debug!("notifying about failure");
        self.sender.send(Msg::Failure)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use testutils::utils::MessageSpy;
    use testutils::utils::TaskSpy;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_task_not_executed_when_transition_not_started() -> Result<()> {
        init_logging();
        let (_, task, _, _) = transition_with_spies();

        assert_eq!(false, task.executed(), "Test task was executed");
        Ok(())
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_task_was_executed_after_transition_start() -> Result<()> {
        init_logging();
        let (transition, task, _, _) = transition_with_spies();

        transition.start()?;
        std::thread::sleep(Duration::from_millis(1000)); // allow transition to execute

        assert_eq!(true, task.executed(), "Test task was executed");
        Ok(())
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_failure_msg_was_sent_when_failure_notified() -> Result<()> {
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
    #[allow(non_upper_case_globals)]
    fn test_success_msg_was_sent_when_success_notified() -> Result<()> {
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

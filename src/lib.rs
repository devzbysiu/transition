use crate::messg::Messg;
use crate::task::Task;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use log::debug;
use log::error;
use log::info;
use std::thread;
use std::thread::JoinHandle;

mod messg;
mod task;

#[cfg(test)]
mod testutils;

#[derive(Default)]
pub struct Transition<T: Task + 'static, M: Messg + 'static> {
    task: Option<&'static T>,
    success_msg: Option<&'static M>,
    failure_msg: Option<&'static M>,
}

impl<T: Task + Send + 'static, M: Messg + Send + 'static> Transition<T, M> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            task: None,
            success_msg: None,
            failure_msg: None,
        }
    }

    pub fn start(self) -> Result<Transmitter, failure::Error> {
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

    fn send_success_msg(&self) -> Result<(), failure::Error> {
        self.send_if_present(&Msg::Success)?;
        Ok(())
    }

    fn send_if_present(&self, msg: &Msg) -> Result<(), failure::Error> {
        let message = match msg {
            Msg::Success => self.success_msg,
            Msg::Failure => self.failure_msg,
        };
        if let Some(message) = message {
            debug!("sending {:?} message", msg);
            message.send()?
        } else {
            error!("no {:?} message found", msg);
            panic!("no {:?} message found", msg)
        }
        Ok(())
    }

    fn send_failure_msg(&self) -> Result<(), failure::Error> {
        self.send_if_present(&Msg::Failure)?;
        Ok(())
    }

    fn execute_task_if_present(&self) -> Result<(), failure::Error> {
        if let Some(task) = self.task {
            debug!("executing task");
            task.execute()?;
        } else {
            debug!("no task to execute");
            panic!("no task to execute");
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Msg {
    Success,
    Failure,
}

pub struct Transmitter {
    sender: Sender<Msg>,
    handle: JoinHandle<Result<(), failure::Error>>,
}

impl Transmitter {
    pub fn notify_success(self) -> Result<(), failure::Error> {
        debug!("notifying about success");
        self.sender.send(Msg::Success)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }

    pub fn notify_failure(self) -> Result<(), failure::Error> {
        debug!("notifying about failure");
        self.sender.send(Msg::Failure)?;
        self.handle.join().expect("cannot joing thread")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::time::Duration;
    use testutils::utils::MessageSpy;
    use testutils::utils::TaskSpy;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_task_not_executed_when_transition_not_started() -> Result<(), failure::Error> {
        init_logging();
        lazy_static! {
            static ref task: TaskSpy = TaskSpy::new();
        }
        let _transition: Transition<TaskSpy, MessageSpy> = Transition {
            task: Some(&task),
            failure_msg: None,
            success_msg: None,
        };

        assert_eq!(false, task.executed(), "Test task was executed");
        Ok(())
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_task_was_executed_after_transition_start() -> Result<(), failure::Error> {
        init_logging();
        lazy_static! {
            static ref task: TaskSpy = TaskSpy::new();
        }
        let transition: Transition<TaskSpy, MessageSpy> = Transition {
            task: Some(&task),
            failure_msg: None,
            success_msg: None,
        };

        transition.start()?;
        std::thread::sleep(Duration::from_millis(1000)); // allow transition to execute

        assert_eq!(true, task.executed(), "Test task was executed");
        Ok(())
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_failure_msg_was_sent_when_failure_notified() -> Result<(), failure::Error> {
        init_logging();
        lazy_static! {
            static ref task: TaskSpy = TaskSpy::new();
            static ref failure_msg: MessageSpy = MessageSpy::new();
            static ref success_msg: MessageSpy = MessageSpy::new();
        }
        let transition: Transition<TaskSpy, MessageSpy> = Transition {
            task: Some(&task),
            failure_msg: Some(&failure_msg),
            success_msg: Some(&success_msg),
        };

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
    fn test_success_msg_was_sent_when_success_notified() -> Result<(), failure::Error> {
        init_logging();
        lazy_static! {
            static ref task: TaskSpy = TaskSpy::new();
            static ref failure_msg: MessageSpy = MessageSpy::new();
            static ref success_msg: MessageSpy = MessageSpy::new();
        }
        let transition: Transition<TaskSpy, MessageSpy> = Transition {
            task: Some(&task),
            failure_msg: Some(&failure_msg),
            success_msg: Some(&success_msg),
        };

        let tx = transition.start()?;
        std::thread::sleep(Duration::from_millis(1000)); // allow transition to execute
        tx.notify_success()?;

        assert_eq!(true, task.executed(), "Test task was executed");
        assert_eq!(false, failure_msg.msg_sent(), "Test failure NOT sent");
        assert_eq!(true, success_msg.msg_sent(), "Test success WAS sent");
        Ok(())
    }
}

use crate::messg::Messg;
use crate::task::Simple;
use crate::task::Task;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use lazy_static::lazy_static;
use log::debug;
use log::error;
use log::info;
use std::thread;

mod messg;
mod task;

#[cfg(test)]
mod testutils;

lazy_static! {
    static ref DEFAULT_TASK: Simple = Simple::new(&["blue", "white"]);
}

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
        thread::spawn(move || -> Result<(), failure::Error> {
            loop {
                match receiver.try_recv() {
                    Ok(Msg::Success) => {
                        debug!("received success, breaking with success message");
                        break self.send_success_msg();
                    }
                    Ok(Msg::Failure) => {
                        debug!("received failure, breaking with failure message");
                        break self.send_failure_msg();
                    }
                    Err(_) => info!("no message received"),
                };
                if let Some(task) = self.task {
                    debug!("executing task");
                    task.execute()?;
                } else {
                    debug!("executing default task");
                    DEFAULT_TASK.execute()?;
                }
            }
        });
        Ok(Transmitter { sender })
    }

    fn send_success_msg(&self) -> Result<(), failure::Error> {
        self.send(&Msg::Success)?;
        Ok(())
    }

    fn send(&self, msg: &Msg) -> Result<(), failure::Error> {
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
        self.send(&Msg::Failure)?;
        Ok(())
    }
}

impl<T: Task, M: Messg> Default for Transition<T, M> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum Msg {
    Success,
    Failure,
}

pub struct Transmitter {
    sender: Sender<Msg>,
}

impl Transmitter {
    pub fn notify_success(&self) -> Result<(), failure::Error> {
        debug!("notifying about success");
        self.sender.send(Msg::Success)?;
        Ok(())
    }

    pub fn notify_failure(&self) -> Result<(), failure::Error> {
        debug!("notifying about failure");
        self.sender.send(Msg::Failure)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::time::Duration;
    use testutils::MessageSpy;
    use testutils::TaskSpy;

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

        assert_eq!(false, task.executed());
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
        std::thread::sleep(Duration::from_millis(1)); // allow transition to execute

        assert_eq!(true, task.executed());
        Ok(())
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_failure_msg_was_send_when_failure_notified() -> Result<(), failure::Error> {
        init_logging();
        lazy_static! {
            static ref task: TaskSpy = TaskSpy::new();
            static ref failure_message: MessageSpy = MessageSpy::new();
            static ref success_message: MessageSpy = MessageSpy::new();
        }
        let transition: Transition<TaskSpy, MessageSpy> = Transition {
            task: Some(&task),
            failure_msg: Some(&failure_message),
            success_msg: Some(&success_message),
        };

        let tx = transition.start()?;
        std::thread::sleep(Duration::from_millis(1)); // allow transition to execute
        tx.notify_failure()?;
        std::thread::sleep(Duration::from_millis(1)); // allow message to be sent

        assert_eq!(true, task.executed());
        assert_eq!(true, failure_message.message_sent());
        assert_eq!(false, success_message.message_sent());
        Ok(())
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn test_success_msg_was_send_when_success_notified() -> Result<(), failure::Error> {
        init_logging();
        lazy_static! {
            static ref task: TaskSpy = TaskSpy::new();
            static ref failure_message: MessageSpy = MessageSpy::new();
            static ref success_message: MessageSpy = MessageSpy::new();
        }
        let transition: Transition<TaskSpy, MessageSpy> = Transition {
            task: Some(&task),
            failure_msg: Some(&failure_message),
            success_msg: Some(&success_message),
        };

        let tx = transition.start()?;
        std::thread::sleep(Duration::from_millis(1)); // allow transition to execute
        tx.notify_success()?;
        std::thread::sleep(Duration::from_millis(1)); // allow message to be sent

        assert_eq!(true, task.executed());
        assert_eq!(false, failure_message.message_sent());
        assert_eq!(true, success_message.message_sent());
        Ok(())
    }
}

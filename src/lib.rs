use crate::task::Task;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use log::debug;
use log::info;
use std::thread;
use std::time::Duration;

mod task;

const NOT_IMPORTANT: usize = 0;

pub struct Transition<T: Task + 'static> {
    task: &'static T,
    success_msg: Option<Message>,
    failure_msg: Option<Message>,
}

impl<T: Task + Send + 'static> Transition<T> {
    #[must_use]
    pub fn new(task: &'static T) -> Self {
        Self {
            task,
            success_msg: None,
            failure_msg: None,
        }
    }

    pub fn start(self) -> Result<Transmitter, failure::Error> {
        debug!("starting transition");
        let (sender, receiver) = unbounded();
        debug!("starting thread with task to execute");
        thread::spawn(move || -> Result<usize, failure::Error> {
            loop {
                match receiver.try_recv() {
                    Ok(Msg::Success) => {
                        debug!("received success, breaking with success message");
                        break Self::send_success_msg();
                    }
                    Ok(Msg::Failure) => {
                        debug!("received failure, breaking with failure message");
                        break Self::send_failure_msg();
                    }
                    Err(_) => info!("no message received"),
                };
                debug!("executing a task");
                self.task.execute()?;
            }
        });
        Ok(Transmitter { sender })
    }

    fn send_success_msg() -> Result<usize, failure::Error> {
        // self.blinkers
        // .send(self.success_msg.unwrap_or_else(|| color_msg("green")))?;
        Ok(NOT_IMPORTANT)
    }

    fn send_failure_msg() -> Result<usize, failure::Error> {
        println!("blinking with failure");
        // self.blinkers
        // .send(self.failure_msg.unwrap_or_else(|| color_msg("red")))?;
        Ok(NOT_IMPORTANT)
    }

    #[must_use]
    pub fn on_success(mut self, color_name: &str) -> Self {
        self.success_msg = Some(color_msg(color_name));
        self
    }

    #[must_use]
    pub fn on_failure(mut self, color_name: &str) -> Self {
        self.failure_msg = Some(color_msg(color_name));
        self
    }
}

pub enum Msg {
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

fn color_msg(color_name: &str) -> Message {
    Message::Fade(Color::from(color_name), Duration::from_millis(500))
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

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
        let _transition: Transition<TaskSpy> = Transition::new(&task);

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
        let transition: Transition<TaskSpy> = Transition::new(&task);

        transition.start()?;
        std::thread::sleep(Duration::from_millis(1)); // allow transition to execute

        assert_eq!(true, task.executed());
        Ok(())
    }

    struct TaskSpy {
        task_executed: AtomicBool,
    }

    impl TaskSpy {
        fn new() -> Self {
            Self {
                task_executed: AtomicBool::new(false),
            }
        }

        fn executed(&self) -> bool {
            self.task_executed.load(Ordering::SeqCst)
        }
    }

    impl Task for TaskSpy {
        fn execute(&self) -> Result<(), failure::Error> {
            self.task_executed.store(true, Ordering::SeqCst);
            Ok(())
        }
    }
}

use crate::task::Simple;
use crate::task::Task;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use lazy_static::lazy_static;
use log::debug;
use log::info;
use std::thread;
use std::time::Duration;

mod task;

lazy_static! {
    static ref DEFAULT_TASK: Simple = Simple::new(&["blue", "white"]);
}

const NOT_IMPORTANT: usize = 0;

pub struct Transition<T: Task + 'static> {
    task: Option<&'static T>,
    success_msg: Option<Message>,
    failure_msg: Option<Message>,
}

impl<T: Task + Send + 'static> Transition<T> {
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

    #[allow(dead_code)]
    fn with_task(mut self, task: &'static T) -> Self {
        self.task = Some(task);
        self
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

impl<T: Task> Default for Transition<T> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

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
        let _transition: Transition<TaskSpy> = Transition::new().with_task(&task);

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
        let transition: Transition<TaskSpy> = Transition::new().with_task(&task);

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

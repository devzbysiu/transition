use crate::task::Task;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;

mod task;

const NOT_IMPORTANT: usize = 0;

pub struct Transition<T: Task> {
    task: T,
    success_msg: Option<Message>,
    failure_msg: Option<Message>,
}

impl<T: Task + Send + 'static> Transition<T> {
    #[must_use]
    pub fn new(task: T) -> Self {
        Self {
            task,
            success_msg: None,
            failure_msg: None,
        }
    }

    pub fn start(self) -> Result<Transmitter, failure::Error> {
        let (sender, receiver) = unbounded();
        thread::spawn(move || -> Result<usize, failure::Error> {
            loop {
                match receiver.try_recv() {
                    Ok(Msg::Success) => break Self::send_success_msg(),
                    Ok(Msg::Failure) => break Self::send_failure_msg(),
                    Err(_) => {}
                };
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
        self.sender.send(Msg::Success)?;
        Ok(())
    }

    pub fn notify_failure(&self) -> Result<(), failure::Error> {
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
    use std::cell::RefCell;

    #[test]
    fn test() -> Result<(), failure::Error> {
        let task = TaskSpy::new();
        let transition: Transition<TaskSpy> = Transition::new(task);
        transition.start()?;
        assert_eq!(true, task.task_executed());
        Ok(())
    }

    struct TaskSpy {
        task_executed: RefCell<bool>,
    }

    impl TaskSpy {
        fn new() -> Self {
            Self {
                task_executed: RefCell::new(false),
            }
        }

        fn task_executed(self) -> bool {
            self.task_executed.into_inner()
        }
    }

    impl Task for TaskSpy {
        fn execute(&self) -> Result<(), failure::Error> {
            self.task_executed.replace(false);
            Ok(())
        }
    }
}

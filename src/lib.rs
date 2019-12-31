use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;

pub struct Transition<T: Task, M: Messg> {
    task: T,
    success_msg: M,
    failure_msg: M,
}

impl<T: Task, M: Messg> Transition<T, M> {
    pub fn new(task: T, success_msg: M, failure_msg: M) -> Self {
        Transition {
            task,
            success_msg,
            failure_msg,
        }
    }
}

impl<T: Task, M: Messg> Transition<T, M> {
    pub fn start(self) -> Result<Transmitter, failure::Error> {
        let (sender, receiver) = unbounded();
        let success_msg = self.success_msg;
        let failure_msg = self.failure_msg;
        let task = self.task;
        thread::spawn(move || -> Result<(), failure::Error> {
            loop {
                match receiver.recv() {
                    Ok(Msg::Success) => break success_msg.send(),
                    Ok(Msg::Failure) => break failure_msg.send(),
                    Err(_) => {}
                };
                task.execute()?
            }
        });
        Ok(Transmitter { sender })
    }
}

pub trait Task: Send + 'static {
    fn execute(&self) -> Result<(), failure::Error>;
}

pub struct BlinkTask {
    transition: Vec<Message>,
    blinkers: &'static Blinkers,
}

impl BlinkTask {
    pub fn new(colors: &Vec<&str>, blinkers: &'static Blinkers) -> Self {
        let mut transition = Vec::new();
        for &color_name in colors {
            transition.push(Message::Fade(
                Color::from(color_name),
                Duration::from_millis(500),
            ));
        }
        Self {
            blinkers,
            transition,
        }
    }
}

impl Task for BlinkTask {
    fn execute(&self) -> Result<(), failure::Error> {
        for &message in &self.transition {
            self.blinkers.send(message).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
        Ok(())
    }
}

pub trait Messg: Send + 'static {
    fn send(&self) -> Result<(), failure::Error>;
}

pub(crate) struct SuccessMsg {
    blinkers: &'static Blinkers,
}

impl SuccessMsg {
    fn new(blinkers: &'static Blinkers) -> Self {
        Self { blinkers }
    }
}

impl Messg for SuccessMsg {
    fn send(&self) -> Result<(), failure::Error> {
        self.blinkers.send(color_msg("green"))?;
        Ok(())
    }
}

pub(crate) struct FailureMsg {
    blinkers: &'static Blinkers,
}

impl FailureMsg {
    fn new(blinkers: &'static Blinkers) -> Self {
        Self { blinkers }
    }
}

impl Messg for FailureMsg {
    fn send(&self) -> Result<(), failure::Error> {
        self.blinkers.send(color_msg("red"))?;
        Ok(())
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

    #[test]
    fn test() -> Result<(), failure::Error> {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        let blink_task = BlinkTask::new(&vec!["blue", "white"], &blinkers);
        let failure_msg = FailureMsg::new(&blinkers);
        Ok(())
    }
}

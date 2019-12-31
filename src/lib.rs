use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;

const NOT_IMPORTANT: usize = 0;

pub struct Transition<T: Task> {
    task: T,
    blinkers: Blinkers,
    transition: Vec<Message>,
    success_msg: Option<Message>,
    failure_msg: Option<Message>,
}

impl<T: Task + Send + 'static> Transition<T> {
    #[must_use]
    pub fn new(colors: &[&str], task: T) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        let mut transition = Vec::new();
        for &color_name in colors {
            transition.push(Message::Fade(
                Color::from(color_name),
                Duration::from_millis(500),
            ));
        }
        Self {
            task,
            blinkers,
            transition,
            success_msg: None,
            failure_msg: None,
        }
    }

    pub fn start(self) -> Result<Transmitter, failure::Error> {
        let (sender, receiver) = unbounded();
        let no_transitions = self.transition.len();
        thread::spawn(move || -> Result<usize, failure::Error> {
            loop {
                match receiver.try_recv() {
                    Ok(Msg::Success) => break self.send_success_msg(),
                    Ok(Msg::Failure) => break self.send_failure_msg(),
                    Err(_) => {}
                };
                self.task.execute()?;
                self.play_transition();
            }
        });
        let duration = Duration::from_millis(500 * no_transitions as u64 + 50);
        Ok(Transmitter { sender, duration })
    }

    fn send_success_msg(&self) -> Result<usize, failure::Error> {
        self.blinkers
            .send(self.success_msg.unwrap_or_else(|| color_msg("green")))?;
        Ok(NOT_IMPORTANT)
    }

    fn send_failure_msg(&self) -> Result<usize, failure::Error> {
        println!("blinking with failure");
        self.blinkers
            .send(self.failure_msg.unwrap_or_else(|| color_msg("red")))?;
        Ok(NOT_IMPORTANT)
    }

    fn play_transition(&self) {
        for &message in &self.transition {
            self.blinkers.send(message).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
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

pub trait Task {
    fn execute(&self) -> Result<(), failure::Error>;
}

pub struct SimpleTask;

impl Task for SimpleTask {
    fn execute(&self) -> Result<(), failure::Error> {
        Ok(())
    }
}

pub enum Msg {
    Success,
    Failure,
}

pub struct Transmitter {
    sender: Sender<Msg>,
    duration: Duration,
}

impl Transmitter {
    pub fn notify_success(&self) -> Result<(), failure::Error> {
        self.sender.send(Msg::Success)?;
        thread::sleep(self.duration);
        Ok(())
    }

    pub fn notify_failure(&self) -> Result<(), failure::Error> {
        self.sender.send(Msg::Failure)?;
        thread::sleep(self.duration);
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
        let _transition: Transition<SimpleTask> = Transition::new(&["blue", "white"], SimpleTask);
        Ok(())
    }
}

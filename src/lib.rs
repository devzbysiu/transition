use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;

const NOT_IMPORTANT: usize = 0;

pub struct Transition {
    blinkers: Blinkers,
    transition: Vec<Message>,
    success_msg: Option<Message>,
    failure_msg: Option<Message>,
}

impl From<&str> for Transition {
    fn from(colors: &str) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        let mut transition = Vec::new();
        for color_name in colors.split(' ') {
            transition.push(Message::Fade(
                Color::from(color_name),
                Duration::from_millis(500),
            ));
        }
        Transition {
            blinkers,
            transition,
            success_msg: None,
            failure_msg: None,
        }
    }
}

impl Transition {
    pub fn run(self) -> Result<Transmitter, failure::Error> {
        let (sender, receiver) = unbounded();
        let no_transitions = self.transition.len();
        thread::spawn(move || -> Result<usize, failure::Error> {
            loop {
                match receiver.try_recv() {
                    Ok(Msg::Success) => break self.send_success_msg(),
                    Ok(Msg::Failure) => break self.send_failure_msg(),
                    Err(_) => {}
                };
                self.play_transition();
            }
        });
        let duration = Duration::from_millis(500 * no_transitions as u64 + 50);
        Ok(Transmitter { sender, duration })
    }

    fn send_success_msg(&self) -> Result<usize, failure::Error> {
        self.blinkers
            .send(self.success_msg.unwrap_or(self.color_msg("green")))?;
        Ok(NOT_IMPORTANT)
    }

    fn color_msg(&self, color_name: &str) -> Message {
        Message::Fade(Color::from(color_name), Duration::from_millis(500))
    }

    fn send_failure_msg(&self) -> Result<usize, failure::Error> {
        println!("blinking with failure");
        self.blinkers
            .send(self.failure_msg.unwrap_or(self.color_msg("red")))?;
        Ok(NOT_IMPORTANT)
    }

    fn play_transition(&self) {
        for &message in &self.transition {
            self.blinkers.send(message).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
    }

    pub fn on_success(mut self, color_name: &str) -> Self {
        self.success_msg = Some(self.color_msg(color_name));
        self
    }

    pub fn on_failure(mut self, color_name: &str) -> Self {
        self.failure_msg = Some(self.color_msg(color_name));
        self
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

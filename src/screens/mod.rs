pub mod login;

use iced::Element;
use std::fmt::Debug;

pub trait ScreenMessage: Clone {}

pub trait Screen: Debug {
    fn update(&mut self, msg: impl Clone, debug: &mut bool);
    fn view<M: Clone>(&mut self, debug: bool) -> Element<M>;
}

#[derive(Debug)]
pub struct Screens {
    screens: Vec<Box<dyn Screen>>,
    current: usize,
}

impl Default for Screens {
    fn default() -> Self {
        let screens: Vec<Box<dyn Screen>> = vec![Box::new(login::Login::default())];
        Self {
            screens,
            ..Default::default()
        }
    }
}

impl Screens {
    fn update(&mut self, msg: impl Clone, debug: &mut bool) {
        self.screens[self.current].update(msg, debug);
    }

    fn view(&mut self, debug: bool) -> Element<impl Clone> {
        self.screens[self.current].view(debug)
    }
}

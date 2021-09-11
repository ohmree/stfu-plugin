mod screens;

use iced::{Sandbox, Settings};
use screens::Screens;

#[derive(Debug, Default)]
pub struct Stfu {
    screens: Screens,
}

impl Sandbox for Stfu {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}

pub fn main() -> iced::Result {
    pretty_env_logger::init();
    Stfu::run(Settings::default())
}

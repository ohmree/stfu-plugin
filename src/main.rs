mod screens;

use std::sync::Arc;
use log::*;

use iced::{Application, Clipboard, Command, Settings, Subscription, executor};
use obws::Client;
use screens::{login, // dashboard
};

#[derive(Debug)]
pub enum Screen {
    Login(login::State),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Login(login::State::default())
    }
}

#[derive(Default)]
pub struct Stfu {
    screen: Screen,
    client: Option<Arc<Client>>
}

impl std::fmt::Debug for Stfu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stfu").field("screen", &self.screen).field("client", &"[client]").finish()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Login(login::Message),
    // Dashboard(dashboard::Message),
}

impl Application for Stfu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("STFU")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match &mut self.screen {
            Screen::Login(state) =>  {
                let Message::Login(message) = message;
                debug!("{:?}", message);
                if let login::Message::ButtonPressed = message {
                    let (command, _) = state.update(message);
                    command.map(Message::Login)
                } else {
                    let _ = state.update(message);
                    Command::none()
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if let Some(client) = self.client {
        }
        Subscription::none()
    }

    fn view(&mut self) -> iced::Element<Message> {
        match &mut self.screen {
            Screen::Login(state) => {
                state.view().map(Message::Login)
            }
        }
    }
}

pub fn main() -> iced::Result {
    pretty_env_logger::init();
    Stfu::run(Settings::default())
}

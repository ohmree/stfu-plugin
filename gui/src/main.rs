load_dotenv::load_dotenv!();

mod screens;

use cfg_if::cfg_if;
use directories_next::ProjectDirs;
use iced::{executor, Application, Clipboard, Command, Element, Settings};
use once_cell::sync::Lazy;
use screens::{get_token, login};
use std::borrow::Cow;

static ADDRESS: Lazy<Option<Cow<'static, str>>> = Lazy::new(move || {
    cfg_if! {
        if #[cfg(any(target_os = "windows", target_os = "linux"))] {
            const PREFIX: &str = "@";
        } else {
            const PREFIX: &str = "";
        }
    }

    ProjectDirs::from("org", "OhmRee", "STFU")
        .map(|d| Cow::Owned(format!("{}{}", PREFIX, d.cache_dir().display())))
});

pub fn main() -> iced::Result {
    Stfu::run(Settings::default())
}

#[derive(Clone, Debug)]
pub enum Message {
    Login(login::Message),
    GetToken(get_token::Message),
}

#[derive(Debug)]
pub enum Stfu {
    Login(login::State),
    GetToken(get_token::State),
}

impl Default for Stfu {
    fn default() -> Self {
        Self::Login(login::State::default())
    }
}

impl Application for Stfu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Stfu, Command<Self::Message>) {
        (Stfu::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("STFU")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match self {
            Stfu::Login(state) => {
                if let Message::Login(message) = message {
                    let (event, command) = state.update(message);
                    if let Some(login::Event::LoginPageOpened) = event {
                        *self = Self::GetToken(get_token::State::default());
                    }

                    command.map(Message::Login)
                } else {
                    Command::none()
                }
            }
            Stfu::GetToken(state) => {
                if let Message::GetToken(message) = message {
                    let (event, command) = state.update(message);

                    if let Some(get_token::Event::TokenFetched(token)) = event {
                        dbg!(token);
                    }
                    command.map(Message::GetToken)
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self {
            Self::Login(state) => state.view().map(Message::Login),
            Self::GetToken(state) => state.view().map(Message::GetToken),
        }
    }
}

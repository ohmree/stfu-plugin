use std::sync::Arc;

use iced::{
    button, text_input, Align, Button, Column, Command, Element, HorizontalAlignment, Row, Text,
    TextInput,
};
use obws::Client;

#[derive(Debug, Default)]
pub struct State {
    host: String,
    port: String,
    password: String,
    host_state: text_input::State,
    port_state: text_input::State,
    password_state: text_input::State,
    button_state: button::State,
}

#[derive(Clone)]
pub enum Message {
    HostChanged(String),
    PortChanged(String),
    PasswordChanged(String),
    ButtonPressed,
    ClientConnected(Option<Arc<Client>>),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HostChanged(arg0) => f.debug_tuple("HostChanged").field(arg0).finish(),
            Self::PortChanged(arg0) => f.debug_tuple("PortChanged").field(arg0).finish(),
            Self::PasswordChanged(arg0) => f.debug_tuple("PasswordChanged").field(arg0).finish(),
            Self::ButtonPressed => write!(f, "ButtonPressed"),
            Self::ClientConnected(_) => {
                f.debug_tuple("ClientConnected").field(&"[client]").finish()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    ButtonPressed,
}

impl State {
    async fn create_client(host: String, port: String, password: String) -> anyhow::Result<Client> {
        let client = Client::connect(host, port.parse()?).await?;
        let password = if password.is_empty() {
            None
        } else {
            Some(password)
        };

        client.login(password).await?;
        log::debug!("logged in");
        Ok(client)
    }
    pub fn update(&mut self, message: Message) -> (Command<Message>, Option<Event>) {
        use Message::*;
        match message {
            HostChanged(host) => {
                self.host = host;
                (Command::none(), None)
            }
            PortChanged(port) => {
                self.port = port;
                (Command::none(), None)
            }
            PasswordChanged(password) => {
                self.password = password;
                (Command::none(), None)
            }
            ButtonPressed => (
                Command::perform(
                    // HACK: it might be a good idea to find a better solution than simply jamming all these clones in there.
                    Self::create_client(
                        self.host.clone(),
                        self.port.clone(),
                        self.password.clone(),
                    ),
                    |result| ClientConnected(result.ok().map(Arc::new)),
                ),
                None,
            ),
            ClientConnected(_) => (Command::none(), None),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let host_input = TextInput::new(
            &mut self.host_state,
            "host",
            &self.host,
            Message::HostChanged,
        )
        .padding(10)
        .size(30);

        let port_input = TextInput::new(
            &mut self.port_state,
            "port",
            &self.port,
            Message::PortChanged,
        )
        .padding(10)
        .size(30);

        let password_input = TextInput::new(
            &mut self.password_state,
            "password",
            &self.password,
            Message::PasswordChanged,
        )
        .padding(10)
        .size(30)
        .password();

        let address_row = Row::new().spacing(2).push(host_input).push(port_input);

        let button = Button::new(
            &mut self.button_state,
            Text::new("Login").horizontal_alignment(HorizontalAlignment::Center),
        )
        .on_press(Message::ButtonPressed);

        Column::new()
            .push(address_row)
            .push(password_input)
            .push(button)
            .align_items(Align::Start)
            .into()
    }
}

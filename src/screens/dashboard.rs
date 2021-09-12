use std::sync::Arc;

use iced::{
    button, text_input, Align, Button, Column, Element, HorizontalAlignment, Row, Text, TextInput,
};
use obws::Client;

#[derive(Clone, Debug)]
pub enum Message {}

pub struct State {
    client: Arc<Client>,
    back_button: button::State,
}

impl State {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client: client.clone(),
            back_button: button::State::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        use Message::*;
        match message {
            HostChanged(host) => {
                self.host = host;
                None
            }
            PortChanged(port) => {
                self.port = port;
                None
            }
            PasswordChanged(password) => {
                self.password = password;
                None
            }
            ButtonPressed => Some(Event::ButtonPressed),
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

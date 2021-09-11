use iced::{text_input, Align, Column, Element, Row, TextInput};

use super::Screen;

#[derive(Debug, Default)]
pub struct Login {
    host: String,
    port: String,
    password: String,
    host_state: text_input::State,
    port_state: text_input::State,
    password_state: text_input::State,
}

#[derive(Clone)]
pub enum Message {
    HostChanged(String),
    PortChanged(String),
    PasswordChanged(String),
}

impl Screen for Login {
    fn update(&mut self, msg: Message, debug: &mut bool) {}

    fn view(&mut self, debug: bool) -> Element<Message> {
        let host_input = TextInput::new(
            &mut self.host_state,
            "Host",
            "localhost",
            Message::HostChanged,
        )
        .padding(10)
        .size(30);

        let port_input = TextInput::new(&mut self.port_state, "port", "4444", Message::PortChanged)
            .padding(10)
            .size(30);

        let password_input = TextInput::new(
            &mut self.password_state,
            "password",
            "",
            Message::PasswordChanged,
        )
        .padding(10)
        .size(30)
        .password();

        let address_row = Row::new().spacing(2).push(host_input).push(port_input);

        Column::new()
            .push(address_row)
            .push(password_input)
            .align_items(Align::Start)
            .into()
    }
}

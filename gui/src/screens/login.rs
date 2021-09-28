use iced::{
    button, text_input, Align, Button, Column, Command, Container, Element, HorizontalAlignment,
    Row, Text, TextInput,
};
use twitch_oauth2::{tokens::ImplicitUserTokenBuilder, url::Url, ClientId};

#[derive(Debug, Default)]
pub struct State {
    button: button::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    ButtonPressed,
    Authenticated,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    LoginPageOpened,
}

impl State {
    fn open_login_url() -> anyhow::Result<()> {
        let client_id = ClientId::new(env!("TWITCH_CLIENT_ID"));
        let redirect_url = Url::parse(env!("TWITCH_REDIRECT_URL"))?;
        let mut builder = ImplicitUserTokenBuilder::new(client_id, redirect_url);
        let url: String = builder.generate_url().0.into();
        open::that(url)?;
        // builder.get_user_token(&surf::Client::new(), None, None, None, None).await;

        Ok(())
    }
    pub fn update(&mut self, message: Message) -> (Option<Event>, Command<Message>) {
        use Message::*;

        (
            match message {
                ButtonPressed => {
                    if let Err(_e) = Self::open_login_url() {
                        // todo!("{}", e);
                        None
                    } else {
                        Some(Event::LoginPageOpened)
                    }
                }
                Authenticated => None,
            },
            Command::none(),
        )
    }

    pub fn view(&mut self) -> Element<Message> {
        let button = Button::new(
            &mut self.button,
            Text::new("Sign in with Twitch").horizontal_alignment(HorizontalAlignment::Center),
        )
        .on_press(Message::ButtonPressed)
        .padding(10);

        Container::new(button)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .into()
    }
}

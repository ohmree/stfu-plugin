use iced::{futures::FutureExt, text_input, Command, Container, Element, TextInput};
use twitch_oauth2::{
    tokens::UserTokenBuilder,
    url::{self, Url},
    ClientId, ClientSecret, UserToken,
};

#[derive(Debug, Default)]
pub struct State {
    url_state: text_input::State,
    url: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    UrlChanged(String),
    UrlSubmitted(String),
    TokenFetched(Option<UserToken>),
}

#[derive(Clone, Debug)]
pub enum Event {
    UrlSubmitted(String),
    TokenFetched(UserToken),
}

impl State {
    pub fn update(&mut self, message: Message) -> (Option<Event>, Command<Message>) {
        use Message::*;

        match message {
            UrlChanged(url) => {
                self.url = url;
                (None, Command::none())
            }
            UrlSubmitted(url) => (
                None,
                if let Some(token) = url
                    .get(
                        (url.find("#access_token=").unwrap() + "#access_token=".len())
                            ..url.find('&').unwrap(),
                    )
                    .map(ToString::to_string)
                {
                    let client_secret = ClientSecret::new(env!("TWITCH_CLIENT_SECRET"));

                    Command::perform(
                        async move {
                            let surf = surf::Client::new();
                            UserToken::from_existing(&surf, token.into(), None, client_secret)
                                .map(Result::ok)
                                .await
                        },
                        Message::TokenFetched,
                    )
                } else {
                    Command::none()
                },
            ),
            TokenFetched(token) => (token.map(Event::TokenFetched), Command::none()),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let url_input = TextInput::new(
            &mut self.url_state,
            "Paste the url you were redirected to here",
            &self.url,
            Message::UrlChanged,
        )
        .password()
        .on_submit(Message::UrlSubmitted(self.url.clone()))
        .padding(10);
        url_input.into()
        // Container::new(url_input).center_x().center_y().into()
    }
}

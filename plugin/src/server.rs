// use anyhow::Context;
// use anyhow::Result;
// use async_std::{channel::Sender, task};
// use tide::{Request, StatusCode};
// use twitch_oauth2::{
//     id::TwitchTokenResponse, tokens::UserTokenBuilder, url::Url, ClientId, ClientSecret,
// };
// type TwitchAuthResult = Result<TwitchTokenResponse, TwitchTokenErrorResponse>;

// pub async fn authenticate(sender: Sender<TwitchTokenResponse>) -> Result<()> {
//     // TODO: figure out the best approach to unwrapping this `Option`.
//     let port = portpicker::pick_unused_port().expect("No free ports");
//     let redirect_uri = Url::parse(&format!("http://localhost:{}", port))?;

//     let mut builder = UserTokenBuilder::new(
//         ClientId::new(env!("TWITCH_CLIENT_ID")),
//         ClientSecret::new(env!("TWITCH_CLIENT_SECRET")),
//         redirect_uri.clone(),
//     )
//     .force_verify(true);

//     let (auth_url, _) = builder.generate_url();

//     let mut server = tide::with_state(sender);
//     server.at("/").get(|mut request: Request<Sender<TwitchTokenResponse>>| async move {
//         if let Ok(auth_response) = request.body_json::<TwitchTokenResponse>().await {
//             let sender = request.state();
//             sender.send(auth_response).await?;
//         }
//         Ok(tide::Response::from(StatusCode::Ok))
//     });

//     let server_task = task::spawn(server.listen(redirect_uri));

//     let stopper = stopper::Stopper::new();
//     let stopping_future = stopper.stop_future(server_task);

//     open::that(auth_url.as_str())?;
//     // let redirect_uri = twitch_oauth2::url::Url::parse(&input).context("when parsing the input as a URL")?;

//     // let map: std::collections::HashMap<_, _> = redirect_uri.query_pairs().collect();

//     // match (map.get("state"), map.get("code")) {
//     //     (Some(state), Some(code)) => {
//     //         let token = builder
//     //             .get_user_token(
//     //                 &reqwest::Client::builder()
//     //                     .redirect(reqwest::redirect::Policy::none())
//     //                     .build()?,
//     //                 state,
//     //                 code,
//     //             )
//     //             .await?;
//     //         println!("Got token: {:?}", token);
//     //     }
//     //     _ => match (map.get("error"), map.get("error_description")) {
//     //         (std::option::Option::Some(error), std::option::Option::Some(error_description)) => {
//     //             anyhow::bail!(
//     //                 "twitch errored with error: {} - {}",
//     //                 error,
//     //                 error_description
//     //             );
//     //         }
//     //         _ => anyhow::bail!("invalid url passed"),
//     //     },
//     // }
//     stopping_future.await.context("IDK")?;
//     Ok(())
// }

use std::borrow::Cow;

use anyhow::Context;
use cfg_if::cfg_if;
use directories_next::ProjectDirs;
use once_cell::sync::Lazy;
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

use interprocess::nonblocking::local_socket::*;

pub async fn create_server() -> anyhow::Result<LocalSocketListener> {
    let server = LocalSocketListener::bind(
        &**ADDRESS
            .as_ref()
            .context("Failed to create socket address")?,
    )
    .await?;

    Ok(server)
}

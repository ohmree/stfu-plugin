use anyhow::{Context, Result};
use cfg_if::cfg_if;
use directories_next::ProjectDirs;
use once_cell::sync::Lazy;
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

use interprocess::local_socket::LocalSocketListener;

pub fn create_server() -> Result<LocalSocketListener> {
    let server = LocalSocketListener::bind(
        &**ADDRESS
            .as_ref()
            .context("Failed to create socket address")?,
    )?;
    server.set_nonblocking(true)?;
    Ok(server)
}

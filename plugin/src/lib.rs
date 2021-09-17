mod server;

use std::sync::Arc;

use futures::{AsyncReadExt, TryFutureExt};
use interprocess::nonblocking::local_socket::LocalSocketListener;
use log::info;
use obs_wrapper::{
    // Logging
    log::Logger,
    // Macro for registering modules
    obs_register_module,
    // Macro for creating strings
    obs_string,
    // Everything required for modules
    prelude::*,
    // Everything required for creating a source
    source::*,
};
use server::create_server;
use tokio::sync::{Mutex, RwLock};
use untokio::v1 as untokio;

// The module that will handle creating the source.
struct Stfu {
    context: ModuleContext,
}

// The state of the source that is managed by OBS and used in each trait method.
#[derive(Default)]
struct RealData {
    is_muted: bool,
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<std::time::Duration>,
    server: Option<LocalSocketListener>,
}

type Data = Arc<RwLock<RealData>>;

// Implement the Sourceable trait for Stfu, this is required for each source.
// It allows you to specify the source ID and type.
impl Sourceable for Stfu {
    fn get_id() -> ObsString {
        obs_string!("stfu_source")
    }

    fn get_type() -> SourceType {
        SourceType::FILTER
    }
}

// Allow OBS to show a name for the source
impl GetNameSource<Data> for Stfu {
    fn get_name() -> ObsString {
        obs_string!("STFU")
    }
}

// Implement the Module trait for Stfu. This will handle the creation of the source and
// has some methods for telling OBS a bit about itself.
impl Module for Stfu {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    // Load the module - create all sources, returning true if all went well.
    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        // Enable logging
        let _ = Logger::new().with_promote_debug(true).init();

        // Create the source
        let source = load_context
            .create_source_builder::<Stfu, Data>()
            // Since GetNameSource is implemented, this method needs to be called to
            // enable it.
            .enable_get_name()
            .enable_create()
            .enable_filter_audio()
            .enable_update()
            .build();

        // Tell OBS about the source so that it will show it.
        load_context.register_source(source);

        // Nothing could have gone wrong, so return true.
        true
    }

    fn description() -> ObsString {
        obs_string!("Mute your streamer using channel points :tf:")
    }

    fn name() -> ObsString {
        obs_string!("STFU")
    }

    fn author() -> ObsString {
        obs_string!("OhmRee")
    }
}

impl FilterAudioSource<Data> for Stfu {
    fn filter_audio(data: &mut Option<Data>, audio: &mut audio::AudioDataContext) {
        let audio = Arc::new(Mutex::new(audio));
        if let Some(data) = data {
            let data = data.clone();
            let audio = audio.clone();
            futures::executor::block_on(untokio::spawn(async move {
                let data = data.read_owned().await;
                let audio = audio.lock_owned().await;
                let channels = (0..audio.channels())
                    .map(|c| audio.get_channel_as_mut_slice(c))
                    .filter_map(|c| c)
                    .collect::<Vec<_>>();

                if data.is_muted {
                    for channel in channels.iter() {
                        for i in channel.iter() {
                            *i = 0f32
                        }
                    }
                }
            }))
            .unwrap()
        }
    }
}

impl UpdateSource<Data> for Stfu {
    fn update(data: &mut Option<Data>, settings: &mut DataObj, _context: &mut GlobalContext) {
        if let Some(data) = data {
            let data = data.clone();
            untokio::spawn(async move {
                let data = data.write_owned().await;
                if let Some(server) = &data.server {
                    if let Some(reply) = server.accept().await.ok() {
                        let mut buf = [0];
                        reply.read(&mut buf);
                        if buf[0] == 0 {
                            data.is_muted = false;
                        } else {
                            data.is_muted = true;
                        }
                    }
                }
            });
        }
    }
}

impl CreatableSource<Data> for Stfu {
    fn create(_create: &mut CreatableSourceContext<Data>, _source: SourceContext) -> Data {
        Arc::new(RwLock::new(RealData {
            server: futures::executor::block_on(untokio::spawn(async {
                create_server()
                    .map_err(|e| {
                        info!("{}", e);
                        e
                    })
                    .await
                    .ok()
            }))
            .unwrap(),
            ..Default::default()
        }))
    }
}

obs_register_module!(Stfu);

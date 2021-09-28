mod server;
mod util;

use std::io::Read;
use util::LogErr;

use interprocess::local_socket::LocalSocketListener;
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

// The module that will handle creating the source.
struct Stfu {
    context: ModuleContext,
}

// The state of the source that is managed by OBS and used in each trait method.
#[derive(Default, Debug)]
struct Data {
    is_muted: bool,
    server: Option<LocalSocketListener>,
}

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
        let _ = Logger::new().with_promote_debug(true).with_max_level(log::LevelFilter::Warn).init();

        // Create the source
        let source = load_context
            .create_source_builder::<Stfu, Data>()
            // Since GetNameSource is implemented, this method needs to be called to
            // enable it.
            .enable_get_name()
            .enable_create()
            .enable_filter_audio()
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
        if let Some(data) = data {
            if data.is_muted {
                for channel in (0..audio.channels())
                    .map(|c| audio.get_channel_as_mut_slice(c))
                    .flatten()
                {
                    for i in channel.iter_mut() {
                        *i = 0f32
                    }
                }
            }
            if let Ok(mut conn) = data.server.as_ref().unwrap().accept() {
                // TODO: figure out if we need this.
                // conn.set_nonblocking(true);
                let mut buf = [0];
                if conn.read(&mut buf).log_err("Failed to read").is_ok() {
                    data.is_muted = buf[0] != 0;
                    if data.is_muted {
                        info!("Muting");
                    } else {
                        info!("Unmuting");
                    }
                }
            }
        }
    }
}

impl CreatableSource<Data> for Stfu {
    fn create(_create: &mut CreatableSourceContext<Data>, _source: SourceContext) -> Data {
        use log::*;
        trace!("Trace");
        debug!("Debug");
        info!("Info");
        warn!("Warn");
        error!("Error");

        Data {
            server: create_server()
                .log_err("Failed to create server")
                .ok(),
            is_muted: false,
        }
    }
}

obs_register_module!(Stfu);

pub trait LogErr {
    fn log_err(self, msg: impl AsRef<str>) -> Self;
}

impl<T, E: std::fmt::Display> LogErr for std::result::Result<T, E> {
    fn log_err(self, msg: impl AsRef<str>) -> Self {
        self.map_err(|e| {
            log::info!("{}: {}", msg.as_ref(), e);
            e
        })
    }
}

use anyhow::Result;

pub trait AnyhowExt<T> {
    fn with_msg(self, msg: &str) -> Result<T>;
}

impl<T, E> AnyhowExt<T> for std::result::Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn with_msg(self, msg: &str) -> Result<T> {
        self.with_context(|| msg.to_string())
    }
}

use anyhow::bail;
use anyhow::Context;
use std::env;

pub(crate) trait IsBlank {
    fn is_blank(&self) -> bool;
}

impl<T: AsRef<str>> IsBlank for T {
    fn is_blank(&self) -> bool {
        let string = self.as_ref();
        string.is_empty() || string.chars().all(|char| char.is_whitespace())
    }
}

pub(crate) trait RequireNonBlank {
    fn require_non_blank(self) -> anyhow::Result<String>;
}

impl<E> RequireNonBlank for anyhow::Result<String, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn require_non_blank(self) -> anyhow::Result<String> {
        match self {
            Ok(string) => {
                if string.is_blank() {
                    bail!("String was blank")
                } else {
                    Ok(string)
                }
            }
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}

pub(crate) fn get_required_env_var(name: &str) -> anyhow::Result<String> {
    env::var(name)
        .require_non_blank()
        .with_context(|| format!("Expected '{name}' environment variable to be set"))
}

pub(crate) fn get_optional_env_var(name: &str) -> Option<String> {
    env::var(name).require_non_blank().ok()
}

use thiserror::Error;

pub type Result<T> = core::result::Result<T, MangonneauError>;

#[derive(Debug, Error)]
pub enum MangonneauError {}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BungusError {
    #[error(transparent)]
    InternalError(#[from] InternalError),

    #[error(transparent)]
    CommandError(#[from] CommandError),
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error(transparent)]
    MissingEnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    EnvParseError(#[from] dotenv::Error),

    #[error(transparent)]
    SerenityError(#[from] poise::serenity_prelude::prelude::SerenityError),

    #[error(transparent)]
    IntConversionError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Token not found")]
    TokenNotFound,
}

#[derive(Error, Debug)]
pub enum CommandError {}

macro_rules! def_error_conv {
    (
        $from: path, $to: ident, $internal_type: ident
    ) => {
        impl From<$from> for $to {
            fn from(e: $from) -> Self {
                $internal_type::from(e).into()
            }
        }
    };
}

def_error_conv!(poise::serenity_prelude::prelude::SerenityError, BungusError, InternalError);
def_error_conv!(std::num::ParseIntError, BungusError, InternalError);
def_error_conv!(dotenv::Error, BungusError, InternalError);
def_error_conv!(serde_json::error::Error, BungusError, InternalError);
def_error_conv!(std::io::Error, BungusError, InternalError);
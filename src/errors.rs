use std::fmt::{self, Formatter};

#[macro_export]
macro_rules! error {
    ($stage:expr, $($arg:tt)*) => {
        $crate::errors::Error {
            message: format!($($arg)*),
            stage: $crate::errors::Stage::try_from($stage).unwrap_or_else(|e| panic!("{e}")),
            line: line!(),
            file: file!(),
        }
    };
}

pub type Result<T> = std::result::Result<T, self::Error>;

impl<T> From<self::Error> for self::Result<T> {
    fn from(err: self::Error) -> Self {
        Err(err)
    }
}

#[derive(Debug)]
pub struct Error {
    pub(crate) message: String,
    pub(crate) stage: Stage,
    pub(crate) line: u32,
    pub(crate) file: &'static str,
}

impl std::error::Error for self::Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if cfg!(debug_assertions) {
            let Self {
                message,
                stage,
                line,
                file,
            } = self;
            write!(f, "{stage} error [{file}:{line}]: {message}")
        } else {
            let Self { message, stage, .. } = self;
            write!(f, "{stage} error: {message}")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stage {
    Lexer,
    Parser,
    Compiler,
    Runtime,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            Stage::Lexer => "lexer",
            Stage::Parser => "parser",
            Stage::Compiler => "compiler",
            Stage::Runtime => "runtime",
        };
        fmt::Display::fmt(name, f)
    }
}

#[derive(Debug)]
pub struct StageParseError {
    /// Original value that should ahve been a stage enum.
    pub input: String,
}

impl fmt::Display for StageParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { input } = self;
        write!(f, "failed to parse error stage: {input:?}")
    }
}

impl TryFrom<&str> for Stage {
    type Error = StageParseError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "lexer" => Ok(Stage::Lexer),
            "parser" => Ok(Stage::Parser),
            "compiler" => Ok(Stage::Compiler),
            "runtime" => Ok(Stage::Runtime),
            _ => Err(StageParseError {
                input: value.to_string(),
            }),
        }
    }
}

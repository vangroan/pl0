use std::fmt::{self, Formatter};

#[macro_export]
macro_rules! error {
    ($stage:expr, $($arg:tt)*) => {
        $crate::errors::Error {
            message: format!($($arg)*),
            stage: $crate::errors::Stage::try_from($stage).unwrap_or_else(|e| panic!("{e}")),
            guest_loc: None,
            location: $crate::errors::HostLoc { line: line!(), file: file!() },
            note: None,
        }
    };
}

pub type Result<T> = std::result::Result<T, self::Error>;

pub trait ResultExt {
    fn with_note<R: ToString>(self, f: impl FnOnce() -> R) -> Self;
}

impl<T> ResultExt for Result<T> {
    #[inline(always)]
    fn with_note<R: ToString>(self, f: impl FnOnce() -> R) -> Self {
        self.map_err(|mut err| {
            err.note = Some(f().to_string());
            err
        })
    }
}

impl<T> From<self::Error> for self::Result<T> {
    fn from(err: self::Error) -> Self {
        Err(err)
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub(crate) message: String,
    pub(crate) stage: Stage,
    pub(crate) guest_loc: Option<GuestLoc>,
    pub(crate) location: HostLoc,
    pub(crate) note: Option<String>,
}

/// Code location in PL/0 script.
#[derive(Debug, Clone)]
pub(crate) struct GuestLoc {
    pub(crate) span: (u32, u32),
    pub(crate) file: String,
}

/// Code lcoation in Rust code.
#[derive(Debug, Clone)]
pub(crate) struct HostLoc {
    pub(crate) line: u32,
    pub(crate) file: &'static str,
}

impl Error {
    pub fn pretty<'a, 'b>(&'a self, text: &'b str) -> ErrorPretty<'a, 'b> {
        ErrorPretty { err: self, text }
    }
}

impl std::error::Error for self::Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if cfg!(debug_assertions) {
            let Self {
                message,
                stage,
                location: HostLoc { line, file },
                ..
            } = self;
            write!(f, "{stage} error [{file}:{line}]: {message}")
        } else {
            let Self { message, stage, .. } = self;
            write!(f, "{stage} error: {message}")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub struct ErrorPretty<'a, 'b> {
    err: &'a self::Error,
    text: &'b str,
}

impl<'a, 'b> fmt::Display for ErrorPretty<'a, 'b> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //    --> src\errors.rs:101:5
        //     |
        // 100 | pub struct ErrorPretty<'a> {
        //     |            ----------- field in this struct
        // 101 |     err: &'a self::Error,
        //     |     ^^^
        //     |
        //     = note: `#[warn(dead_code)]` on by default

        // error: expected `;`, found `todo`
        //    --> src\errors.rs:114:12
        //     |
        // 114 |         asd
        //     |            ^ help: add `;` here
        // 115 |         todo!()
        //     |         ---- unexpected token

        // error: could not compile `pl0` (lib) due to 1 previous error
        let Error {
            stage,
            message,
            guest_loc,
            location,
            note,
            ..
        } = self.err;

        // Error Message
        writeln!(f, "{stage} error: {message}")?;

        // Block Header (Optional)
        // procedure foobar;

        // Text Fragment
        if let Some(_guest_loc) = guest_loc {
            writeln!(f, "|")?;
            writeln!(f, "|")?;
            writeln!(f, "|")?;
            writeln!(f, "|")?;
        };

        // Notes (Optional)
        if let Some(note) = note {
            writeln!(f, "note: {note}")?
        };

        // Rust Location
        write!(f, "rust: {}:{}", location.file, location.line)?;

        Ok(())
    }
}

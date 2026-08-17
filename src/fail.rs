//use failure::Backtrace;

use parking_lot::Mutex;
use termion::event::Key;

use std::path::PathBuf;
use std::sync::Arc;

use crate::foldview::LogEntry;
use crate::mediaview::MediaError;

pub type HResult<T> = Result<T, HError>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum HError {
    #[error("IO error: {0} ")]
    IoError(String),
    #[error("Mutex failed")]
    MutexError,
    #[error("Can't lock!")]
    TryLockError,
    #[error("Channel failed: {error}")]
    ChannelTryRecvError {
        #[source]
        error: std::sync::mpsc::TryRecvError,
    },
    #[error("Channel failed: {error}")]
    ChannelRecvError {
        #[source]
        error: std::sync::mpsc::RecvError,
    },
    #[error("Channel failed")]
    ChannelSendError,
    #[error("Timer ran out while waiting for message on channel!")]
    ChannelRecvTimeout(#[source] std::sync::mpsc::RecvTimeoutError),
    #[error("Previewer failed on file: {file}")]
    PreviewFailed { file: String },
    #[error("StalePreviewer for file: {file}")]
    StalePreviewError { file: String },
    #[error("Accessed stale value")]
    StaleError,
    #[error("Failed: {0}")]
    Error(String),
    #[error("Was None!")]
    NoneError,
    #[error("Async Error: {0}")]
    AError(crate::async_value::AError),
    #[error("No widget found")]
    NoWidgetError,
    #[error("Path: {path:?} not in this directory: {dir:?}")]
    WrongDirectoryError { path: PathBuf, dir: PathBuf },
    #[error("Widget finnished")]
    PopupFinnished,
    #[error("No completions found")]
    NoCompletionsError,
    #[error("No more history")]
    NoHistoryError,
    #[error("No core for widget")]
    NoWidgetCoreError,
    #[error("No header for widget")]
    NoHeaderError,
    #[error("You wanted this!")]
    Quit,
    #[error("HBox ratio mismatch: {wnum} widgets, ratio is {ratio:?}")]
    HBoxWrongRatioError { wnum: usize, ratio: Vec<usize> },
    #[error("Got wrong widget: {got}! Wanted: {wanted}")]
    WrongWidgetError { got: String, wanted: String },
    #[error("Strip Prefix Error: {error}")]
    StripPrefixError {
        #[source]
        error: std::path::StripPrefixError,
    },
    #[error("INofify failed: {0}")]
    INotifyError(String),
    #[error("Tags not loaded yet")]
    TagsNotLoadedYetError,
    #[error("Undefined key: {key:?}")]
    WidgetUndefinedKeyError { key: Key },
    #[error("Terminal has been resized!")]
    TerminalResizedError,
    #[error("Widget has been resized!")]
    WidgetResizedError,
    #[error("{0}")]
    Log(String),
    #[error("Metadata already processed")]
    MetadataProcessedError,
    #[error("No files to take from widget")]
    WidgetNoFilesError,
    #[error("Invalid line in settings file: {0}")]
    ConfigLineError(String),
    #[error("New input in Minibuffer")]
    MiniBufferInputUpdated(String),
    #[error("Failed to parse into UTF8")]
    UTF8ParseError(std::str::Utf8Error),
    #[error("Failed to parse integer!")]
    ParseIntError(std::num::ParseIntError),
    #[error("Failed to parse char!")]
    ParseCharError(std::char::ParseCharError),
    #[error("{0}")]
    Media(MediaError),
    #[error("{0}")]
    Mime(MimeError),
    #[error("{0}")]
    KeyBind(KeyBindError),
    #[error("FileBrowser needs to know about all tab's files to run exec!")]
    FileBrowserNeedTabFiles,
    #[error("{0}")]
    FileError(crate::files::FileError),
    #[error("{0}")]
    Nix(#[source] nix::Error),
    #[error("Refresh parent widget!")]
    RefreshParent,
    #[error("Refresh parent widget!")]
    MiniBufferEvent(crate::minibuffer::MiniBufferEvent),
}

impl HError {
    pub fn log<T>(log: &str) -> HResult<T> {
        Err(HError::Log(String::from(log))).log_and()
    }
    pub fn quit() -> HResult<()> {
        Err(HError::Quit)
    }
    pub fn wrong_ratio<T>(wnum: usize, ratio: Vec<usize>) -> HResult<T> {
        Err(HError::HBoxWrongRatioError {
            wnum: wnum,
            ratio: ratio,
        })
    }
    pub fn no_widget<T>() -> HResult<T> {
        Err(HError::NoWidgetError)
    }
    pub fn wrong_widget<T>(got: &str, wanted: &str) -> HResult<T> {
        Err(HError::WrongWidgetError {
            got: got.to_string(),
            wanted: wanted.to_string(),
        })
    }
    pub fn popup_finnished<T>() -> HResult<T> {
        Err(HError::PopupFinnished)
    }
    pub fn tags_not_loaded<T>() -> HResult<T> {
        Err(HError::TagsNotLoadedYetError)
    }
    pub fn undefined_key<T>(key: Key) -> HResult<T> {
        Err(HError::WidgetUndefinedKeyError { key: key })
    }
    pub fn wrong_directory<T>(path: PathBuf, dir: PathBuf) -> HResult<T> {
        Err(HError::WrongDirectoryError {
            path: path,
            dir: dir,
        })
    }
    pub fn preview_failed<T>(file: &crate::files::File) -> HResult<T> {
        let name = file.name.clone();
        Err(HError::PreviewFailed { file: name })
    }

    pub fn terminal_resized<T>() -> HResult<T> {
        Err(HError::TerminalResizedError)
    }

    pub fn widget_resized<T>() -> HResult<T> {
        Err(HError::WidgetResizedError)
    }

    pub fn stale<T>() -> HResult<T> {
        Err(HError::StaleError)
    }

    pub fn config_error<T>(line: String) -> HResult<T> {
        Err(HError::ConfigLineError(line))
    }

    pub fn metadata_processed<T>() -> HResult<T> {
        Err(HError::MetadataProcessedError)
    }

    pub fn no_files<T>() -> HResult<T> {
        Err(HError::WidgetNoFilesError)
    }

    pub fn input_updated<T>(input: String) -> HResult<T> {
        Err(HError::MiniBufferInputUpdated(input))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ErrorCause {
    #[error("{0}")]
    Str(String),
}

lazy_static! {
    static ref LOG: Mutex<Vec<LogEntry>> = Mutex::new(vec![]);
}

pub fn get_logs() -> HResult<Vec<LogEntry>> {
    let logs = LOG.lock().drain(..).collect();
    Ok(logs)
}

pub fn put_log<L: Into<LogEntry>>(log: L) -> HResult<()> {
    LOG.lock().push(log.into());
    Ok(())
}

pub trait ErrorLog
where
    Self: Sized,
{
    fn log(self);
    fn log_and(self) -> Self;
}

// impl<T> ErrorLog for HResult<T> {
//     fn log(self) {
//         if let Err(err) = self {
//             put_log(&err).ok();
//         }
//     }

//     fn log_and(self) -> Self {
//         if let Err(err) = &self {
//             put_log(err).ok();
//         }
//         self
//     }
// }

// impl<T> ErrorLog for Result<T, AError> {
//     fn log(self) {
//         if let Err(err) = self {
//             put_log(&err.into()).ok();
//         }
//     }

//     fn log_and(self) -> Self {
//         if let Err(err) = &self {
//             put_log(&err.clone().into()).ok();
//         }
//         self
//     }
// }

impl<T, E> ErrorLog for Result<T, E>
where
    E: Into<HError> + Clone,
{
    fn log(self) {
        if let Err(err) = self {
            let err: HError = err.into();
            put_log(&err).ok();
        }
    }
    fn log_and(self) -> Self {
        if let Err(ref err) = self {
            let err: HError = err.clone().into();
            put_log(&err).ok();
        }
        self
    }
}

impl<E> ErrorLog for E
where
    E: Into<HError> + Clone,
{
    fn log(self) {
        let err: HError = self.into();
        put_log(&err).ok();
    }
    fn log_and(self) -> Self {
        let err: HError = self.clone().into();
        put_log(&err).ok();
        self
    }
}

impl From<std::io::Error> for HError {
    fn from(error: std::io::Error) -> Self {
        let err = HError::IoError(format!("{}", error));
        err
    }
}

impl From<anyhow::Error> for HError {
    fn from(error: anyhow::Error) -> Self {
        let err = HError::Error(format!("{}", error));
        err
    }
}

impl From<std::sync::mpsc::TryRecvError> for HError {
    fn from(error: std::sync::mpsc::TryRecvError) -> Self {
        let err = HError::ChannelTryRecvError { error: error };
        err
    }
}

impl From<std::sync::mpsc::RecvError> for HError {
    fn from(error: std::sync::mpsc::RecvError) -> Self {
        let err = HError::ChannelRecvError { error: error };
        err
    }
}

impl From<std::sync::mpsc::RecvTimeoutError> for HError {
    fn from(error: std::sync::mpsc::RecvTimeoutError) -> Self {
        let err = HError::ChannelRecvTimeout(error);
        err
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for HError {
    fn from(_error: std::sync::mpsc::SendError<T>) -> Self {
        let err = HError::ChannelSendError;
        err
    }
}

impl<T> From<std::sync::PoisonError<T>> for HError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        let err = HError::MutexError;
        err
    }
}

impl<T> From<std::sync::TryLockError<T>> for HError {
    fn from(_error: std::sync::TryLockError<T>) -> Self {
        let err = HError::TryLockError;
        err
    }
}

impl From<std::path::StripPrefixError> for HError {
    fn from(error: std::path::StripPrefixError) -> Self {
        let err = HError::StripPrefixError { error: error };
        err
    }
}

impl From<notify::Error> for HError {
    fn from(error: notify::Error) -> Self {
        let err = HError::INotifyError(format!("{}", error));
        err
    }
}

impl From<crate::async_value::AError> for HError {
    fn from(error: crate::async_value::AError) -> Self {
        let err = HError::AError(error);
        err
    }
}

impl From<std::str::Utf8Error> for HError {
    fn from(error: std::str::Utf8Error) -> Self {
        let err = HError::UTF8ParseError(error);
        err
    }
}

impl From<std::num::ParseIntError> for HError {
    fn from(error: std::num::ParseIntError) -> Self {
        let err = HError::ParseIntError(error);
        err
    }
}

impl From<nix::Error> for HError {
    fn from(error: nix::Error) -> Self {
        let err = HError::Nix(error);
        err
    }
}

impl From<std::char::ParseCharError> for HError {
    fn from(error: std::char::ParseCharError) -> Self {
        let err = HError::ParseCharError(error);
        err
    }
}

// MIME Errors

#[derive(Debug, Clone, thiserror::Error)]
pub enum MimeError {
    #[error("Need a file to determine MIME type")]
    NoFileProvided,
    #[error("File access failed! Error: {0}")]
    AccessFailed(Box<HError>),
    #[error("No MIME type found for this file")]
    NoMimeFound,
    #[error("Paniced while trying to find MIME type for: {0}!")]
    Panic(String),
}

impl From<MimeError> for HError {
    fn from(e: MimeError) -> Self {
        HError::Mime(e)
    }
}

impl From<KeyBindError> for HError {
    fn from(e: KeyBindError) -> Self {
        HError::KeyBind(e)
    }
}

impl From<crate::minibuffer::MiniBufferEvent> for HError {
    fn from(e: crate::minibuffer::MiniBufferEvent) -> Self {
        HError::MiniBufferEvent(e)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum KeyBindError {
    #[error("Movement has not been defined for this widget")]
    MovementUndefined,
    #[error("Keybind defined with wrong key: {0} -> {1}")]
    WrongKey(String, String),
    #[error("Defined keybind for non-existing action: {0}")]
    WrongAction(String),
    #[error("Failed to parse keybind: {0}")]
    ParseKeyError(String),
    #[error("Trouble with ini file! Error: {0}")]
    IniError(Arc<ini::ini::Error>),
    #[error("Couldn't parse as either char or u8: {0}")]
    CharOrNumParseError(String),
    #[error("Wanted {0}, but got {1}!")]
    CharOrNumWrongType(String, String),
}

impl From<ini::ini::Error> for KeyBindError {
    fn from(err: ini::ini::Error) -> Self {
        KeyBindError::IniError(Arc::new(err))
    }
}

impl From<crate::files::FileError> for HError {
    fn from(err: crate::files::FileError) -> Self {
        HError::FileError(err)
    }
}

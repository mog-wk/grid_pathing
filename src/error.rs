
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic Error")]
    Generic(String),
    #[error("grid build error")]
    GridBuildError,

    #[error(transparent)]
    WindowBuildError(#[from] sdl2::video::WindowBuildError),
    #[error(transparent)]
    CanvasBuildError(#[from] sdl2::IntegerOrSdlError),
}

impl std::convert::From<String> for Error {
    fn from(st: String) -> Self {
        Error::Generic(st)
    }
}

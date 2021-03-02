use failure::Fail;

#[derive(Debug, Fail)]
pub enum ContextualError {
    /// Fully customized errors, not inheriting from any error
    #[allow(dead_code)]
    #[fail(display = "{}", _0)]
    CustomError(String),

    /// Any kind of IO errors
    #[allow(dead_code)]
    #[fail(display = "{}\ncaused by: {}", _0, _1)]
    IOError(String, std::io::Error),

    /// MultipartError, which might occur during file upload, when processing the multipart request fails
    #[allow(dead_code)]
    #[fail(display = "Failed to process multipart request\ncaused by: {}", _0)]
    MultipartError(actix_multipart::MultipartError),

    /// Any error related to an invalid path (failed to retrieve entry name, unexpected entry type, etc)
    #[allow(dead_code)]
    #[fail(display = "Invalid path\ncaused by: {}", _0)]
    InvalidPathError(String),

    /// This error might occur if the HTTP credential string does not respect the expected format
    #[allow(dead_code)]
    #[fail(
        display = "Invalid format for credentials string. Expected username:password, username:sha256:hash or username:sha512:hash"
    )]
    InvalidAuthFormat,

    /// This error might occure if the hash method is neither sha256 nor sha512
    #[allow(dead_code)]
    #[fail(
        display = "{} is not a valid hashing method. Expected sha256 or sha512",
        _0
    )]
    InvalidHashMethod(String),

    /// This error might occur if the HTTP auth hash password is not a valid hex code
    #[allow(dead_code)]
    #[fail(display = "Invalid format for password hash. Expected hex code")]
    InvalidPasswordHash,

    /// This error might occur if the HTTP auth password exceeds 255 characters
    #[allow(dead_code)]
    #[fail(display = "HTTP password length exceeds 255 characters")]
    PasswordTooLongError,

    /// This error might occur if the user has unsufficient permissions to create an entry in a given directory
    #[allow(dead_code)]
    #[fail(display = "Insufficient permissions to create file in {}", _0)]
    InsufficientPermissionsError(String),

    /// Any error related to parsing.
    #[fail(display = "Failed to parse {}\ncaused by: {}", _0, _1)]
    #[allow(dead_code)]
    ParseError(String, String),

    /// This error might occur when the creation of an archive fails
    #[allow(dead_code)]
    #[fail(
        display = "An error occured while creating the {}\ncaused by: {}",
        _0, _1
    )]
    ArchiveCreationError(String, Box<ContextualError>),

    /// This error might occur when the HTTP authentication fails
    #[allow(dead_code)]
    #[fail(
        display = "An error occured during HTTP authentication\ncaused by: {}",
        _0
    )]
    HTTPAuthenticationError(Box<ContextualError>),

    /// This error might occur when the HTTP credentials are not correct
    #[allow(dead_code)]
    #[fail(display = "Invalid credentials for HTTP authentication")]
    InvalidHTTPCredentials,

    /// This error might occur when an HTTP request is invalid
    #[allow(dead_code)]
    #[fail(display = "Invalid HTTP request\ncaused by: {}", _0)]
    InvalidHTTPRequestError(String),

    /// This error might occur when trying to access a page that does not exist
    #[allow(dead_code)]
    #[fail(display = "Route {} could not be found", _0)]
    RouteNotFoundError(String),
}
#[allow(dead_code)]
pub fn log_error_chain(description: String) {
    for cause in description.lines() {
        log::error!("{}", cause);
    }
}

/// This makes creating CustomErrors easier
impl From<String> for ContextualError {
    fn from(msg: String) -> ContextualError {
        ContextualError::CustomError(msg)
    }
}

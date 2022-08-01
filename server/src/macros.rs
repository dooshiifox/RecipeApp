/// Prints out an error message by passing the provided args to [`format!`]
/// with a random UUID, returning the UUID.
///
/// The purpose of this is to be used in `InternalServerError` errors in
/// the API. If we are given an Error ID by a user when they recieve an
/// error, we can more easily search through the logs if using this.
///
/// [`format!`]: https://doc.rust-lang.org/std/macro.format.html
#[macro_export]
macro_rules! id_error {
    ($($arg:tt)*) => {{
        use tracing::error;
        let err_id = Uuid::random();
        error!("Err ID: {}\n{}", err_id, format!($($arg)*));
        err_id
    }};
}

/// Finds the environment variables and returns them as a
/// `Result<String, String>`
///
/// Specifying a filename with `envvar!({var} from {file})` will make the
/// returned error message more specific, and is advised. However, if the
/// filename is not known, simply `envvar!({var})` can be used.
#[macro_export]
macro_rules! envvar {
    ($name:ident) => {
        dotenv::var(stringify!($name))
            .map_err(|_| format!("Could not find envvar `{}`", stringify!($name)))
    };

    ($name:ident from $filename:expr) => {
        dotenv::var(stringify!($name)).map_err(|_| {
            format!(
                "Could not find envvar `{}` in file `{}`",
                stringify!($name),
                $filename
            )
        })
    };
}

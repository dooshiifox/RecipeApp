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

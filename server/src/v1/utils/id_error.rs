#[macro_export]
macro_rules! id_error {
    ($($arg:tt)*) => {{
        let err_id = Uuid::random();
        error!("Err ID: {}\n{}", err_id, format!($($arg)*));
        err_id
    }};
}

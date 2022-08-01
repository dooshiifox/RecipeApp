use actix_web::HttpRequest;

/// Any issues with checking the user is authenticated to perform an action.
pub enum AuthError {
    /// There is no authentication header.
    NoAuthHeader,
    /// The authentication header cannot be converted to a string.
    AuthHeaderInvalid,
    /// The authentication header does not start with "Bearer".
    NotBearer,
    /// The authentication key encountered an error splitting at "Bearer".
    /// This should never happen.
    AuthKeySplitError,
    /// The key was not correct.
    InvalidKey,
}

/// Checks if a user is authenticated to perform an action.
pub fn check_user_auth(req: HttpRequest) -> Result<(), AuthError> {
    // Get the auth header from the request. Return a 401 if not present.
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return Err(AuthError::NoAuthHeader),
    };

    // Convert auth header to string. Check it starts with "Bearer",
    // or else it won't be considered valid.
    let auth_content = match auth_header.to_str() {
        Ok(content) => content,
        Err(_) => return Err(AuthError::AuthHeaderInvalid),
    };

    if !auth_content.starts_with("Bearer") {
        return Err(AuthError::NotBearer);
    }

    // Convert to key.
    let key = match auth_content.split_once("Bearer") {
        Some(("", key)) => key.trim(),
        _ => return Err(AuthError::AuthKeySplitError),
    };

    // Check the auth key matches the secret key.
    if key != crate::envvar!(SECRET_KEY).unwrap() {
        return Err(AuthError::InvalidKey);
    }

    Ok(())
}

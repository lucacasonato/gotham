pub use jsonwebtoken::TokenData;

/// Struct to contain the JSON Web Token on a per-request basis.
#[derive(Debug)]
pub struct AuthorizationToken<T: Send + 'static>(pub TokenData<T>);

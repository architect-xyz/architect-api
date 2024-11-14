//! JWT authentication claims for upstream gRPC services

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims<'a> {
    pub aud: Cow<'a, str>,
    pub exp: i64,
    pub iat: i64,
    pub iss: Cow<'a, str>,
    pub nbf: i64,
    pub sub: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk<'a> {
    /// JWT key id; understood to be SHAKE256(issuer/subject/domain),
    /// truncated to 128 bits, and hex-encoded
    pub kid: Cow<'a, str>,
    /// Base64-encoded RSA modulus (big-endian)
    pub n: Cow<'a, str>,
    /// Base64-encoded RSA exponent (big-endian)
    pub e: Cow<'a, str>,
}

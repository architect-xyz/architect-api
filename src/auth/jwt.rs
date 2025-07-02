//! JWT authentication claims for upstream gRPC services

use super::Grants;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deref, Clone, Serialize, Deserialize)]
pub struct Claims<'a> {
    pub aud: Cow<'a, str>,
    pub exp: i64,
    pub iat: i64,
    pub iss: Cow<'a, str>,
    pub nbf: i64,
    pub sub: Cow<'a, str>,
    #[serde(flatten)]
    #[deref]
    pub grants: Grants,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_serialization() {
        // legacy JWT
        let json = r#"
        {
            "aud": "*.architect.co",
            "exp": 1751396859,
            "iat": 1719859200,
            "iss": "app.architect.co",
            "nbf": 1751310459,
            "sub": "test@architect.co"
        }
        "#;
        let claims: Claims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.is_scoped(), false);
        assert_eq!(claims.allowed_to_marketdata(&"CME".into()), true);
        // new scoped JWT
        let json = r#"
        {
            "aud": "*.architect.co",
            "exp": 1751396859,
            "iat": 1719859200,
            "iss": "app.architect.co",
            "nbf": 1751310459,
            "sub": "test@architect.co",
            "scoped": true,
            "marketdata": ["CME"]
        }
        "#;
        let claims: Claims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.is_scoped(), true);
        assert_eq!(claims.allowed_to_marketdata(&"CME".into()), true);
        let json = r#"
        {
            "aud": "*.architect.co",
            "exp": 1751396859,
            "iat": 1719859200,
            "iss": "app.architect.co",
            "nbf": 1751310459,
            "sub": "test@architect.co",
            "scoped": true
        }
        "#;
        let claims: Claims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.is_scoped(), true);
        assert_eq!(claims.allowed_to_marketdata(&"CME".into()), false);
        let json = r#"
        {
            "aud": "*.architect.co",
            "exp": 1751396859,
            "iat": 1719859200,
            "iss": "app.architect.co",
            "nbf": 1751310459,
            "sub": "test@architect.co",
            "scoped": true,
            "marketdata": true
        }
        "#;
        let claims: Claims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.is_scoped(), true);
        assert_eq!(claims.allowed_to_marketdata(&"CME".into()), true);
    }
}

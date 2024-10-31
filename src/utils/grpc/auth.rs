use crate::UserId;
use anyhow::Result;
use arcstr::ArcStr;
use rustls_pki_types::CertificateDer;
use std::task::{Context, Poll};
use tonic::transport::server::{TcpConnectInfo, TlsConnectInfo};
use tower_layer::Layer;
use tower_service::Service;

#[derive(Debug, Clone)]
pub enum AuthInfo {
    Tls { subject: ArcStr, user_id: UserId },
}

impl AuthInfo {
    pub fn subject(&self) -> Option<&ArcStr> {
        match self {
            Self::Tls { subject, .. } => Some(subject),
        }
    }
}

/// Read the TLS certificate attached to a request, and forward it's subject onto the wrapped services
///
/// To use this, add a layer to your tonic server as follows:
///
/// ```ignore
/// let mut server = Server::builder();
/// server.layer(AuthInfoLayer);
/// ```
///
/// Make sure to serve TLS and validate the certificate against the Architect CA as well.
///
/// ```ignore
/// let tls_config = ServerTlsConfig::new();
/// server.identity(tls.identity)
///       .client_ca_root(tls.trusted)
///       .client_auth_optional(false);
/// ```
///
/// To access to resulting `AuthInfo` object on the request, call `request.extensions::<AuthInfo>()`
#[derive(Copy, Clone)]
pub struct AuthInfoLayer;

impl<S: Clone> Layer<S> for AuthInfoLayer {
    type Service = AuthInfoService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthInfoService { inner }
    }
}

#[derive(Clone)]
pub struct AuthInfoService<S: Clone> {
    inner: S,
}

impl<S, ReqBody, Response> Service<http::Request<ReqBody>> for AuthInfoService<S>
where
    S: Service<http::Request<ReqBody>, Response = Response> + Clone,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: http::Request<ReqBody>) -> Self::Future {
        match request
            .extensions()
            .get::<TlsConnectInfo<TcpConnectInfo>>()
            .and_then(|i| i.peer_certs())
            .and_then(|peer_certs| peer_certs.first().and_then(extract_subject))
        {
            None => {}
            Some(tls_subject) => {
                let user_id = UserId::from(&tls_subject);
                request
                    .extensions_mut()
                    .insert(AuthInfo::Tls { subject: tls_subject, user_id });
            }
        }
        self.inner.call(request)
    }
}

fn extract_subject(cert: &CertificateDer<'_>) -> Option<ArcStr> {
    use x509_parser::prelude::*;
    if let Ok((_, cert)) = X509Certificate::from_der(cert) {
        cert.subject()
            .iter_common_name()
            .next()
            .and_then(|cn| cn.as_str().ok())
            .map(ArcStr::from)
    } else {
        None
    }
}

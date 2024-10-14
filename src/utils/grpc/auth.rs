use crate::UserId;
use anyhow::Result;
use arcstr::ArcStr;
use rustls_pki_types::CertificateDer;
use std::task::{Context, Poll};
use tonic::transport::server::{TcpConnectInfo, TlsConnectInfo};
use tower_layer::Layer;
use tower_service::Service;

#[derive(Clone)]
pub enum AuthInfo {
    Anonymous,
    Tls { email: ArcStr, user_id: UserId },
}

/// Read the TLS certificate attached to a request, and forward it's subject onto the wrapped services
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
            .and_then(|i| i.peer_certs()) // >>=
            .and_then(|peer_certs| peer_certs.first().map(|cert| cert.to_owned())) // >>=
            .and_then(|end_cert| extract_subject(&end_cert)) // return
        {
            None => {
                request.extensions_mut().insert(AuthInfo::Anonymous);
            }
            Some(tls_subject) => {
                let user_id = UserId::from(&tls_subject);
                request.extensions_mut().insert(AuthInfo::Tls {
                    email: tls_subject,
                    user_id,
                });
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

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;
use jsonwebtoken::{decode, Validation};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
}

pub struct JwtLogin;

impl<S, B> Transform<S> for JwtLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtLoginMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtLoginMiddleware { service })
    }
}
pub struct JwtLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service for JwtLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if req.path() == "/login" {
            Either::A(self.service.call(req))
        } else {
            if let Some(header_value) = req.headers().get(http::header::AUTHORIZATION) {
                let token = header_value.to_str().unwrap().replace("Bearer", "");
                let mut validation = Validation::default();
                validation.validate_exp = false; // our logins don't expire
                if let Ok(_) =
                    decode::<Claims>(&token.trim(), crate::TOKEN_SECRET.as_ref(), &validation)
                {
                    Either::A(self.service.call(req))
                } else {
                    Either::B(ok(
                        req.into_response(HttpResponse::Unauthorized().finish().into_body())
                    ))
                }
            } else {
                Either::B(ok(
                    req.into_response(HttpResponse::Unauthorized().finish().into_body())
                ))
            }
        }
    }
}

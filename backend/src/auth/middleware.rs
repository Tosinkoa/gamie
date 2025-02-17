use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub exp: usize,         // Expiration time
    pub token_type: String, // "access" or "refresh"
}

pub struct AuthMiddleware {
    jwt_secret: Rc<String>,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret: Rc::new(jwt_secret),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_secret: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("=== Auth Middleware Start ===");
        info!("Request path: {}", req.path());
        info!("Request method: {}", req.method());

        // Log all cookies
        match req.cookies() {
            Ok(cookies) => {
                info!(
                    "All cookies: {:?}",
                    cookies.iter().map(|c| c.name()).collect::<Vec<_>>()
                );
            }
            Err(e) => {
                error!("Error getting cookies: {}", e);
            }
        }

        let access_token = req.cookie("access_token");
        info!("Access token present: {}", access_token.is_some());

        let jwt_secret = self.jwt_secret.clone();

        if let Some(cookie) = access_token {
            info!(
                "Access token value (first 10 chars): {}",
                cookie.value().chars().take(10).collect::<String>()
            );
            let token = cookie.value();
            let mut validation = Validation::default();
            validation.validate_exp = true;

            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &validation,
            ) {
                Ok(token_data) => {
                    info!("Token decoded successfully");
                    info!("Token type: {}", token_data.claims.token_type);
                    info!("User ID from token: {}", token_data.claims.sub);

                    if token_data.claims.token_type != "access" {
                        error!("Invalid token type: {}", token_data.claims.token_type);
                        return Box::pin(async move {
                            Err(ErrorUnauthorized(json!({
                                "code": "INVALID_TOKEN_TYPE",
                                "message": "Invalid token type",
                                "field": None::<String>
                            })))
                        });
                    }

                    match ObjectId::parse_str(&token_data.claims.sub) {
                        Ok(user_id) => {
                            info!("User ID parsed successfully: {}", user_id);
                            req.extensions_mut().insert(user_id);
                            let fut = self.service.call(req);
                            info!("=== Auth Middleware End - Success ===");
                            return Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            });
                        }
                        Err(e) => {
                            error!("Failed to parse user ID: {}", e);
                            return Box::pin(async move {
                                Err(ErrorUnauthorized(json!({
                                    "code": "INVALID_USER_ID",
                                    "message": "Invalid user ID in token",
                                    "field": None::<String>
                                })))
                            });
                        }
                    }
                }
                Err(e) => {
                    error!("Token validation failed: {}", e);
                    return Box::pin(async move {
                        Err(ErrorUnauthorized(json!({
                            "code": "INVALID_TOKEN",
                            "message": "Invalid or expired token",
                            "field": None::<String>
                        })))
                    });
                }
            }
        } else {
            error!("No access_token cookie found");
            if let Ok(cookies) = req.cookies() {
                info!(
                    "Available cookies: {}",
                    cookies
                        .iter()
                        .map(|c| c.name())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }

        info!("=== Auth Middleware End - Unauthorized ===");
        Box::pin(async move {
            Err(ErrorUnauthorized(json!({
                "code": "UNAUTHORIZED",
                "message": "Authentication required",
                "field": None::<String>
            })))
        })
    }
}

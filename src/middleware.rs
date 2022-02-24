use crate::data::CacheControl;
use actix_service::{Service, Transform};
use actix_web::{
	dev::{ServiceRequest, ServiceResponse},
	http::header::{HeaderValue, CACHE_CONTROL},
	Error,
};
use futures::future::{ok, FutureExt, LocalBoxFuture, Ready};
use std::{
	rc::Rc,
	task::{Context, Poll},
};

#[derive(Clone)]
/// Middleware factory than instanciate CacheHeadersMiddleware
pub struct CacheHeaders(Rc<CacheControl>);

impl Default for CacheHeaders {
	fn default() -> Self {
		Self(Rc::new(CacheControl::default()))
	}
}

impl CacheHeaders {
	/// Construct a CacheHeader instance that forwards a CacheControl struct to all its middlewares
	pub fn new(cache: Option<CacheControl>) -> Self {
    	if let Some(cache) = cache {
			Self(Rc::new(cache))
    	} else {
        	Self(Rc::default())
    	}
	}
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for CacheHeaders
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type InitError = ();
	type Transform = CacheHeadersMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ok(CacheHeadersMiddleware {
			service,
			cache_control: self.0.clone(),
		})
	}
}

/// The middleware that contains the next service (future) and the cachecontrol
/// structure that drives its behavior
pub struct CacheHeadersMiddleware<S> {
	service: S,
	cache_control: Rc<CacheControl>,
}

impl<S, B> Service for CacheHeadersMiddleware<S>
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}

	fn call(&mut self, req: ServiceRequest) -> Self::Future {
		let cache_control = self.cache_control.clone();
		let path = req.path().to_owned();
		let fut = self.service.call(req);

		async move {
			let mut res = fut.await?;
			// set cache control value to the value specified in a rule in case of match
			if let Some(cache_control) = cache_control.get_value(&path) {
				res.headers_mut()
					.insert(CACHE_CONTROL, HeaderValue::from_str(cache_control)?);
			}
			Ok(res)
		}
		.boxed_local()
	}
}

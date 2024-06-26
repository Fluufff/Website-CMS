use crate::modules::core::helpers::auth::get_user_id_from_header;
use crate::modules::users::models::user::User;

use crate::errors::{AppError, AppErrorValue};
use crate::modules::core::middleware::state::AppState;

use actix_web::http::StatusCode;
use actix_web::HttpMessage;
use actix_web::{
	body::EitherBody,
	dev::{Service, ServiceRequest, ServiceResponse, Transform},
	http::Method,
	web::Data,
	Error, HttpRequest, HttpResponse,
};
use diesel::pg::PgConnection;
use futures::future::{ok, Ready};
use futures::Future;
use regex::Regex;
use serde_json::json;
use std::pin::Pin;
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type InitError = ();
	type Transform = AuthenticationMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ok(AuthenticationMiddleware { service })
	}
}

pub struct AuthenticationMiddleware<S> {
	service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;

	#[allow(clippy::type_complexity)] // TODO: want to remove allowness to skip and refactor somehow
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

	actix_web::dev::forward_ready!(service);

	fn call(&self, mut req: ServiceRequest) -> Self::Future {
		let is_verified = if should_skip_auth(&req) {
			true
		} else {
			set_auth_user(&mut req)
		};

		if is_verified {
			let fut = self.service.call(req);
			Box::pin(async move {
				let res = fut.await?.map_into_left_body();
				Ok(res)
			})
		} else {
			Box::pin(async move {
				let (req, _res) = req.into_parts();
				let res = HttpResponse::Unauthorized()
					.json(json!({
						"message": "Please authenticate",
						"status": 401,
						"identifier": Uuid::new_v4(),
						"code": "UNAUTHORIZED"
					}))
					.map_into_right_body();

				let srv = ServiceResponse::new(req, res);
				Ok(srv)
			})
		}
	}
}

fn should_skip_auth(req: &ServiceRequest) -> bool {
	let method = req.method();
	if Method::OPTIONS == *method {
		return true;
	}

	SKIP_AUTH_ROUTES
		.iter()
		.any(|route| route.matches_path_and_method(req.path(), req.method()))
}

fn set_auth_user(req: &mut ServiceRequest) -> bool {
	match fetch_user(req) {
		Ok(user) => {
			req.extensions_mut().insert(user);
			true
		}
		Err(err_msg) => {
			println!("Cannot fetch user {}", err_msg);
			false
		}
	}
}

fn fetch_user(req: &ServiceRequest) -> Result<User, &str> {
	let user_id = get_user_id_from_header(req)?;

	let conn = &mut req
		.app_data::<Data<AppState>>()
		.ok_or("Cannot get state.")
		.and_then(|state| state.get_conn().map_err(|_err| "Cannot get db connection."))?;

	find_auth_user(conn, user_id).map_err(|_err| "Cannot find user")
}

pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
	req.extensions()
		.get::<User>()
		.map(|user| user.to_owned()) // TODO: avoid copy
		.ok_or_else(|| {
			AppError::Unauthorized(AppErrorValue {
				message: "Unauthorized, please login".to_owned(),
				status: StatusCode::UNAUTHORIZED.as_u16(),
				identifier: Uuid::new_v4(),
				code: "UNAUTHORIZED".to_owned(),
			})
		})
}

struct SkipAuthRoute {
	path: Regex,
	method: Method,
}

impl SkipAuthRoute {
	fn matches_path_and_method(&self, path: &str, method: &Method) -> bool {
		self.matches_path(path) && self.matches_method(method)
	}

	fn matches_path(&self, path: &str) -> bool {
		let expect_path = &self.path;

		expect_path.is_match(path)
	}

	fn matches_method(&self, method: &Method) -> bool {
		self.method == method
	}
}

lazy_static! {
	static ref SKIP_AUTH_ROUTES: [SkipAuthRoute; 9] = [
		SkipAuthRoute {
			path: Regex::new(r"/admin-api/v1/auth/(.*)/login").unwrap(),
			method: Method::POST,
		},
		SkipAuthRoute {
			path: Regex::new(r"/admin-api/v1/status").unwrap(),
			method: Method::GET,
		},
		SkipAuthRoute {
			path: Regex::new(r"/admin-api/v1/config").unwrap(),
			method: Method::GET,
		},
		SkipAuthRoute {
			path: Regex::new(r"/admin-api/v1/auth/(.*)/callback").unwrap(),
			method: Method::POST,
		},
		SkipAuthRoute {
			path: Regex::new(r"/admin-api/v1/authentication-methods$").unwrap(),
			method: Method::GET,
		},
		SkipAuthRoute {
			path: Regex::new(r"/admin-api/v1/setup/*").unwrap(),
			method: Method::POST,
		},
		SkipAuthRoute {
			path: Regex::new(r"/admin-api/v1/sites/(.*)/storage-repositories/(.*)/files").unwrap(),
			method: Method::GET,
		},
		SkipAuthRoute {
			path: Regex::new(r"/docs/?(.*)").unwrap(),
			method: Method::GET
		},
		SkipAuthRoute {
			path: Regex::new(r"/api/?(.*)").unwrap(),
			method: Method::GET
		}
	];
}

// ================
// TODO: should inject this func
fn find_auth_user(conn: &mut PgConnection, user_id: Uuid) -> Result<User, AppError> {
	let user = User::find_one(conn, user_id)?;
	Ok(user)
}

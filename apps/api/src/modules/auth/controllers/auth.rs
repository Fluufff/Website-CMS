use crate::modules::auth::dto::request;
use crate::modules::auth::helpers::permissions::get_user_permissions;
use crate::modules::core::middleware::state::AppState;
use crate::modules::users::models::user::{UpdateUser, User};
use crate::modules::{auth::dto::response, core::middleware::auth};
use crate::utils::api::ApiResponse;
use actix_web::{get, put, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct MeQueryParams {
	site_id: Option<Uuid>,
}

#[utoipa::path(
	context_path = "/api/v1/auth",
	responses(
		(status = 200, body = AuthDTO),
		(status = 401, body = AppErrorValue, description = "Unauthorized")
	),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/me")]
pub async fn me(
	state: web::Data<AppState>,
	req: HttpRequest,
	query: web::Query<MeQueryParams>,
) -> ApiResponse {
	let conn = &mut state.get_conn()?;
	let user = auth::get_current_user(&req)?;
	let token = user.generate_token(None)?;
	let permissions = get_user_permissions(conn, user.id, query.site_id)?;
	let res = response::MeDTO::from((user, token, permissions));
	Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
	context_path = "/api/v1/auth",
    request_body = UpdateUserDTO,
	responses(
		(status = 200, body = AuthDTO),
		(status = 401, body = AppErrorValue, description = "Unauthorized")
	),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/me")]
pub async fn update(
	state: web::Data<AppState>,
	req: HttpRequest,
	form: web::Json<request::UpdateUserDTO>,
) -> ApiResponse {
	let conn = &mut state.get_conn()?;
	let current_user = auth::get_current_user(&req)?;
	let user = User::update(
		conn,
		current_user.id,
		UpdateUser {
			email: form.email.clone(),
			name: form.name.clone(),
			password: form.password.clone(),
			avatar: form.avatar.clone(),
			bio: form.bio.clone(),
		},
	)?;
	let token = user.generate_token(None)?;
	let sites = user.get_sites(conn)?;
	let roles = user.get_roles(conn)?;
	let res = response::AuthDTO::from((user, sites, roles, token));
	Ok(HttpResponse::Ok().json(res))
}

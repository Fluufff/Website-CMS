use super::super::dto::blocks::{request, response};
use crate::modules::auth::helpers::permissions::ensure_permission;
use crate::modules::content_components::dto::content_components::response::FieldWithContentComponentDTO;
use crate::modules::content_types::models::field::{FieldTypeEnum, UpdateField};
use crate::modules::content_types::models::field_config::FieldConfig;
use crate::modules::core::middleware::state::AppState;
use crate::modules::core::models::hal::HALPage;
use crate::utils::api::ApiResponse;
use crate::{errors::AppError, modules::content_types::models::field::FieldModel};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct FindPathParams {
	site_id: Uuid,
	content_type_id: Uuid,
	field_id: Uuid,
}

#[derive(Deserialize, IntoParams)]
pub struct FindOnePathParams {
	site_id: Uuid,
	block_field_id: Uuid,
	content_type_id: Uuid,
}

#[derive(Deserialize, IntoParams)]
pub struct FindAllQueryParams {
	page: Option<i64>,
	pagesize: Option<i64>,
}

#[utoipa::path(
	context_path = "/api/v1/sites/{site_id}/content-types/{content_type_id}/fields/{field_id}/blocks",
    request_body = CreateFieldDTO,
	responses(
		(status = 200, body = FieldDTO),
		(status = 401, body = AppErrorValue, description = "Unauthorized")
	),
    security(
        ("jwt_token" = [])
    ),
	params(FindPathParams)
)]
#[post("")]
pub async fn create(
	req: HttpRequest,
	state: web::Data<AppState>,
	form: web::Json<request::CreateBlockFieldDTO>,
	params: web::Path<FindPathParams>,
) -> Result<HttpResponse, AppError> {
	ensure_permission(
		&req,
		Some(params.site_id),
		format!("urn:dcm:content-types:{}", params.content_type_id),
		"sites::content-types:update",
	)?;
	let conn = &mut state.get_conn()?;
	let field = FieldModel::create(
		conn,
		params.site_id,
		params.field_id,
		form.content_component_id,
		None,
		FieldTypeEnum::ContentTypeField,
		&form.name,
	)?;

	let res = FieldWithContentComponentDTO::from(field);
	Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
	context_path = "/api/v1/sites/{site_id}/content-types/{content_type_id}/fields/{field_id}/blocks",
	responses(
		(status = 200, body = FieldsDTO),
		(status = 401, body = AppErrorValue, description = "Unauthorized")
	),
    security(
        ("jwt_token" = [])
    ),
	params(FindAllQueryParams)
)]
#[get("")]
pub async fn find_all(
	req: HttpRequest,
	state: web::Data<AppState>,
	query: web::Query<FindAllQueryParams>,
	params: web::Path<FindPathParams>,
) -> Result<HttpResponse, AppError> {
	ensure_permission(
		&req,
		Some(params.site_id),
		format!("urn:dcm:content-types:{}", params.content_type_id),
		"sites::content-types:update",
	)?;
	let conn = &mut state.get_conn()?;
	let page = query.page.unwrap_or(1);
	let pagesize = query.pagesize.unwrap_or(20);

	let (fields, total_elements) =
		FieldModel::find(conn, params.site_id, params.field_id, page, pagesize)?;

	let res = response::BlockFieldsDTO::from((
		fields,
		HALPage {
			number: page,
			size: pagesize,
			total_elements,
			total_pages: (total_elements / pagesize + (total_elements % pagesize).signum()).max(1),
		},
		params.site_id,
		params.content_type_id,
		params.field_id,
	));

	Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
	context_path = "/api/v1/sites/{site_id}/content-types/{content_type_id}/fields/{field_id}/blocks",
	responses(
		(status = 200, body = FieldModelDTO),
		(status = 401, body = AppErrorValue, description = "Unauthorized")
	),
    security(
        ("jwt_token" = [])
    ),
	params(FindPathParams)
)]
#[get("/{block_field_id}")]
pub async fn find_one(
	req: HttpRequest,
	state: web::Data<AppState>,
	params: web::Path<FindOnePathParams>,
) -> Result<HttpResponse, AppError> {
	ensure_permission(
		&req,
		Some(params.site_id),
		format!("urn:dcm:content-types:{}", params.content_type_id),
		"sites::content-types:update",
	)?;
	let conn = &mut state.get_conn()?;
	let field = FieldModel::find_one(conn, params.site_id, params.block_field_id)?;

	let res = FieldWithContentComponentDTO::from(field);
	Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
	context_path = "/api/v1/sites/{site_id}/content-types/{content_type_id}/fields/{field_id}/blocks",
    request_body = UpdateFieldDTO,
	responses(
		(status = 200, body = FieldWithContentComponentDTO),
		(status = 401, body = AppErrorValue, description = "Unauthorized")
	),
    security(
        ("jwt_token" = [])
    ),
	params(FindPathParams)
)]
#[put("/{block_field_id}")]
pub async fn update(
	req: HttpRequest,
	state: web::Data<AppState>,
	params: web::Path<FindOnePathParams>,
	form: web::Json<request::UpdateBlockFieldDTO>,
) -> ApiResponse {
	ensure_permission(
		&req,
		Some(params.site_id),
		format!("urn:dcm:content-types:{}", params.content_type_id),
		"sites::content-types:update",
	)?;
	let conn = &mut state.get_conn()?;

	FieldConfig::upsert(conn, params.block_field_id, form.config.clone())?;
	let field = FieldModel::update(
		conn,
		params.site_id,
		params.block_field_id,
		UpdateField {
			name: form.name.clone(),
			slug: form.slug.clone(),
			description: form.description.clone(),
			min: form.min.clone(),
			max: form.max.clone(),
			hidden: form.hidden.clone(),
			multi_language: form.multi_language.clone(),
			sequence_number: form.sequence_number.clone(),
			compartment_id: None,
			validation: form.validation.clone(),
		},
	)?;

	let res = FieldWithContentComponentDTO::from(field);
	Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
	context_path = "/api/v1/sites/{site_id}/content-types/{content_type_id}/fields/{field_id}/blocks",
	responses(
		(status = 204),
		(status = 401, body = AppErrorValue, description = "Unauthorized")
	),
    security(
        ("jwt_token" = [])
    ),
	params(FindPathParams)
)]
#[delete("/{block_field_id}")]
pub async fn remove(
	req: HttpRequest,
	state: web::Data<AppState>,
	params: web::Path<FindOnePathParams>,
) -> ApiResponse {
	ensure_permission(
		&req,
		Some(params.site_id),
		format!("urn:dcm:content-types:{}", params.content_type_id),
		"sites::content-types:update",
	)?;
	let conn = &mut state.get_conn()?;
	FieldModel::remove(conn, params.block_field_id)?;
	Ok(HttpResponse::NoContent().body(()))
}

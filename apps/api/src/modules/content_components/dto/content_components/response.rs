use crate::modules::{
	content_components::{
		enums::data_type::DataTypeEnum,
		models::content_component::{ContentComponent, PopulatedContentComponent},
	},
	content_types::models::{
		content_type::PopulatedBlockField, field::FieldModel, field_config::FieldConfigContent,
	},
	core::models::hal::{HALLinkList, HALPage},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, convert::From};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentComponentDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub internal: bool,
	pub description: Option<String>,
	pub component_name: String,
	pub data_type: DataTypeEnum,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}

impl From<ContentComponent> for ContentComponentDTO {
	fn from(content_component: ContentComponent) -> Self {
		Self {
			id: content_component.id,
			name: content_component.name,
			slug: content_component.slug,
			internal: content_component.internal,
			description: content_component.description,
			component_name: content_component.component_name,
			data_type: content_component.data_type,
			created_at: content_component.created_at,
			updated_at: content_component.updated_at,
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub compartment_id: Option<Uuid>,
	pub multi_language: bool,
	pub hidden: bool,
	pub min: i32,
	pub max: i32,
	pub sequence_number: Option<i32>,
	pub content_component: ContentComponentDTO,
	pub config: HashMap<String, Value>,
	pub validation: Option<Value>,
}

impl
	From<(
		FieldModel,
		PopulatedContentComponent,
		HashMap<String, FieldConfigContent>,
	)> for FieldDTO
{
	fn from(
		(field, content_component, config): (
			FieldModel,
			PopulatedContentComponent,
			HashMap<String, FieldConfigContent>,
		),
	) -> Self {
		Self {
			id: field.id,
			name: field.name,
			slug: field.slug,
			description: field.description,
			multi_language: field.multi_language,
			compartment_id: field.compartment_id,
			sequence_number: field.sequence_number,
			hidden: field.hidden,
			min: field.min,
			max: field.max,
			validation: field.validation,
			content_component: ContentComponentDTO::from(content_component.content_component),
			config: config
				.into_iter()
				.map(|(key, config_item)| {
					match config_item {
						FieldConfigContent::Text(text) => (key, Value::String(text)),
						FieldConfigContent::Json(json) => (key, json),
						FieldConfigContent::Fields(fields) => {
							// TODO: clean this up somehow 🤮
							let mapped_fields = fields
								.into_iter()
								.map(FieldDTO::from)
								.collect::<Vec<FieldDTO>>();
							let json = serde_json::to_string(&mapped_fields).unwrap();

							(key, serde_json::from_str(&json).unwrap())
						}
					}
				})
				.collect::<HashMap<_, _>>(),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldWithContentComponentDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub multi_language: bool,
	pub compartment_id: Option<Uuid>,
	pub hidden: bool,
	pub min: i32,
	pub max: i32,
	pub sequence_number: Option<i32>,
	pub content_component: ContentComponentWithFieldsDTO,
	pub blocks: Option<Vec<FieldWithContentComponentDTO>>,
	pub config: HashMap<String, Value>,
	pub validation: Option<Value>,
}

impl
	From<(
		FieldModel,
		PopulatedContentComponent,
		HashMap<String, FieldConfigContent>,
	)> for FieldWithContentComponentDTO
{
	fn from(
		(field, content_component, config): (
			FieldModel,
			PopulatedContentComponent,
			HashMap<String, FieldConfigContent>,
		),
	) -> Self {
		Self {
			id: field.id,
			name: field.name,
			slug: field.slug,
			description: field.description,
			multi_language: field.multi_language,
			hidden: field.hidden,
			compartment_id: field.compartment_id,
			sequence_number: field.sequence_number,
			min: field.min,
			max: field.max,
			validation: field.validation,
			blocks: None,
			content_component: ContentComponentWithFieldsDTO::from(content_component),
			config: config
				.into_iter()
				.map(|(key, config_item)| {
					match config_item {
						FieldConfigContent::Text(text) => (key, Value::String(text)),
						FieldConfigContent::Json(json) => (key, json),
						FieldConfigContent::Fields(fields) => {
							// TODO: clean this up somehow 🤮
							let mapped_fields = fields
								.into_iter()
								.map(FieldDTO::from)
								.collect::<Vec<FieldDTO>>();
							let json = serde_json::to_string(&mapped_fields).unwrap();

							(key, serde_json::from_str(&json).unwrap())
						}
					}
				})
				.collect::<HashMap<_, _>>(),
		}
	}
}

impl
	From<(
		FieldModel,
		PopulatedContentComponent,
		HashMap<String, FieldConfigContent>,
		Vec<PopulatedBlockField>,
	)> for FieldWithContentComponentDTO
{
	fn from(
		(field, content_component, config, blocks): (
			FieldModel,
			PopulatedContentComponent,
			HashMap<String, FieldConfigContent>,
			Vec<PopulatedBlockField>,
		),
	) -> Self {
		Self {
			id: field.id,
			name: field.name,
			slug: field.slug,
			description: field.description,
			multi_language: field.multi_language,
			hidden: field.hidden,
			compartment_id: field.compartment_id,
			sequence_number: field.sequence_number,
			min: field.min,
			max: field.max,
			validation: field.validation,
			content_component: ContentComponentWithFieldsDTO::from(content_component),
			blocks: Some(
				blocks
					.into_iter()
					.map(FieldWithContentComponentDTO::from)
					.collect(),
			),
			config: config
				.into_iter()
				.map(|(key, config_item)| {
					match config_item {
						FieldConfigContent::Text(text) => (key, Value::String(text)),
						FieldConfigContent::Json(json) => (key, json),
						FieldConfigContent::Fields(fields) => {
							// TODO: clean this up somehow 🤮
							let mapped_fields = fields
								.into_iter()
								.map(FieldDTO::from)
								.collect::<Vec<FieldDTO>>();
							let json = serde_json::to_string(&mapped_fields).unwrap();

							(key, serde_json::from_str(&json).unwrap())
						}
					}
				})
				.collect::<HashMap<_, _>>(),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentComponentWithFieldsDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub component_name: String,
	pub internal: bool,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub data_type: DataTypeEnum,
	pub configuration_fields: Vec<FieldWithContentComponentDTO>,
	pub fields: Vec<FieldWithContentComponentDTO>,
}

impl From<PopulatedContentComponent> for ContentComponentWithFieldsDTO {
	fn from(cc: PopulatedContentComponent) -> Self {
		Self {
			id: cc.content_component.id,
			name: cc.content_component.name,
			slug: cc.content_component.slug,
			description: cc.content_component.description,
			component_name: cc.content_component.component_name,
			internal: cc.content_component.internal,
			data_type: cc.content_component.data_type,
			created_at: cc.content_component.created_at,
			updated_at: cc.content_component.updated_at,
			configuration_fields: cc
				.configuration_fields
				.into_iter()
				.map(FieldWithContentComponentDTO::from)
				.collect(),
			fields: cc
				.fields
				.into_iter()
				.map(FieldWithContentComponentDTO::from)
				.collect(),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentComponentsEmbeddedDTO {
	pub content_components: Vec<ContentComponentDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct ContentComponentsDTO {
	pub _links: HALLinkList,
	pub _page: HALPage,
	pub _embedded: ContentComponentsEmbeddedDTO,
}

impl From<(Vec<ContentComponent>, HALPage, Uuid)> for ContentComponentsDTO {
	fn from((content_components, page, site_id): (Vec<ContentComponent>, HALPage, Uuid)) -> Self {
		Self {
			_links: HALLinkList::from((
				format!("/api/v1/sites/{}/content-components", site_id),
				&page,
			)),
			_embedded: ContentComponentsEmbeddedDTO {
				content_components: content_components
					.into_iter()
					.map(|content_component| ContentComponentDTO::from(content_component))
					.collect(),
			},
			_page: page,
		}
	}
}

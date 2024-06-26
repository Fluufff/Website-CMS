use crate::modules::{
	content::models::{
		content::Content, content_field::ContentField, content_revision::ContentRevision,
	},
	content_components::enums::data_type::DataTypeEnum,
	content_types::{
		dto::content_types::response::ContentTypeDTO, models::content_type::ContentType,
	},
	core::models::hal::{HALLinkList, HALPage},
	languages::models::language::Language,
	sites::dto::languages::response::LanguageDTO,
	workflows::{
		dto::workflow_states::response::WorkflowStateDTO, models::workflow_state::WorkflowState,
	},
};
use chrono::NaiveDateTime;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, convert::From, str::FromStr};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentWithFieldsDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub workflow_state_id: Uuid,
	pub content_type_id: Uuid,
	pub translation_id: Uuid,
	pub published: bool,
	pub deleted: bool,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub fields: HashMap<String, Option<Value>>,
	pub language: LanguageDTO,
	pub current_workflow_state: WorkflowStateDTO,
	pub revision_id: Uuid,
}

// TODO: dedupe
fn parse_field(
	content_id: Option<Uuid>,
	translation_id: Uuid,
	field: &ContentField,
	all_fields: Vec<ContentField>,
	populate: bool,
) -> Option<Value> {
	match field.data_type {
		DataTypeEnum::TEXT | DataTypeEnum::NUMBER | DataTypeEnum::BOOLEAN => field.value.clone(),
		DataTypeEnum::REFERENCE => {
			if field.value.is_none()
				|| field.value.as_ref().unwrap()["contentId"]
					.as_str()
					.is_none() || field.value.as_ref().unwrap()["translationId"]
				.as_str()
				.is_none() || !populate
			{
				let json = serde_json::to_string(&field.value).unwrap();
				return serde_json::from_str(&json).unwrap();
			}

			let referenced_fields = parse_object_fields(
				Some(
					Uuid::from_str(field.value.as_ref().unwrap()["contentId"].as_str().unwrap())
						.expect("No uuid"),
				),
				Uuid::from_str(
					field.value.as_ref().unwrap()["translationId"]
						.as_str()
						.unwrap(),
				)
				.expect("No uuid"),
				None,
				all_fields.clone(),
				populate,
			);

			// TODO: clean this up somehow 🤮
			let json = serde_json::to_string(&json!({
				"contentId": field.value.as_ref().unwrap()["contentId"],
				"translationId": field.value.as_ref().unwrap()["translationid"],
				"fields": &referenced_fields
			}))
			.unwrap();
			serde_json::from_str(&json).unwrap()
		}
		DataTypeEnum::ARRAY => {
			let sub_fields = parse_array_fields(
				content_id,
				translation_id,
				Some(field.id),
				all_fields.clone(),
				populate,
			);

			// TODO: clean this up somehow 🤮
			let json = serde_json::to_string(&sub_fields).unwrap();
			serde_json::from_str(&json).unwrap()
		}
		DataTypeEnum::OBJECT => {
			let sub_fields = parse_object_fields(
				content_id,
				translation_id,
				Some(field.id),
				all_fields.clone(),
				populate,
			);

			// TODO: clean this up somehow 🤮
			let json = serde_json::to_string(&sub_fields).unwrap();
			serde_json::from_str(&json).unwrap()
		}
		DataTypeEnum::BLOCK => {
			// One block
			let sub_fields = parse_object_fields(
				content_id,
				translation_id,
				Some(field.id),
				all_fields.clone(),
				populate,
			);

			// TODO: clean this up somehow 🤮
			let json = serde_json::to_string(&json!({
			"block":  field.value.to_owned(),
				"fields": &sub_fields
			}))
			.unwrap();
			serde_json::from_str(&json).unwrap()
		}
	}
}

fn parse_array_fields(
	content_id: Option<Uuid>,
	translation_id: Uuid,
	parent_id: Option<Uuid>,
	fields: Vec<ContentField>,
	populate: bool,
) -> Vec<Option<Value>> {
	let filtered_fields = fields
		.iter()
		.filter(|field| {
			field.parent_id == parent_id
				&& vec![content_id, Some(translation_id)].contains(&Some(field.source_id))
		})
		.collect::<Vec<&ContentField>>();

	filtered_fields
		.iter()
		.sorted_by(|field_a, field_b| Ord::cmp(&field_a.sequence_number, &field_b.sequence_number))
		.map(|field| parse_field(content_id, translation_id, field, fields.clone(), populate))
		.collect()
}

pub fn parse_object_fields(
	content_id: Option<Uuid>,
	translation_id: Uuid,
	parent_id: Option<Uuid>,
	fields: Vec<ContentField>,
	populate: bool,
) -> HashMap<String, Option<Value>> {
	let fields = fields
		.iter()
		.filter(|field| {
			field.parent_id == parent_id
				&& vec![content_id, Some(translation_id)].contains(&Some(field.source_id))
		})
		.map(|field| {
			let parsed_field =
				parse_field(content_id, translation_id, field, fields.clone(), populate);
			(field.name.clone(), parsed_field)
		})
		.collect::<HashMap<_, _>>();

	fields
}

impl
	From<(
		Content,
		ContentRevision,
		Vec<ContentField>,
		Language,
		WorkflowState,
		bool,
	)> for ContentWithFieldsDTO
{
	fn from(
		(content, revision, fields, language, workflow_state, populate): (
			Content,
			ContentRevision,
			Vec<ContentField>,
			Language,
			WorkflowState,
			bool,
		),
	) -> Self {
		Self {
			id: content.id,
			name: content.name,
			slug: content.slug,
			workflow_state_id: content.workflow_state_id,
			content_type_id: content.content_type_id,
			translation_id: content.translation_id,
			published: content.published,
			deleted: content.deleted,
			created_at: content.created_at,
			updated_at: content.updated_at,
			fields: parse_object_fields(
				Some(revision.id),
				revision.revision_translation_id,
				None,
				fields,
				populate,
			),
			language: LanguageDTO::from(language),
			current_workflow_state: WorkflowStateDTO::from(workflow_state),
			revision_id: revision.id,
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicContentTranslationsDTO {
	pub id: Uuid,
	pub slug: String,
	pub language: String,
}

impl From<(Content, Language)> for PublicContentTranslationsDTO {
	fn from((content, language): (Content, Language)) -> Self {
		Self {
			id: content.id,
			slug: content.slug,
			language: language.key,
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicContentDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub fields: HashMap<String, Option<Value>>,
	pub language: String,
	pub translations: Vec<PublicContentTranslationsDTO>,
}

impl
	From<(
		Content,
		ContentRevision,
		Vec<ContentField>,
		Language,
		Vec<(Content, Language)>,
		bool,
	)> for PublicContentDTO
{
	fn from(
		(content, revision, fields, language, translations, populate): (
			Content,
			ContentRevision,
			Vec<ContentField>,
			Language,
			Vec<(Content, Language)>,
			bool,
		),
	) -> Self {
		Self {
			id: content.id,
			name: content.name,
			slug: content.slug,
			created_at: content.created_at,
			updated_at: content.updated_at,
			fields: parse_object_fields(
				Some(revision.id),
				revision.revision_translation_id,
				None,
				fields,
				populate,
			),
			language: language.key,
			translations: translations
				.into_iter()
				.map(PublicContentTranslationsDTO::from)
				.collect(),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentDefaultValuesDTO {
	pub fields: HashMap<String, Option<Value>>,
}

impl From<(Option<Uuid>, Uuid, Vec<ContentField>, bool)> for ContentDefaultValuesDTO {
	fn from(
		(content_id, translation_id, fields, populate): (
			Option<Uuid>,
			Uuid,
			Vec<ContentField>,
			bool,
		),
	) -> Self {
		Self {
			fields: parse_object_fields(content_id, translation_id, None, fields, populate),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub workflow_state_id: Uuid,
	pub content_type_id: Uuid,
	pub translation_id: Uuid,
	pub published: bool,
	pub deleted: bool,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub language: LanguageDTO,
	pub content_type: ContentTypeDTO,
	pub current_workflow_state: WorkflowStateDTO,
}

impl From<(Content, Language, ContentType, WorkflowState)> for ContentDTO {
	fn from(
		(content, language, content_type, workflow_state): (
			Content,
			Language,
			ContentType,
			WorkflowState,
		),
	) -> Self {
		Self {
			id: content.id,
			name: content.name,
			slug: content.slug,
			workflow_state_id: content.workflow_state_id,
			translation_id: content.translation_id,
			content_type_id: content.content_type_id,
			published: content.published,
			deleted: content.deleted,
			created_at: content.created_at,
			updated_at: content.updated_at,
			language: LanguageDTO::from(language),
			content_type: ContentTypeDTO::from(content_type),
			current_workflow_state: WorkflowStateDTO::from(workflow_state),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentListEmbeddedDTO {
	pub content: Vec<ContentDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct ContentListDTO {
	pub _links: HALLinkList,
	pub _page: HALPage,
	pub _embedded: ContentListEmbeddedDTO,
}

impl
	From<(
		Vec<(Content, Language, ContentType, WorkflowState)>,
		HALPage,
		Uuid,
	)> for ContentListDTO
{
	fn from(
		(content, page, site_id): (
			Vec<(Content, Language, ContentType, WorkflowState)>,
			HALPage,
			Uuid,
		),
	) -> Self {
		Self {
			_links: HALLinkList::from((format!("/api/v1/sites/{}/content", site_id), &page)),
			_embedded: ContentListEmbeddedDTO {
				content: content.into_iter().map(ContentDTO::from).collect(),
			},
			_page: page,
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicContentListEmbeddedDTO {
	pub content: Vec<PublicContentDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct PublicContentListDTO {
	pub _links: HALLinkList,
	pub _page: HALPage,
	pub _embedded: PublicContentListEmbeddedDTO,
}

impl
	From<(
		Vec<(
			Content,
			ContentRevision,
			Vec<ContentField>,
			Language,
			Vec<(Content, Language)>,
		)>,
		HALPage,
		bool,
		Uuid,
	)> for PublicContentListDTO
{
	fn from(
		(content, page, populated, site_id): (
			Vec<(
				Content,
				ContentRevision,
				Vec<ContentField>,
				Language,
				Vec<(Content, Language)>,
			)>,
			HALPage,
			bool,
			Uuid,
		),
	) -> Self {
		Self {
			_links: HALLinkList::from((format!("/api/v1/sites/{}/content", site_id), &page)),
			_embedded: PublicContentListEmbeddedDTO {
				content: content
					.into_iter()
					.map(|(content, revision, fields, language, translations)| {
						PublicContentDTO::from((
							content,
							revision,
							fields,
							language,
							translations,
							populated,
						))
					})
					.collect(),
			},
			_page: page,
		}
	}
}

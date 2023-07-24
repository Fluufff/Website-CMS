use diesel::prelude::*;
use slug::slugify;
use serde::Deserialize;
use crate::modules::content_components::models::content_component::ContentComponent;
use crate::modules::content_types::models::content_type::ContentType;
use uuid::Uuid;

use crate::errors::AppError;
use crate::schema::{fields, content_components};

use super::field_config::FieldConfig;

#[derive(Selectable, Queryable, Debug, Identifiable, Associations, Clone)]
#[diesel(belongs_to(ContentComponent, foreign_key = content_component_id))]
#[diesel(belongs_to(ContentType, foreign_key = parent_id))]
#[diesel(table_name = fields)]
#[diesel(primary_key(id))]
pub struct FieldModel {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub parent_id: Uuid,
	pub content_component_id: Uuid,
}

impl FieldModel {
	pub fn create(
		conn: &mut PgConnection,
		site_id: Uuid,
		parent_id: Uuid,
		content_component_id: Uuid,
		name: &String,
	) -> Result<(Self, (ContentComponent, Vec<(FieldModel, ContentComponent, Vec<FieldConfig>)>), Vec<FieldConfig>), AppError> {
		let field = diesel::insert_into(fields::table)
			.values(CreateField {
				name: name.to_owned(),
				slug: slugify(name),
				parent_id,
				content_component_id,
			})
			.returning(FieldModel::as_returning())
			.get_result(conn)?;

		let result = Self::find_one(conn, site_id, field.id)?;

		Ok(result)
	}

	pub fn find_one(conn: &mut PgConnection, _site_id: Uuid, id: Uuid) -> Result<(Self, (ContentComponent, Vec<(FieldModel, ContentComponent, Vec<FieldConfig>)>), Vec<FieldConfig>), AppError> {
		let field = fields::table.find(id).first::<Self>(conn)?;

		let field_config = FieldConfig::belonging_to(&field)
			.select(FieldConfig::as_select())
			.load::<FieldConfig>(conn)?;

		let content_component = ContentComponent::find_one_with_fields(conn, None, field.content_component_id)?;
		
		Ok((field, content_component, field_config))
	}

	pub fn find(
		conn: &mut PgConnection,
		site_id: Uuid,
		parent_id: Uuid,
		page: i64,
		pagesize: i64,
	) -> Result<(Vec<(Self, ContentComponent, Vec<FieldConfig>)>, i64), AppError> {
		let fields = fields::table
			.filter(fields::parent_id.eq(parent_id))
			.offset((page - 1) * pagesize)
			.limit(pagesize)
			.select(FieldModel::as_select())
			.load::<FieldModel>(conn)?;

		let all_content_components = content_components::table
			.select(ContentComponent::as_select())
			.load::<ContentComponent>(conn)?;

		let field_config = FieldConfig::belonging_to(&fields)
			.select(FieldConfig::as_select())
			.load::<FieldConfig>(conn)?;

		let grouped_config: Vec<Vec<FieldConfig>> = field_config.grouped_by(&fields);
		let fields_with_config: Vec<(FieldModel, ContentComponent, Vec<FieldConfig>)> = fields
			.into_iter()
			.zip(grouped_config)
			.map(|(field, field_configs)| {
				let content_component = all_content_components.iter().find(|cp| cp.id == field.content_component_id).map(|cp| cp.to_owned());
				(field, content_component.unwrap(), field_configs)
			})
			.collect();

		let total_elements = fields::table.filter(fields::parent_id.eq(parent_id)).count().get_result::<i64>(conn)?;

		Ok((fields_with_config, total_elements))
	}

	pub fn update(
		conn: &mut PgConnection,
		site_id: Uuid,
		id: Uuid,
		changeset: UpdateField,
	) -> Result<(Self, (ContentComponent, Vec<(FieldModel, ContentComponent, Vec<FieldConfig>)>), Vec<FieldConfig>), AppError> {
		let target = fields::table.find(id);
		diesel::update(target)
			.set(changeset)
			.get_result::<Self>(conn)?;
	
		let field = Self::find_one(conn, site_id, id)?;

		Ok(field)
	}

	pub fn remove(conn: &mut PgConnection, field_id: Uuid) -> Result<(), AppError> {
		// TODO: delete config aswell
		let target = fields::table.filter(fields::id.eq(field_id));
		diesel::delete(target).get_result::<FieldModel>(conn)?;
		Ok(())
	}
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = fields)]
pub struct CreateField {
	name: String,
	slug: String,
	parent_id: Uuid,
	content_component_id: Uuid,
}

#[derive(AsChangeset, Insertable, Debug, Deserialize)]
#[diesel(table_name = fields)]
pub struct UpdateField {
	pub name: Option<String>
}

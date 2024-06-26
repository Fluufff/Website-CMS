use crate::modules::{
	core::models::hal::{HALLinkList, HALPage},
	iam_policies::models::{iam_policy::IAMPolicy, permission::Permission},
	languages::models::language::Language,
	roles::{dto::response::RoleWithPoliciesWithPermissionsDTO, models::role::Role},
	sites::models::site::Site,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::From;
use utoipa::ToSchema;
use uuid::Uuid;

use super::languages::response::LanguageDTO;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SiteDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}

impl From<Site> for SiteDTO {
	fn from(site: Site) -> Self {
		Self {
			id: site.id,
			name: site.name,
			slug: site.slug,
			created_at: site.created_at,
			updated_at: site.updated_at,
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SiteWithLanguagesDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub languages: Vec<LanguageDTO>,
	pub has_permission: Option<bool>,
}

impl From<(Site, Option<bool>, Vec<Language>)> for SiteWithLanguagesDTO {
	fn from((site, has_permission, languages): (Site, Option<bool>, Vec<Language>)) -> Self {
		Self {
			id: site.id,
			name: site.name,
			slug: site.slug,
			created_at: site.created_at,
			updated_at: site.updated_at,
			has_permission,
			languages: languages
				.into_iter()
				.map(|language| LanguageDTO::from(language))
				.collect(),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SiteWithRolesDTO {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub roles: Vec<RoleWithPoliciesWithPermissionsDTO>,
	pub languages: Vec<LanguageDTO>,
}
impl
	From<(
		Site,
		Vec<(Role, Vec<(IAMPolicy, Vec<(Permission, Vec<String>)>)>)>,
		Vec<Language>,
	)> for SiteWithRolesDTO
{
	fn from(
		(site, roles, languages): (
			Site,
			Vec<(Role, Vec<(IAMPolicy, Vec<(Permission, Vec<String>)>)>)>,
			Vec<Language>,
		),
	) -> Self {
		Self {
			id: site.id,
			name: site.name,
			slug: site.slug,
			created_at: site.created_at,
			updated_at: site.updated_at,
			roles: roles
				.into_iter()
				.map(|role| RoleWithPoliciesWithPermissionsDTO::from(role))
				.collect(),
			languages: languages
				.into_iter()
				.map(|language| LanguageDTO::from(language))
				.collect(),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct SitesEmbeddedDTO {
	pub sites: Vec<SiteWithLanguagesDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct SitesDTO {
	pub _links: HALLinkList,
	pub _page: HALPage,
	pub _embedded: SitesEmbeddedDTO,
}

impl From<(Vec<(Site, Option<bool>, Vec<Language>)>, HALPage)> for SitesDTO {
	fn from((sites, page): (Vec<(Site, Option<bool>, Vec<Language>)>, HALPage)) -> Self {
		Self {
			_links: HALLinkList::from(("/api/v1/sites".to_owned(), &page)),
			_embedded: SitesEmbeddedDTO {
				sites: sites
					.into_iter()
					.map(|site| SiteWithLanguagesDTO::from(site))
					.collect(),
			},
			_page: page,
		}
	}
}

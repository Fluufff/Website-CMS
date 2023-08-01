use std::io::Write;

use diesel::{FromSqlRow, AsExpression};
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use serde::{Deserialize, Serialize};
use diesel::serialize::{self, IsNull, Output, ToSql};

use crate::schema::sql_types::DataTypes;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Clone, Deserialize, Serialize)]
#[diesel(sql_type = DataTypes)]
pub enum DataTypeEnum {
	TEXT,
	ARRAY,
	OBJECT,
	NUMBER,
	BOOLEAN,
	REFERENCE
}

impl ToSql<DataTypes, Pg> for DataTypeEnum {
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
		match *self {
			DataTypeEnum::TEXT => out.write_all(b"TEXT")?,
			DataTypeEnum::ARRAY => out.write_all(b"ARRAY")?,
			DataTypeEnum::OBJECT => out.write_all(b"OBJECT")?,
			DataTypeEnum::NUMBER => out.write_all(b"NUMBER")?,
			DataTypeEnum::BOOLEAN => out.write_all(b"BOOLEAN")?,
			DataTypeEnum::REFERENCE => out.write_all(b"REFERENCE")?,
		}
		Ok(IsNull::No)
	}
}

impl FromSql<DataTypes, Pg> for DataTypeEnum {
	fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
		match bytes.as_bytes() {
			b"TEXT" => Ok(DataTypeEnum::TEXT),
			b"ARRAY" => Ok(DataTypeEnum::ARRAY),
			b"OBJECT" => Ok(DataTypeEnum::OBJECT),
			b"NUMBER" => Ok(DataTypeEnum::NUMBER),
			b"BOOLEAN" => Ok(DataTypeEnum::BOOLEAN),
			b"REFERENCE" => Ok(DataTypeEnum::REFERENCE),
			_ => Err("Unrecognized enum variant".into()),
		}
	}
}

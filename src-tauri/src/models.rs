use diesel::prelude::*;
use serde::Serialize;

use crate::schema::*;

#[derive(Queryable, Debug, Serialize, Clone, Insertable, Selectable, QueryableByName)]
#[diesel(table_name = numbers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Number {
    pub id: Option<i32>,
    pub digits: String,
    pub is_valid: bool,
    pub international: Option<String>,
    pub national: Option<String>,
    pub rfc3966: Option<String>,
    pub e164: Option<String>,
    pub email: Option<String>,
    pub carrier: Option<String>,
}

use diesel::prelude::*;
use serde::Serialize;
use std::clone::Clone;

use crate::dao::IGetRecordId;

use crate::schema::org;

#[derive(AsChangeset, Clone, Debug, Default, Insertable, Queryable, Serialize)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = org)]
pub struct Org {
    pub id: String,
    pub name: String,
    pub created: i64,
}

impl IGetRecordId for Org {
    fn get_record_id(&self) -> String {
        self.id.clone()
    }
}

use diesel;
use diesel::prelude::*;
use std::clone::Clone;

use crate::model::org::Org;
use crate::schema::org;

pub fn create(
    connection: &mut MysqlConnection,
    record: &Org,
) -> Result<Org, diesel::result::Error> {
    let id = record.id.clone();
    diesel::insert_into(org::table)
        .values(record)
        .execute(connection)?;

    match org::table
        .filter(org::id.eq(id))
        .load(connection)?
        .into_iter()
        .next()
    {
        Some(val) => Ok(val),
        None => Err(diesel::result::Error::NotFound),
    }
}

// pub fn read(connection: &mut MysqlConnection) -> Vec<Org> {
//   org::table.load::<Org>(connection).unwrap()
// }

// pub fn read_by_ids(connection: &mut MysqlConnection, ids: &Vec<String>) -> Vec<Org> {
//   org::table
//     .filter(org::id.eq_any(ids))
//     .load::<Org>(connection)
//     .expect("Error reading orgs")
// }

pub fn get_by_id(connection: &mut MysqlConnection, id: &str) -> Option<Org> {
    let results = org::table
        .filter(org::id.eq(id))
        .limit(1)
        .load::<Org>(connection)
        .expect("Error reading orgs");

    results.into_iter().next()
}

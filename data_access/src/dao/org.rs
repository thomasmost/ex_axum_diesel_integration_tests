use super::{ICreate, IGetByField};
use crate::{db::Pool, ident::PrimaryKey, model::org::Org};

#[macro_export]
macro_rules! unpool {
    ( $e:expr ) => {
        match $e.0.get() {
            Ok(x) => x,
            Err(_) => {
                panic!()
            }
        }
    };
}

pub struct OrgDao<'r> {
    pool: &'r Pool,
}

impl<'r> OrgDao<'r> {
    pub fn new(pool: &Pool) -> OrgDao {
        OrgDao { pool }
    }
}

impl<'r> IGetByField<Org, PrimaryKey> for OrgDao<'r> {
    fn get_by_field(&self, id: &PrimaryKey) -> Option<Org> {
        let connection = &mut unpool!(self.pool);
        crate::dal::org::get_by_id(connection, &id.0)
    }
}

impl<'r> ICreate<Org> for OrgDao<'r> {
    fn create(&mut self, record: &Org) -> Result<Org, diesel::result::Error> {
        let connection = &mut unpool!(self.pool);
        crate::dal::org::create(connection, record)
    }
}

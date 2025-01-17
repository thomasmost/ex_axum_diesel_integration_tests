use std::collections::HashMap;

pub mod org;

pub trait ICreate<T> {
    fn create(&mut self, record: &T) -> Result<T, diesel::result::Error>;
}

pub trait IGetByField<T, F> {
    fn get_by_field(&self, field: &F) -> Option<T>;
}

pub trait IGetByFieldForOrg<T, F> {
    fn get_by_field_for_org(&self, org_id: &str, field: &F) -> Option<T>;
}

pub trait IGetLikeField<T, F> {
    fn get_like_field(&self, field: &F) -> Vec<T>;
}

pub trait IGetManyByField<T, F> {
    fn get_many_by_field(&self, field: &F) -> Vec<T>;
}

pub trait IGetManyByFieldSet<T, F> {
    fn get_many_by_field_set(&self, field_set: &Vec<F>) -> Vec<T>;
}

pub trait IList<T> {
    fn list(&self, cursor: &Option<String>) -> Vec<T>;
}

pub trait IQueryField<T, F> {
    fn query_by_field(&self, field: &F) -> Vec<T>;
}

pub trait IUpdate<T> {
    fn update(&mut self, record: &T) -> Result<T, ()>;
}

pub trait IDelete<T> {
    fn delete(&mut self, record: T) -> Result<T, ()>;
}

/// DATA ACCESS HELPERS

pub trait IGetRecordId<I = String> {
    fn get_record_id(&self) -> I;
}

pub fn map_by_id<'a, I, T>(records: impl IntoIterator<Item = T>) -> HashMap<I, T>
where
    I: std::hash::Hash + std::cmp::Eq + std::cmp::PartialEq,
    T: IGetRecordId<I>,
{
    let mut records_by_id: HashMap<I, T> = HashMap::new();
    for record in records.into_iter() {
        let id = record.get_record_id();
        records_by_id.insert(id, record);
    }
    records_by_id
}

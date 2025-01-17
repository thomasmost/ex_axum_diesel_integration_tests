use std::time::SystemTime;

use axum::extract::{Json, State};
use data_access::{dao::ICreate, db::Pool, model::org::Org};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrgRegistrationRequest {
    pub name: String,
}

pub async fn register_org(
    State(pool): State<Pool>,
    Json(req): Json<OrgRegistrationRequest>,
) -> http::StatusCode {
    let org_dao = &mut data_access::dao::org::OrgDao::new(&pool);
    let new_org = Org {
        id: data_access::ident::generate(data_access::ident::Model::Org),
        name: req.name,
        created: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    };
    match org_dao.create(&new_org) {
        Ok(_) => http::StatusCode::CREATED,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

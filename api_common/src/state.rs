use axum::extract::FromRef;
use data_access::db::Pool;

#[derive(Debug, Clone, Default)]
pub struct ServiceApplicationState {
    pub pool: Pool,
}

impl FromRef<ServiceApplicationState> for Pool {
    fn from_ref(app_state: &ServiceApplicationState) -> Pool {
        app_state.pool.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServiceApplicationStateLayer {
    pub pool: Pool,
}

use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use std::env;

type R2D2Pool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub use tracing::info;

#[derive(Clone, Debug)]
pub struct Pool(pub R2D2Pool);

impl Default for Pool {
    fn default() -> Self {
        Pool(init_pool())
    }
}

pub fn init_pool() -> R2D2Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url());
    let pool = R2D2Pool::builder()
        .max_size(32)
        .min_idle(Some(8))
        .build(manager)
        .expect("Database Pool failed to initialize");
    info!(
        "Initialized db pool with min_idle {:?} and max_size {}",
        pool.min_idle(),
        pool.max_size()
    );
    pool
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub mod db {
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::MysqlConnection;
    pub fn get_pool() -> crate::db::Pool {
        let manager = ConnectionManager::<MysqlConnection>::new(
            "mysql://root:@0.0.0.0:3306/ex_axum_diesel_integration",
        );

        // In case of local password, use the following, where "password" is replaced by the actual password:
        // let manager = ConnectionManager::<MysqlConnection>::new("mysql://root:password@localhost:3306/test_db");

        crate::db::Pool(
            Pool::builder()
                .max_size(32)
                .min_idle(Some(1))
                .build(manager)
                .expect("Database Pool failed to initialize"),
        )
    }

    pub fn clear_org_tables(conn: &mut MysqlConnection) -> () {
        use crate::schema::org;
        use diesel::prelude::*;

        diesel::delete(org::table)
            .execute(conn)
            .expect("Failed to clear org table");
    }
}

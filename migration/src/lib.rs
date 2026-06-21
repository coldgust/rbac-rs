use dotenvy::dotenv;
pub use sea_orm_migration::prelude::*;

mod m20260619_000001_create_table;
mod m20260620_000001_init_data;
mod m20260621_000001_init_super_admin;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        dotenv().ok();
        vec![
            Box::new(m20260619_000001_create_table::Migration),
            Box::new(m20260620_000001_init_data::Migration),
            Box::new(m20260621_000001_init_super_admin::Migration),
        ]
    }
}

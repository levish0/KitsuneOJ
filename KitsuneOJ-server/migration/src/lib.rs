pub use sea_orm_migration::prelude::*;
mod common;

mod m20250908_122528_create_users;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250908_122528_create_users::Migration),
        ]
    }
}

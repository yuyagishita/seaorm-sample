pub use sea_orm_migration::prelude::*;

mod m20220601_220000_create_documents;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220601_220000_create_documents::Migration)]
    }
}

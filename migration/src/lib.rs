pub use sea_orm_migration::prelude::*;

mod m20241219_000001_create_registration_table;
mod m20241219_000002_create_nonce_table;
mod m20241219_000003_create_tls_cert_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241219_000001_create_registration_table::Migration),
            Box::new(m20241219_000002_create_nonce_table::Migration),
            Box::new(m20241219_000003_create_tls_cert_table::Migration),
        ]
    }
}

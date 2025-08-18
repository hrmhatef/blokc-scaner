pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20250818_000001_create_blocks_table;
mod m20250818_000002_create_transactions_table;
mod m20250818_000003_create_events_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250818_000001_create_blocks_table::Migration),
            Box::new(m20250818_000002_create_transactions_table::Migration),
            Box::new(m20250818_000003_create_events_table::Migration),
        ]
    }
}

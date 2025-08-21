use crate::m20250818_000002_create_transactions_table::Transactions;

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250818_000003_create_events_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .col(
                        ColumnDef::new(Events::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Events::From).string().not_null())
                    .col(ColumnDef::new(Events::To).string().not_null())
                    .col(ColumnDef::new(Events::Value).string().not_null())
                    .col(ColumnDef::new(Events::LogIndex).big_unsigned().not_null())
                    .col(ColumnDef::new(Events::TxId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-events-transactions_id")
                            .from(Events::Table, Events::TxId)
                            .to(Transactions::Table, Transactions::Id),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    TxId,
    From,
    To,
    Value,
    LogIndex,
}

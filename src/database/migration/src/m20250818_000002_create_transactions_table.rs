use crate::m20250818_000001_create_blocks_table::Blocks;

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250818_000002_create_transactions_table"
    }
}


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .col(
                        ColumnDef::new(Transactions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk-blocks-transactions_id")
                        .from(Blocks::Table, Blocks::Id)
                        .to(Transactions::Table, Transactions::BlockId),
                    )
                    .col(ColumnDef::new(Transactions::Hash).string().not_null())
                    .col(ColumnDef::new(Transactions::TagIndex).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transactions {
    Table,
    Id,
    BlockId,
    Hash,
    TagIndex
}

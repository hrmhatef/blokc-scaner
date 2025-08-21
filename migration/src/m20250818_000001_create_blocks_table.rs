use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250818_000001_create_blocks_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blocks::Table)
                    .col(
                        ColumnDef::new(Blocks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Blocks::BlockNumber).big_unsigned().not_null())
                    .col(ColumnDef::new(Blocks::Hash).string().not_null())
                    .col(ColumnDef::new(Blocks::Timestamp).big_unsigned())
                    .col(ColumnDef::new(Blocks::Tag).string().not_null())
                    .col(ColumnDef::new(Blocks::IsRemoved).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blocks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Blocks {
    Table,
    Id,
    BlockNumber,
    Hash,
    Timestamp,
    Tag,
    IsRemoved
}

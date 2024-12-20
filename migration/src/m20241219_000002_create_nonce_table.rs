use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Nonce::Table)
                    .if_not_exists()
                    .col(pk_auto(Nonce::Id))
                    .col(string(Nonce::Uuid).unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Nonce::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Nonce {
    Table,
    Id,
    Uuid,
}

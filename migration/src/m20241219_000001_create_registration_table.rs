use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Registration::Table)
                    .if_not_exists()
                    .col(pk_auto(Registration::Id))
                    .col(string(Registration::Uuid).unique_key())
                    .col(string(Registration::Issuer))
                    .col(string(Registration::PlatformAuthEndpoint))
                    .col(string(Registration::PlatformJwksUri))
                    .col(string(Registration::ClientId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Registration::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Registration {
    Table,
    Id,
    Uuid,
    Issuer,
    PlatformAuthEndpoint,
    PlatformJwksUri,
    ClientId,
}

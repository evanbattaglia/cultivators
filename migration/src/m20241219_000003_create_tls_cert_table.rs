use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TlsCert::Table)
                    .if_not_exists()
                    .col(pk_auto(TlsCert::Id))
                    .col(string(TlsCert::Domain).unique_key())
                    .col(timestamp(TlsCert::CreatedAt))
                    .col(text(TlsCert::CertPem))
                    .col(text(TlsCert::KeyPem))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TlsCert::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TlsCert {
    Table,
    Id,
    Domain,
    CreatedAt,
    CertPem,
    KeyPem,
}

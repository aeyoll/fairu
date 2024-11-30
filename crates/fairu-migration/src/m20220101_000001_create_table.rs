use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Fairu::Table)
                    .if_not_exists()
                    .col(pk_auto(Fairu::Id))
                    .col(ColumnDef::new(Fairu::Uuid).string().not_null())
                    .col(ColumnDef::new(Fairu::Name).string().not_null())
                    .col(ColumnDef::new(Fairu::Content).string().not_null())
                    .col(
                        ColumnDef::new(Fairu::CreateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Fairu::ExpireAfter)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Fairu::ExpireTime)
                            .date_time(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Fairu::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Fairu {
    Table,
    Id,
    Name,
    Uuid,
    Content,
    CreateTime,
    ExpireAfter,
    ExpireTime,
}

use sea_orm_migration::prelude::*;

const LENGTH_FAKES: u32 = 100;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Address::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Address::PostalCode).string().not_null())
                    .col(ColumnDef::new(Address::City).string().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Address::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "Address")]
enum Address {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "postal_code")]
    PostalCode,
    #[sea_orm(column_name = "city")]
    City,
}


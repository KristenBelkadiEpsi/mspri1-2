use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Order::CustomerId).uuid().not_null())
                    .foreign_key(ForeignKey::create().from(Order::Table, Order::CustomerId).to(Customer::Table, Customer::Id))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Customer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Customer::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Customer::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Customer::FirstName).string().not_null())
                    .col(ColumnDef::new(Customer::LastName).string().not_null())
                    .col(ColumnDef::new(Customer::UserName).string().not_null())
                    .col(ColumnDef::new(Customer::AddressId).string().not_null())
                    .foreign_key(ForeignKey::create().from(Customer::Table, Customer::AddressId).to(Customer::Table, Customer::Id))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum Customer {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "created_at")]
    CreatedAt,
    #[sea_orm(column_name = "name")]
    Name,
    #[sea_orm(column_name = "user_name")]
    UserName,
    #[sea_orm(column_name = "first_name")]
    FirstName,
    #[sea_orm(column_name = "last_name")]
    LastName,
    #[sea_orm(column_name = "address_id")]
    AddressId,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "created_at")]
    CreatedAt,
    #[sea_orm(column_name = "customer_id")]
    CustomerId,
}

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
                    .col(ColumnDef::new(Customer::AddressId).uuid().not_null())
                    .col(ColumnDef::new(Customer::ProfileId).uuid().not_null())
                    .col(ColumnDef::new(Customer::CompanyId).uuid().not_null())
                    .foreign_key(ForeignKey::create().from(Customer::Table, Customer::AddressId).to(Customer::Table, Customer::Id))
                    .foreign_key(ForeignKey::create().from(Customer::Table, Customer::ProfileId).to(Profile::Table, Profile::Id))
                    .foreign_key(ForeignKey::create().from(Customer::Table, Customer::CompanyId).to(Company::Table, Company::Id))
                    .to_owned(),
            )
            .await?;
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
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Profile::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Profile::FirstName).string().not_null())
                    .col(ColumnDef::new(Profile::LastName).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Company::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Company::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Company::CompanyName).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Product::Name).date_time().not_null())
                    .col(ColumnDef::new(Product::Stock).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Product::Name).date_time().not_null())
                    .col(ColumnDef::new(Product::Stock).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Detail::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Detail::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Detail::Price).string().not_null())
                    .col(ColumnDef::new(Detail::Description).text().not_null())
                    .col(ColumnDef::new(Detail::Color).string().not_null())
                    .col(ColumnDef::new(Detail::ProductId).uuid().not_null())
                    .foreign_key(ForeignKey::create().from(Detail::Table, Detail::ProductId).to(Product::Table, Product::Id))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(OrderProduct::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OrderProduct::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderProduct::ProductId).uuid().not_null())
                    .foreign_key(ForeignKey::create().from(OrderProduct::Table, OrderProduct::OrderId).to(Order::Table, Order::Id))
                    .primary_key(Index::create().name("pk_order_product").col(OrderProduct::OrderId).col(OrderProduct::ProductId))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Address::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Profile::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Company::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Product::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Detail::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Customer::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrderProduct::Table).cascade().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Order::Table).cascade().to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Address {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "postal_code")]
    PostalCode,
    #[sea_orm(column_name = "city")]
    City,
}

#[derive(DeriveIden)]
enum Profile {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "fist_name")]
    FirstName,
    #[sea_orm(column_name = "last_name")]
    LastName,
}

#[derive(DeriveIden)]
enum Company {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "company_name")]
    CompanyName,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "created_at")]
    CreatedAt,
    #[sea_orm(column_name = "name")]
    Name,
    #[sea_orm(column_name = "stock")]
    Stock,
}

#[derive(DeriveIden)]
enum Detail {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "price")]
    Price,
    #[sea_orm(column_name = "description")]
    Description,
    #[sea_orm(column_name = "color")]
    Color,
    #[sea_orm(column_name = "product_id")]
    ProductId,
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
    #[sea_orm(column_name = "profile_id")]
    ProfileId,
    #[sea_orm(column_name = "company_id")]
    CompanyId,
}

#[derive(DeriveIden)]
enum OrderProduct {
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "order_id")]
    OrderId,
    #[sea_orm(column_name = "product_id")]
    ProductId,
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

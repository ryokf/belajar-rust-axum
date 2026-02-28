use sea_orm_migration::prelude::*; // Cukup gunakan prelude standar

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Users::Table)
                .if_not_exists()
                // Menggunakan ColumnDef untuk mendefinisikan ID secara eksplisit
                .col(ColumnDef::new(Users::Id).integer().not_null().auto_increment().primary_key())
                // Menggunakan ColumnDef untuk mendefinisikan Username secara eksplisit
                .col(ColumnDef::new(Users::Username).string().not_null())
                .col(ColumnDef::new(Users::CreatedAt).date().not_null())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    CreatedAt,
}

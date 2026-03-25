use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Productos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Productos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Productos::Nombre).string_len(150).not_null())
                    .col(ColumnDef::new(Productos::Precio).integer().not_null())
                    .col(
                        ColumnDef::new(Productos::Stock)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Productos::CreadoEl)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Productos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Productos {
    Table,
    Id,
    Nombre,
    Precio,
    Stock,
    CreadoEl,
}

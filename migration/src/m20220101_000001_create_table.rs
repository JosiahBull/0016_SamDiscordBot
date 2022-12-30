use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Payment {
    Table,
    Id,
    TotalAmount,
    CreatedBy,
    CreatedAt,
    OriginatingMessageId,
    IndividualUsers,
    IndividualAmounts,
    ImagePath,
}

#[derive(Iden)]
enum List {
    Table,
    Id,
    Name,
    CreatedBy,
    CreatedAt,
    OriginalListMessageId,
    ItemMessageIds,
    Items,
    ItemAmounts,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Payment::TotalAmount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Payment::CreatedBy).big_integer().not_null())
                    .col(ColumnDef::new(Payment::CreatedAt).date_time().not_null())
                    .col(
                        ColumnDef::new(Payment::OriginatingMessageId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Payment::IndividualUsers)
                            .array(ColumnType::BigInteger(None))
                            .default(Vec::with_capacity(0))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Payment::IndividualAmounts)
                            .array(ColumnType::BigInteger(None))
                            .default(Vec::with_capacity(0))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Payment::ImagePath).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(List::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(List::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(List::Name).string().not_null())
                    .col(ColumnDef::new(List::CreatedBy).big_integer().not_null())
                    .col(ColumnDef::new(List::CreatedAt).date_time().not_null())
                    .col(
                        ColumnDef::new(List::OriginalListMessageId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(List::ItemMessageIds)
                            .array(ColumnType::BigInteger(None))
                            .default(Vec::with_capacity(0))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(List::Items)
                            .array(ColumnType::String(None))
                            .default(Vec::with_capacity(0))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(List::ItemAmounts)
                            .array(ColumnType::BigInteger(None))
                            .default(Vec::with_capacity(0))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Payment::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(List::Table).to_owned())
            .await
    }
}

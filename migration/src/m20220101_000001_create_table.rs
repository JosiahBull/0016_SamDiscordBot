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

    CreationMessageId,
    CreationMessageChannelId,
    CreationMessageGuildId,
}

#[derive(Iden)]
enum ListItem {
    Table,
    Id,
    ListId,
    MessageId,
    UserId,

    CreatedAt,
    Bought,

    Item,
    Quantity,
    Personal,
    Store,
    Notes,
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
                            .array(ColumnType::BigUnsigned(None))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Payment::IndividualAmounts)
                            .array(ColumnType::BigInteger(None))
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
                        ColumnDef::new(List::CreationMessageId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(List::CreationMessageChannelId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(List::CreationMessageGuildId).big_integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ListItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ListItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ListItem::ListId).integer().not_null())
                    .col(ColumnDef::new(ListItem::MessageId).big_integer().not_null())
                    .col(ColumnDef::new(ListItem::UserId).big_integer().not_null())
                    .col(ColumnDef::new(ListItem::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(ListItem::Bought).boolean().not_null())
                    .col(ColumnDef::new(ListItem::Item).string().not_null())
                    .col(ColumnDef::new(ListItem::Quantity).big_integer().not_null())
                    .col(ColumnDef::new(ListItem::Personal).boolean().not_null())
                    .col(ColumnDef::new(ListItem::Store).string())
                    .col(ColumnDef::new(ListItem::Notes).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .from_tbl(ListItem::Table)
                    .from_col(ListItem::ListId)
                    .to_tbl(List::Table)
                    .to_col(ListItem::Id)
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

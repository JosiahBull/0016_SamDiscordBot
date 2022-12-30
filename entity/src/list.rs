//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "list"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub created_by: i64,
    pub created_at: DateTime,
    pub original_list_message_id: i64,
    pub bought: bool,
    pub item_message_ids: Vec<i64>,
    pub items: Vec<String>,
    pub item_amounts: Vec<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    CreatedBy,
    CreatedAt,
    OriginalListMessageId,
    Bought,
    ItemMessageIds,
    Items,
    ItemAmounts,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(None).def(),
            Self::CreatedBy => ColumnType::BigInteger.def(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::OriginalListMessageId => ColumnType::BigInteger.def(),
            Self::Bought => ColumnType::Boolean.def(),
            Self::ItemMessageIds => {
                ColumnType::Array(sea_orm::sea_query::SeaRc::new(ColumnType::BigInteger)).def()
            }
            Self::Items => {
                ColumnType::Array(sea_orm::sea_query::SeaRc::new(ColumnType::String(None))).def()
            }
            Self::ItemAmounts => {
                ColumnType::Array(sea_orm::sea_query::SeaRc::new(ColumnType::BigInteger)).def()
            }
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

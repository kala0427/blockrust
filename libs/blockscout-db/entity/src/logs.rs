//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "logs")]
pub struct Model {
    pub data: Vec<u8>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub index: i32,
    pub r#type: Option<String>,
    pub first_topic: Option<String>,
    pub second_topic: Option<String>,
    pub third_topic: Option<String>,
    pub fourth_topic: Option<String>,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
    pub address_hash: Option<Vec<u8>>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub transaction_hash: Vec<u8>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub block_hash: Vec<u8>,
    pub block_number: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::addresses::Entity",
        from = "Column::AddressHash",
        to = "super::addresses::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Addresses,
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::BlockHash",
        to = "super::blocks::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Blocks,
    #[sea_orm(
        belongs_to = "super::transactions::Entity",
        from = "Column::TransactionHash",
        to = "super::transactions::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Transactions,
}

impl Related<super::addresses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Addresses.def()
    }
}

impl Related<super::blocks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blocks.def()
    }
}

impl Related<super::transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

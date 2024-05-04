use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};

#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    #[sea_orm(default_value = 1)]
    pub status: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

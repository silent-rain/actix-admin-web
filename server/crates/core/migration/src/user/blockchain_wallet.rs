//! 用户区块链钱包表
//! Entity: [`entity::user::BlockchainWallet`]
use crate::user::user_base::UserBase;

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(BlockchainWallet::Table)
                    .comment("用户区块链钱包表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlockchainWallet::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("钱包ID"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::UserId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::WalletAddress)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .not_null()
                            .comment("钱包地址"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::Mnemonic)
                            .string()
                            .string_len(255)
                            .null()
                            .comment("助记词"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::PrivateKey)
                            .string()
                            .string_len(255)
                            .null()
                            .comment("私钥"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::ChainId)
                            .integer()
                            .null()
                            .comment("区块链ID"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(BlockchainWallet::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra({
                                match manager.get_database_backend() {
                                    DatabaseBackend::Sqlite => "DEFAULT CURRENT_TIMESTAMP",
                                    _ => "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP",
                                }
                            })
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(BlockchainWallet::Table.to_string(), "idx_user_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx_user_id")
                        .table(BlockchainWallet::Table)
                        .col(BlockchainWallet::UserId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(BlockchainWallet::Table.to_string(), "idx_wallet_address")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx_wallet_address")
                        .table(BlockchainWallet::Table)
                        .col(BlockchainWallet::WalletAddress)
                        .to_owned(),
                )
                .await?;
        }

        // Sqlite 不支持外键
        if manager.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }

        if !manager
            .has_index(
                BlockchainWallet::Table.to_string(),
                "fk_user_blockchain_wallet_user_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_user_blockchain_wallet_user_id")
                        .from(BlockchainWallet::Table, BlockchainWallet::UserId)
                        .to(UserBase::Table, UserBase::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(BlockchainWallet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BlockchainWallet {
    #[sea_orm(iden = "t_user_blockchain_wallet")]
    Table,
    Id,
    UserId,
    WalletAddress,
    Mnemonic,
    PrivateKey,
    ChainId,
    Desc,
    CreatedAt,
    UpdatedAt,
}

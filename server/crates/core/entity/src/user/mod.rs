//! 任务调度相关表
pub mod blockchain_wallet;
pub mod location;
pub mod user_base;
pub mod user_email;
pub mod user_phone;
pub mod user_role;
pub mod user_role_rel;

pub use blockchain_wallet::Entity as BlockchainWallet;
pub use location::Entity as Location;
pub use user_base::Entity as UserBase;
pub use user_email::Entity as UserEmail;
pub use user_phone::Entity as UserPhone;
pub use user_role::Entity as UserRole;
pub use user_role_rel::Entity as UserRoleRel;

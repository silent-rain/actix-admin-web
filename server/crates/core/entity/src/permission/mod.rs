//! 权限相关
pub mod menu;
pub mod menu_role_rel;
pub mod openapi;
pub mod openapi_role_rel;
pub mod token;
pub mod token_role_rel;

pub use menu::Entity as Menu;
pub use menu_role_rel::Entity as MenuRoleRel;
pub use openapi::Entity as Openapi;
pub use openapi_role_rel::Entity as OpenapiRoleRel;
pub use token::Entity as Token;
pub use token_role_rel::Entity as TokenRoleRel;

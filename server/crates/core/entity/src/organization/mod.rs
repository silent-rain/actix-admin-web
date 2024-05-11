//! 组织相关表
pub mod department;
pub mod department_role_rel;
pub mod position;
pub mod rank;

pub use department::Entity as Department;
pub use department_role_rel::Entity as DepartmentRoleRel;
pub use position::Entity as Position;
pub use rank::Entity as Rank;

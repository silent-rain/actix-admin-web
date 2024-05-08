//! 组织管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{department::DepartmentDao, department_role_rel::DepartmentRoleRelDao};

pub(crate) mod service;
pub use service::{department::DepartmentService, department_role_rel::DepartmentRoleRelService};

pub(crate) mod controller;
pub use controller::{
    department::DepartmentController, department_role_rel::DepartmentRoleRelController,
};

pub(crate) mod router;
pub use router::{
    department::DepartmentRouter, department_role_rel::DepartmentRoleRelRouter, OrganizationRouter,
};

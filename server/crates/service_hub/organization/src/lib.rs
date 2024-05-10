//! 组织管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    department::DepartmentDao, department_role_rel::DepartmentRoleRelDao, position::PositionDao,
    rank::RankDao,
};

pub(crate) mod service;
pub use service::{
    department::DepartmentService, department_role_rel::DepartmentRoleRelService,
    position::PositionService, rank::RankService,
};

pub(crate) mod controller;
pub use controller::{
    department::DepartmentController, department_role_rel::DepartmentRoleRelController,
    position::PositionController, rank::RankController,
};

pub(crate) mod router;
pub use router::{
    department::DepartmentRouter, department_role_rel::DepartmentRoleRelRouter,
    position::PositionRouter, rank::RankRouter, OrganizationRouter,
};

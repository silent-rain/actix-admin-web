//! 枚举

/// 角色状态
#[derive(Debug, PartialEq)]
pub enum RoleStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 部门状态
#[derive(Debug, PartialEq)]
pub enum DeptStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

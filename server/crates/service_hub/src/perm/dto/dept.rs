//! 部门管理
use crate::perm::enums::DeptStatus;

use actix_validator::Validate;
use entity::perm_dept;

use serde::{Deserialize, Serialize};


/// 查询部门列表
#[derive(Default, Deserialize, Validate)]
pub struct GetDeptListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加部门
#[derive(Serialize, Deserialize, Validate)]
pub struct AddDeptReq {
    /// 上级部门ID
    pub pid: Option<i32>,
    /// 所有上级部门ID, 用逗号分开
    pub pids: Option<String>,
    /// 部门名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
}

/// 更新数据
#[derive(Default, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDeptReq {
    /// 部门ID
    pub id: i32,
    /// 上级部门ID
    pub pid: Option<i32>,
    /// 所有上级部门ID, 用逗号分开
    pub pids: Option<String>,
    /// 部门名称
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: DeptStatus,
}

/// 更新数据状态
#[derive(Default, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDeptStatusReq {
    /// ID
    pub id: i32,
    /// 状态,0:停用,1:正常
    pub status: DeptStatus,
}

/// 部门树列表
#[derive(Debug, Serialize, Deserialize)]
pub struct DeptTree {
    #[serde(flatten)]
    pub dept: perm_dept::Model,
    /// 子部门列表
    pub children: Vec<DeptTree>,
}

impl DeptTree {
    /// 将Dept转换为DeptTree
    pub fn new(model: &perm_dept::Model) -> Self {
        DeptTree {
            dept: model.clone(),
            children: Vec::new(),
        }
    }

    /// 添加子部门
    pub fn add_child(&mut self, child: DeptTree) {
        self.children.push(child);
    }
}

/// 将列表转换为树列表
pub fn dept_list_to_tree(depts: &[perm_dept::Model], pid: Option<i32>) -> Vec<DeptTree> {
    let mut trees = Vec::new();
    for dept in depts {
        // 根节点或子节点
        if (dept.pid.is_none() && pid.is_none())
            || (dept.pid.is_some() && pid.is_some() && dept.pid == pid)
        {
            trees.push(DeptTree::new(dept));
        }
    }
    for item in trees.iter_mut() {
        let children = dept_list_to_tree(depts, Some(item.dept.id));
        item.children.extend(children)
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dept_list_to_tree() {
        let depts = vec![
            perm_dept::Model {
                id: 1,
                pid: None,
                name: "name1".to_string(),
                status: 1,
                ..Default::default()
            },
            perm_dept::Model {
                id: 2,
                pid: None,
                name: "name2".to_string(),
                status: 1,
                ..Default::default()
            },
            perm_dept::Model {
                id: 3,
                pid: Some(2),
                name: "name3".to_string(),
                status: 1,
                ..Default::default()
            },
            perm_dept::Model {
                id: 4,
                pid: Some(3),
                name: "name4".to_string(),
                status: 1,
                ..Default::default()
            },
        ];
        let results = dept_list_to_tree(&depts, None);
        assert!(!results.is_empty());
    }
}

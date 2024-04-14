//! 列表转为树结构

use serde::{Deserialize, Serialize};

/// 泛型部门模型接口
pub trait GenericTreeTrait {
    /// 主键ID
    fn id(&self) -> i32;
    /// 父ID
    fn pid(&self) -> Option<i32>;
}

/// 通用树结构
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericTree<T> {
    #[serde(flatten)]
    pub data: T,
    /// 子列表
    pub children: Vec<GenericTree<T>>,
}

impl<T: GenericTreeTrait + Clone> GenericTree<T> {
    /// 创建树结构对象
    pub fn new(model: &T) -> Self {
        GenericTree {
            data: model.clone(),
            children: Vec::new(),
        }
    }

    /// 添加列表
    pub fn add_child(&mut self, child: GenericTree<T>) {
        self.children.push(child);
    }
}

impl<T: GenericTreeTrait + Clone> GenericTree<T> {
    /// 将列表转换为树列表
    pub fn to_tree(depts: &[T], pid: Option<i32>) -> Vec<GenericTree<T>> {
        let mut trees = Vec::new();
        for dept in depts {
            if dept.pid() == pid {
                let mut tree = GenericTree::new(dept);
                tree.children = Self::to_tree(depts, Some(dept.id()));
                trees.push(tree);
            }
        }
        trees
    }
}

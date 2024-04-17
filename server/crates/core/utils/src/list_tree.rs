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
    pub fn to_tree(data_list: &[T], pid: Option<i32>) -> Vec<GenericTree<T>> {
        let mut trees = Vec::new();
        for data in data_list {
            if data.pid() == pid {
                let mut tree = GenericTree::new(data);
                tree.children = Self::to_tree(data_list, Some(data.id()));
                trees.push(tree);
            }
        }
        trees
    }

    /// 获取所有上级ID
    pub fn get_pids(data_list: &[T], id: i32) -> Vec<i32> {
        let mut ids = Vec::new();
        if id == 0 {
            return ids;
        }
        for data in data_list {
            if data.id() != id {
                continue;
            }

            let pid = match data.pid() {
                Some(v) => v,
                None => break,
            };
            ids.push(pid);
            let inner_pids = Self::get_pids(data_list, pid);
            ids.extend(inner_pids);
            break;
        }
        ids
    }
}

use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

/**
 * 226. 翻转二叉树
简单
相关标签
相关企业
给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。

 

示例 1：



输入：root = [4,2,7,1,3,6,9]
输出：[4,7,2,9,6,3,1]
示例 2：



输入：root = [2,1,3]
输出：[2,3,1]
示例 3：

输入：root = []
输出：[]
 

提示：

树中节点数目范围在 [0, 100] 内
-100 <= Node.val <= 100
 */
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        // 翻转左子树
        let left = invert_tree(node.borrow_mut().left.take());
        // 翻转右子树
        let right = invert_tree(node.borrow_mut().right.take());
        // 交换左右儿子
        node.borrow_mut().left = right;
        node.borrow_mut().right = left;
        Some(node)
    } else {
        None
    }
}
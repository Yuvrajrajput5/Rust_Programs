struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left:None,
            right:None,
        }
    }
}

fn maxDepth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth=max_depth(&node.left);
            let right_depth=max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
    }
}

fn main() {
    let mut root = Some(Box::new(TreeNode::new(2)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(14)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(40)));
    root.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(56)));
    root.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(17)));
    root.as_mut().unwrap().right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(152)));
 
    println!("Maximum depth: {}", maxDepth(&root));
}
use colored::Colorize;
use nd_core::executor::runner::Runner;
use std::{fmt, path::PathBuf};

use crate::actions::utils::resolve_path;

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    children: Vec<Box<TreeNode<T>>>,
}

impl<T: fmt::Display> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: Box<TreeNode<T>>) {
        self.children.push(child);
    }

    fn print(&self, prefix: &str, is_last: bool) {
        println!(
            "{}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            self.value
        );

        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        for (i, child) in self.children.iter().enumerate() {
            child.print(&new_prefix, i == self.children.len() - 1);
        }
    }
}

enum NodeType {
    File(String),
    Sequence(String),
    Request(String),
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            NodeType::File(name) => write!(f, "{}", name),
            NodeType::Sequence(name) => write!(f, "{}", name.yellow()),
            NodeType::Request(name) => write!(f, "{}", name.green()),
        };
    }
}

pub fn draw_tree(filepath: &PathBuf) {
    let (filepath, is_project) = resolve_path(filepath);
    let runner = Runner::new(
        &filepath,
        None,
        nd_core::executor::runner::ScriptEngine::None,
        is_project,
    )
    .unwrap();

    let mut root: TreeNode<NodeType> = TreeNode::new(NodeType::File(runner.schema.filename));

    for (sequence, requests) in runner.schema.calls {
        let mut seq_root = TreeNode::new(NodeType::Sequence(sequence));
        for request in requests {
            seq_root.add_child(Box::new(TreeNode::new(NodeType::Request(request))));
        }
        root.add_child(Box::new(seq_root));
    }

    root.print("", false);
}

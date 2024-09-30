use std::fmt::{Display, Formatter};
use crate::tree::TreeNode;

fn intersperse<T: Clone>(v: &Vec<T>, sep: T, count: usize) -> Vec<T> {
    if v.is_empty() {
        Vec::new()
    } else {
        let mut result = Vec::new();
        for (i, elt) in v.iter().enumerate() {
            result.push(elt.clone());
            if i + 1 != v.len() {
                for _ in 0..count {
                    result.push(sep.clone());
                }
            }
        }
        result
    }
}

impl<T: Ord + Clone + Display> TreeNode<T> {
    fn rows(&self) -> Vec<Vec<Option<T>>> {
        let mut rows: Vec<Vec<Option<T>>> = Vec::new();
        let mut current: Vec<TreeNode<T>> = vec![self.clone()];
        let mut next: Vec<TreeNode<T>> = vec![];
        loop {
            let mut row = vec![];
            for elt in current.iter() {
                match elt {
                    TreeNode::Leaf => row.push(None),
                    TreeNode::Node(value, _, _) => row.push(Some(value.clone())),
                }
            }
            if row.iter().all(|x| x.is_none()) {
                break;
            }
            rows.push(row);

            for value in current {
                match value {
                    TreeNode::Leaf => {
                        next.push(TreeNode::Leaf);
                        next.push(TreeNode::Leaf);
                    }
                    TreeNode::Node(_, left, right) => {
                        next.push(*left);
                        next.push(*right);
                    }
                }
            }
            current = next;
            next = vec![];
        }
        rows
    }
    pub fn show(&self, width: usize) -> String {
        let rows = self.rows();

        let mut with_padding = vec![];
        let mut around_padding_count = 0;
        let mut inter_padding_count = 1;
        for row in rows.into_iter().rev() {
            let around_padding = vec![None; around_padding_count];
            with_padding.push(
                around_padding
                    .clone()
                    .into_iter()
                    .chain(intersperse(&row, None, inter_padding_count).into_iter())
                    .chain(around_padding)
                    .collect::<Vec<_>>(),
            );
            around_padding_count = inter_padding_count;
            inter_padding_count = inter_padding_count * 2 + 1;
        }

        with_padding.into_iter().rev().map(|row| {
            row.into_iter().map(|x| {
                match x {
                    Some(v) => format!("{: ^width$}", v, width = width),
                    None => format!("{: ^width$}", "", width = width),
                }
            }).collect::<Vec<_>>().join("")
        }).collect::<Vec<_>>().join("\n")
    }
}

impl<T: Ord + Clone + Display> Display for TreeNode<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.show(3))
    }
}

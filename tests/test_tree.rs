use links::tree::*;
use quickcheck::quickcheck;
#[allow(unused_imports)]
use std::collections::HashSet;

fn validate<T: Ord>(node: &TreeNode<T>) -> bool {
    fn property1<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
        match tree {
            TreeNode::Leaf => true,
            TreeNode::Node(value, left, right) => {
                min.map_or(true, |min| value > min)
                    && max.map_or(true, |max| value < max)
                    && property1(left, min, Some(value))
                    && property1(right, Some(value), max)
            }
        }
    }
    fn property2<T: Ord>(tree: &TreeNode<T>) -> Option<i32> {
        match tree {
            TreeNode::Leaf => Some(0),
            TreeNode::Node(_, left, right) => {
                let x = property2(left)?;
                let y = property2(right)?;
                Some((x - y).abs()).filter(|x| x <= &1).map(|_| 1 + x.max(y))
            }
        }
    }
    property1(node, None, None) && property2(node).is_some()
}

/// A wrapper over i32 that implements `PartialOrd`, `PartialEq`, `Eq`, `Ord`
#[derive(PartialOrd, PartialEq, Eq, Ord)]
struct Num(i32);

/// This test checks that the default implementation of `TreeNode` is `Leaf`
/*
#[test]
pub fn test_impls_default_5() {
    let list: TreeNode<Num> = TreeNode::default();
    match list {
        TreeNode::Leaf => {}
        _ => {
            eprintln!(
                "list is not Leaf (see test case source in {} for details)",
                file!()
            );
            assert!(false);
        }
    }
}
*/

/// This test checks that `TreeNode` properly implements `Eq`
/*
#[test]
pub fn test_impls_eq_5() {
    let tree1: TreeNode<Num> = TreeNode::Node(
        Num(1),
        Box::new(TreeNode::Node(
            Num(2),
            Box::new(TreeNode::Node(
                Num(3),
                Box::new(TreeNode::Leaf),
                Box::new(TreeNode::Leaf),
            )),
            Box::new(TreeNode::Leaf),
        )),
        Box::new(TreeNode::Leaf),
    );
    let tree2: TreeNode<Num> = TreeNode::Node(
        Num(-1),
        Box::new(TreeNode::Node(
            Num(999),
            Box::new(TreeNode::Leaf),
            Box::new(TreeNode::Node(
                Num(0),
                Box::new(TreeNode::Leaf),
                Box::new(TreeNode::Leaf),
            )),
        )),
        Box::new(TreeNode::Leaf),
    );
    let tree3: TreeNode<Num> = TreeNode::Leaf;

    // Can't use `assert` macros here because the type does not implement Debug
    if tree1 != tree1 {
        eprintln!(
            "tree1 != tree1 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if tree2 != tree2 {
        eprintln!(
            "tree2 != tree2 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if tree3 != tree3 {
        eprintln!(
            "tree3 != tree3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }

    if tree1 == tree3 {
        eprintln!(
            "tree1 == tree3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if tree2 == tree3 {
        eprintln!(
            "tree2 == tree3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if tree1 == tree2 {
        eprintln!(
            "tree1 == tree2 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
}
*/

/// This test checks that inserting duplicate values does not affect the tree.
/*
#[test]
fn test_insert_same_5() {
    let mut t = TreeNode::new();

    t.insert(1);
    t.insert(1);
    t.insert(1);
    t.insert(1);

    assert_eq!(
        t,
        TreeNode::Node(1, Box::new(TreeNode::Leaf), Box::new(TreeNode::Leaf))
    );
}
*/

/// This test checks that right rotation works correctly.
/*
#[test]
fn test_rotate_right_5() {
    let mut t = TreeNode::Node(
        1,
        Box::new(TreeNode::Node(
            2,
            Box::new(TreeNode::Node(
                3,
                Box::new(TreeNode::Leaf),
                Box::new(TreeNode::Leaf),
            )),
            Box::new(TreeNode::Leaf),
        )),
        Box::new(TreeNode::Leaf),
    );

    let expected = TreeNode::Node(
        2,
        Box::new(TreeNode::Node(
            3,
            Box::new(TreeNode::Leaf),
            Box::new(TreeNode::Leaf),
        )),
        Box::new(TreeNode::Node(
            1,
            Box::new(TreeNode::Leaf),
            Box::new(TreeNode::Leaf),
        )),
    );

    t.right_rotate();
    assert_eq!(t, expected);
}
*/

/// This test checks that left rotation works correctly.
/*
#[test]
fn test_rotate_left_5() {
    let mut t = TreeNode::Node(
        1,
        Box::new(TreeNode::Leaf),
        Box::new(TreeNode::Node(
            2,
            Box::new(TreeNode::Leaf),
            Box::new(TreeNode::Node(
                3,
                Box::new(TreeNode::Leaf),
                Box::new(TreeNode::Leaf),
            )),
        )),
    );

    let expected = TreeNode::Node(
        2,
        Box::new(TreeNode::Node(
            1,
            Box::new(TreeNode::Leaf),
            Box::new(TreeNode::Leaf),
        )),
        Box::new(TreeNode::Node(
            3,
            Box::new(TreeNode::Leaf),
            Box::new(TreeNode::Leaf),
        )),
    );

    t.left_rotate();
    assert_eq!(t, expected);
}
*/

/// This test checks that the tree is balanced after inserting a few values.
#[test]
fn test_rebalance_root_5() {
    {
        let mut t = TreeNode::new();

        t.insert(1);
        assert!(validate(&t));
        t.insert(0);
        assert!(validate(&t));
        t.insert(-1);
        assert!(validate(&t));
    }

    {
        let mut t = TreeNode::new();
        t.insert(1);
        assert!(validate(&t));
        t.insert(2);
        assert!(validate(&t));
        t.insert(3);
        assert!(validate(&t));
    }
}

/// This test uses "property-based testing": this means it generates random test cases
/// and checks that a property holds for all of them.
/// In this case, the property is that the tree is always valid after inserting a value.
#[test]
fn test_insert_5() {
    fn insert_is_valid(v: Vec<i32>) -> bool {
        let mut t = TreeNode::new();
        for x in v.iter() {
            t.insert(*x);
            if !validate(&t) {
                return false;
            }
        }
        return true;
    }
    quickcheck(insert_is_valid as fn(Vec<i32>) -> bool);
}

/// This test uses "property-based testing": this means it generates random test cases
/// and checks that a property holds for all of them.
/// In this case, the property is that adding all the elements from a `Vec<i32>` to a `TreeNode<i32>`
/// and converting it back to a `Vec<i32>` should yield the same `Vec<i32>` you started with.
/*
#[test]
fn test_insert_roundtrip_10() {
    fn roundtrip(v: Vec<i32>) -> bool {
        // Insert the values into the tree and convert back to a vector
        let mut t = TreeNode::new();
        for x in v.iter() {
            t.insert(*x);
        }
        let v2: Vec<i32> = t.into();

        // Deduplicate the vector and compare the lengths
        let set: HashSet<i32> = v.iter().cloned().collect();
        if v2.len() != set.len() {
            return false;
        }

        // Check that all values in the tree are in the set
        for x in v2.iter() {
            if !set.contains(x) {
                return false;
            }
        }
        return true;
    }
    quickcheck(roundtrip as fn(Vec<i32>) -> bool);
}
*/

const _UNUSED: bool = true;

use links::list::*;
#[allow(unused_imports)]
use quickcheck::quickcheck;
use std::fmt::Display;

/// A wrapper over i32 that doesn't implement any traits
struct Num(i32);

/// A wrapper over i32 that implements `PartialEq`
#[derive(PartialEq)]
struct PartialEqNum(i32);

/// A wrapper over i32 that implements `Eq`
#[derive(PartialEq, Eq)]
struct EqNum(i32);

/// A wrapper over i32 that implements `Display`
struct DisplayNum(i32);
impl Display for DisplayNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// This test checks that the default implementation of `ListNode` is `Nil`
#[test]
pub fn test_impls_default_3() {
    let list: ListNode<Num> = ListNode::default();
    match list {
        ListNode::Nil => {}
        _ => {
            eprintln!(
                "list is not Nil (see test case source in {} for details)",
                file!()
            );
            assert!(false);
        }
    }
}

/// This test checks that `ListNode` properly implements `PartialEq`
/*
#[test]
pub fn test_impls_partialeq_5() {
    let list1 = ListNode::Cons(
        PartialEqNum(1),
        Box::new(ListNode::Cons(
            PartialEqNum(2),
            Box::new(ListNode::Cons(PartialEqNum(3), Box::new(ListNode::Nil))),
        )),
    );
    let list2 = ListNode::Cons(
        PartialEqNum(-1),
        Box::new(ListNode::Cons(
            PartialEqNum(999),
            Box::new(ListNode::Cons(PartialEqNum(0), Box::new(ListNode::Nil))),
        )),
    );
    let list3 = ListNode::Nil;

    // Can't use `assert` macros here because the type does not implement Debug
    if list1 != list1 {
        eprintln!(
            "list1 != list1 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list2 != list2 {
        eprintln!(
            "list2 != list2 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list3 != list3 {
        eprintln!(
            "list3 != list3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }

    if list1 == list3 {
        eprintln!(
            "list1 == list3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list2 == list3 {
        eprintln!(
            "list2 == list3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list1 == list2 {
        eprintln!(
            "list1 == list2 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
}
*/

/// This test checks that `ListNode` properly implements `Eq`
/*
#[test]
pub fn test_impls_eq_3() {
    let list1 = ListNode::Cons(
        EqNum(1),
        Box::new(ListNode::Cons(
            EqNum(2),
            Box::new(ListNode::Cons(EqNum(3), Box::new(ListNode::Nil))),
        )),
    );
    let list2 = ListNode::Cons(
        EqNum(-1),
        Box::new(ListNode::Cons(
            EqNum(999),
            Box::new(ListNode::Cons(EqNum(0), Box::new(ListNode::Nil))),
        )),
    );
    let list3 = ListNode::Nil;

    // Can't use `assert` macros here because the type does not implement Debug
    if list1 != list1 {
        eprintln!(
            "list1 != list1 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list2 != list2 {
        eprintln!(
            "list2 != list2 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list3 != list3 {
        eprintln!(
            "list3 != list3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }

    if list1 == list3 {
        eprintln!(
            "list1 == list3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list2 == list3 {
        eprintln!(
            "list2 == list3 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
    if list1 == list2 {
        eprintln!(
            "list1 == list2 (see test case source in {} for details)",
            file!()
        );
        assert!(false);
    }
}
*/

/// This test checks that `ListNode` properly implements `Display`
/*
#[test]
pub fn test_impls_display_3() {
    let list1 = ListNode::Cons(
        DisplayNum(1),
        Box::new(ListNode::Cons(
            DisplayNum(2),
            Box::new(ListNode::Cons(DisplayNum(3), Box::new(ListNode::Nil))),
        )),
    );
    let list2 = ListNode::Cons(
        DisplayNum(-1),
        Box::new(ListNode::Cons(
            DisplayNum(999),
            Box::new(ListNode::Cons(DisplayNum(0), Box::new(ListNode::Nil))),
        )),
    );
    let list3: ListNode<DisplayNum> = ListNode::Nil;

    assert_eq!(format!("{}", list1), "1 -> 2 -> 3 -> Nil");
    assert_eq!(format!("{}", list2), "-1 -> 999 -> 0 -> Nil");
    assert_eq!(format!("{}", list3), "Nil");
}
*/

/// This test checks that `ListNode` properly implements `From<Vec>`
/*
#[test]
pub fn test_impls_list_from_vec_3() {
    let v = vec![Num(1), Num(2), Num(3)];
    let list: ListNode<Num> = v.into();
    match list {
        ListNode::Cons(Num(1), next1) => match *next1 {
            ListNode::Cons(Num(2), next2) => match *next2 {
                ListNode::Cons(Num(3), next3) => match *next3 {
                    ListNode::Nil => {
                        return;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
    eprintln!(
        "list is not [1, 2, 3] (see test case source in {} for details)",
        file!()
    );
    assert!(false);
}
*/

/// This test checks that Vec properly implements `From<ListNode>`
/*
#[test]
pub fn test_impls_vec_from_list_3() {
    let list = ListNode::Cons(
        Num(1),
        Box::new(ListNode::Cons(
            Num(2),
            Box::new(ListNode::Cons(Num(3), Box::new(ListNode::Nil))),
        )),
    );

    let v: Vec<Num> = list.into();
    assert_eq!(v.len(), 3);

    match v[0] {
        Num(1) => {
            match v[1] {
                Num(2) => {
                    match v[2] {
                        Num(3) => {
                            return;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    };
    eprintln!(
        "list is not [1, 2, 3] (see test case source in {} for details)",
        file!()
    );
}
*/

/// This test uses "property-based testing": this means it generates random test cases
/// and checks that a property holds for all of them.
/// In this case, the property is that converting a `Vec<i32>` to a `ListNode<i32>` and back
/// to a `Vec<i32>` should yield the same `Vec<i32>` you started with.
/*
#[test]
pub fn test_vec_to_list_to_vec_roundtrip_8() {
    fn roundtrip(v: Vec<i32>) {
        let list: ListNode<i32> = v.clone().into();
        let v2: Vec<i32> = list.into();
        assert_eq!(v, v2);
    }
    quickcheck(roundtrip as fn(Vec<i32>));
}
*/

/// This test checks that the `insert` works in a simple case
/*
#[test]
pub fn test_insert_simple_3() {
    let mut head = ListNode::default();
    let mut list = &mut head;
    list = list.insert(-10);
    list = list.insert(4);
    list.insert(1);
    assert_eq!(
        head,
        ListNode::Cons(
            -10,
            Box::new(ListNode::Cons(
                4,
                Box::new(ListNode::Cons(1, Box::new(ListNode::Nil)))
            ))
        )
    );
}
*/

/// This test uses "property-based testing": this means it generates random test cases
/// and checks that a property holds for all of them.
/// In this case, the property is that adding all the elements from a `Vec<i32>` to a `ListNode<i32>`
/// and converting it back to a `Vec<i32>` should yield the same `Vec<i32>` you started with.
/*
#[test]
pub fn test_insert_7() {
    fn insert_is_valid(v: Vec<i32>) -> bool {
        let mut head = ListNode::default();
        let mut list = &mut head;
        for x in v.iter() {
            list = list.insert(*x);
        }
        v == Vec::from(head)
    }
    quickcheck(insert_is_valid as fn(Vec<i32>) -> bool);
}
*/

/// This test uses "property-based testing": this means it generates random test cases
/// and checks that a property holds for all of them.
/// In this case, the property is that converting a `Vec<i32>` to a `ListNode<i32>` and
/// calling `reverse` on it should yield a `ListNode<i32>` that, when converted back to a `Vec<i32>`,
/// is the reverse of the original `Vec<i32>`.
/*
#[test]
pub fn test_reverse_7() {
    fn reverse_is_valid(v: Vec<i32>) -> bool {
        let mut head: ListNode<i32> = v.clone().into();
        head.reverse();
        v.iter().rev().eq(Vec::from(head).iter())
    }
    quickcheck(reverse_is_valid as fn(Vec<i32>) -> bool);
}
*/

const _UNUSED: bool = true;

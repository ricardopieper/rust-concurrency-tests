use rust_concurrency_tests::coarse_grained_linked_set::CoarseLockLinkedSet;

#[test]
fn adds_item() {
    let mut set = CoarseLockLinkedSet::<i32>::new();
    set.add(1);
    assert!(set.len() == 1);

    set.add(2);
    assert!(set.len() == 2);

    set.add(3);
    assert!(set.len() == 3);

    set.add(4);
    assert!(set.len() == 4);

    set.add(5);
    assert!(set.len() == 5);
}

#[test]
fn adding_same_item_has_no_effect() {
    let mut set = CoarseLockLinkedSet::<i32>::new();
    set.add(1);
    set.add(1);
    set.add(1);
    set.add(1);
    set.add(1);
    set.add(1);
    set.add(1);
    assert!(set.len() == 1);
    set.add(2);
    assert!(set.len() == 2);

}

#[test]
fn adds_and_removes_root() {
    let mut set = CoarseLockLinkedSet::<i32>::new();
    set.add(1);
    let removed = set.remove(1);
    assert!(removed == Some(1));
    assert!(set.len() == 0);
}

#[test]
fn adds_root_and_fails_to_remove() {
    let mut set = CoarseLockLinkedSet::<i32>::new();
    set.add(1);
    let removed = set.remove(2);
    assert!(removed == None);
    assert!(set.len() == 1);
}

#[test]
fn adds_multiple_items_and_removes_everything_front_to_back() {
    let mut set = CoarseLockLinkedSet::<i32>::new();
    set.add(1);
    set.add(2);
    set.add(3);
    set.add(4);
    
    let removed = set.remove(1);
    assert!(removed == Some(1));
    assert!(set.len() == 3);

    let removed = set.remove(2);
    assert!(removed == Some(2));
    assert!(set.len() == 2);

    let removed = set.remove(3);
    assert!(removed == Some(3));
    assert!(set.len() == 1);

    let removed = set.remove(4);
    assert!(removed == Some(4));
    assert!(set.len() == 0);
}

#[test]
fn adds_multiple_items_and_removes_everything_back_to_front() {
    let mut set = CoarseLockLinkedSet::<i32>::new();
    set.add(1);
    set.add(2);
    set.add(3);
    set.add(4);
    
    let removed = dbg!(set.remove(4));
    assert!(removed == Some(4));
    assert!(set.len() == 3);

    let removed = set.remove(3);
    assert!(removed == Some(3));
    assert!(set.len() == 2);

    let removed = set.remove(2);
    assert!(removed == Some(2));
    assert!(set.len() == 1);

    let removed = set.remove(1);
    assert!(removed == Some(1));
    assert!(set.len() == 0);
}

#[test]
fn adds_multiple_items_and_removes_everything_no_particular_order() {
    let mut set = CoarseLockLinkedSet::<i32>::new();
    set.add(1);
    set.add(2);
    set.add(3);
    set.add(4);
    
    let removed = dbg!(set.remove(3));
    assert!(removed == Some(3));
    assert!(set.len() == 3);

    let removed = set.remove(2);
    assert!(removed == Some(2));
    assert!(set.len() == 2);

    let removed = set.remove(4);
    assert!(removed == Some(4));
    assert!(set.len() == 1);

    let removed = set.remove(1);
    assert!(removed == Some(1));
    assert!(set.len() == 0);
}

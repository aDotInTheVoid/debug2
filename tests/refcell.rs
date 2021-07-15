use std::cell::RefCell;

use debug2::pprint;

#[test]
fn test_refcell() {
    let refcell = RefCell::new(5);
    assert_eq!(pprint(&refcell), "RefCell { value: 5 }");
    let borrow = refcell.borrow_mut();
    assert_eq!(pprint(&refcell), "RefCell { value: <borrowed> }");
    drop(borrow);
    assert_eq!(pprint(&refcell), "RefCell { value: 5 }");
}

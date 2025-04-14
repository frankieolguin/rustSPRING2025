use std::rc::Rc;
use std::cell::RefCell;

/*

*/
fn interior_mutability() {
    #[derive(Debug)]
    struct MyData {
        data: f64
    }
    // reference count of refcell data? 
    // rc enables multiple pointers. refcell alows mutability of some data.

    let base: Rc<RefCell<MyData>> = Rc::new(RefCell::new(
        MyData {
            data: 70.00
        }
    ));

    println!("base: {:?}", base);
    
    {
        let mut base_2 = base.borrow_mut();
        base_2.data -= 20.00;
        println!("base_2: {:?}", base_2);
    }
 
    println!("base: {:?}", base);
 
    let mut base_3 = base.borrow_mut();
    base_3.data *= 2.00;

    // data should be borrowed by base_3 at this point
    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
// interior mutability pattern
fn main() {
    interior_mutability();
}

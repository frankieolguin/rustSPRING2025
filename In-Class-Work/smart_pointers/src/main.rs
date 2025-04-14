/*
    basic example of RC reference counters.
    6 copies means Rc::strong_count(&var) should return 4 here.
*/
use std::rc::Rc;

fn reference_counting_simple() {
    let num = 10;
    let num_in_heap = Rc::new(num);

    let _copy2_of_num = Rc::clone(&num_in_heap);
    let _copy3_of_num = Rc::clone(&num_in_heap);
    let _copy4_of_num = Rc::clone(&num_in_heap);
    let _copy5_of_num = Rc::clone(&num_in_heap);
    let _copy6_of_num = Rc::clone(&num_in_heap);

    println!("num in heap has: {} references", 
             Rc::strong_count(&num_in_heap));
}

fn main() {
    reference_counting_simple();
}

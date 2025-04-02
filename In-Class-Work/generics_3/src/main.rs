
fn implemeting_traits_on_native_data_type() {
    // every Rust data type is a struct technically,
    // which means you can implement your own trait on any of them

    trait Double {
        fn double(&self) -> Self;
    }
    trait Triple {
        fn triple(&self) -> Self;
    }

    impl Double for i32 {
        fn double(&self) -> Self {
            self * 2
        }
    }
    impl Triple for i32 {
        fn triple(&self) -> Self {
            self * 3
        }
    }

    impl Double for String {
        fn double(&self) -> Self {
            format!("{}:{}",&self,&self)
        }
    }
    impl Triple for String {
        fn triple(&self) -> Self {
            format!("{}:{}:{}",&self,&self,&self)
        }
    }

    println!("double 5_i32 == {}", 5_i32.double());
    println!("double hello == {}\n", "hello".to_string().double());

    println!("triple 5_i32 == {}", 5_i32.triple());
    println!("triple hello == {}", "hello".to_string().triple());
}
fn main() {
    implemeting_traits_on_native_data_type();
}

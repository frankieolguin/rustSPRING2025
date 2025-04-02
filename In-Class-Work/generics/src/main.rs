fn same_method_same_logical_entity(){

    // this is a big idea.
    // bind different data types with same behaviour
    // as one logical unit
    pub trait AreaInfo {
        fn get_area(&self) -> f64;
        fn get_perimeter(&self) ->f64;
    }
    

    pub struct Rectangle{
        pub width: f64,
        pub height: f64,
    }

    impl AreaInfo for Rectangle {
        fn get_area(&self) -> f64 {
            self.width * self.height
        }
        fn get_perimeter(&self)->f64 {
            (self.width + self.height) * 2.0 
        }
    }
    


    pub struct Circle {
        pub radius: f64,
    }

    impl AreaInfo for Circle {
        fn get_area(&self) -> f64 {
            self.radius * self.radius * 3.14 as f64
        }
        fn get_perimeter(&self)->f64 {
            self.radius * 6.28 
        }
    }

    // You could say, it's almost the same, well what is same for you is not the same for the compiler.

    // Make sure nothing is broken

    let rec = Rectangle {width:5.0,height:8.0};
    let circle = Circle {radius: 5.0};

    println!("Area of a rectangle {}", rec.get_area());
    println!("Area of a circle {}\n", circle.get_area());

    // And now the Magic or cheating, I don't know how to call it
    
    let shapes: Vec<&dyn AreaInfo> = vec![&rec,&circle];

    // dyn -> dynamic keyword 
    // https://doc.rust-lang.org/std/keyword.dyn.html

    // Dynamically dispatch the type of object
    for shape in shapes.iter() {
        println!("area: {}", shape.get_area());
        println!("perimter: {}\n", shape.get_perimeter());
    }
}

fn main() {
    println!("Generics Practice! \n");
    same_method_same_logical_entity();
}

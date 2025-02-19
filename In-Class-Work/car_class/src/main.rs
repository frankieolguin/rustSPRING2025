#[derive(Debug)]
struct Car {
    // variable : Data Type,
    body : String,
    year : u16,
    color : String,
}
impl Car {
    //&self takes the car object reference as a parameter
    fn print_car(&self){
        println!(
            "Car Body: {}\nCar Year: {}\nCar Color: {}\n",self.body,self.year,self.color)
    }
    fn new(b:String,y:u16,c:String)->Self{
        // this method replaces car data with new info
        // we can call a constructor for this!
        Car{
            body:b,
            year:y,
            color:c,
        }
    }
    fn change_color(&mut self, c:String){
        self.color=c;
    }
}
fn main() {
    let my_car = Car{
        body: "Sedan".to_string(),
        year: 2020,
        color: "Purple".to_string(),
    };

    my_car.print_car();
    let mut my_car = Car::new("SUV".to_string(),2024,"Black".to_string());
    println!("This is my new car information!");
    my_car.print_car();
    println!("Getting a paint job...");
    my_car.change_color("Red".to_string());
    my_car.print_car();
}
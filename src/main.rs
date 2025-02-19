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
            "Car Body: {}\n
            Car Year: {}\n
            Car Color: {}",self.body,self.year,self.color)
    }

}
fn main() {
    let my_car = Car{
        body: "Sedan".to_string(),
        year: 2020,
        color: "Purple".to_string(),
    };

    my_car.print_car();
    my_car.print_car();
}
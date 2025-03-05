
// enums are fixed set of variants not classes
enum Insurance{
    Car(String),
    House(u16),
}

impl Insurance{
    fn show_info(&self){
        // covers every case so no default needed _ =>
        match self{
            Insurance::Car(model)=>println!("My car model is {:?}",model), // String is model
            Insurance::House(year)=>println!("My house was built in {}",year), // u16 is year
        };
    }
}

fn main() {
    let car = Insurance::Car("BMW".to_string());
    let house = Insurance::House(2024);
    car.show_info();
    house.show_info();

}

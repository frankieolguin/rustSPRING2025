// Enums
// all user defined data
// describe soemthing in real world and fix possibilities
// fixed enums (RIGID) and variable Structs

// Insurance Enumerator
#[derive(Debug)] // what does this do???
enum Insurance{
    House,
    Car,
    Life
}
#[derive(Debug)]
struct Car {
    model:String,
}
// Person Struct
#[derive(Debug)]
struct Person{
    name: String,
    car:Car,
    insurances:Vec<Insurance>
}


impl Person{
    fn new(n:String,c:Car) -> Person {
        Person{
            name : n,
            car : c,
            insurances : vec![], 
        }
    }

    fn add_insurance(&mut self,i:Insurance){
        self.insurances.push(i);
    }
    
    fn show_insurance(&self){
        println!("My I am {:?}. I have the following insurances.\n",self);
        for i in self.insurances.iter(){
            match i {
                Insurance::Car =>println!("I have insured my {:?}",self.car),
                Insurance::House => println!("I have insured my house."),
                _ => println!("."),
            };
            //println!("{:?}",i);
        }
    }
}


fn main() {
    // double colon syntax, car belongs to Insurance Enum
    let c = Insurance::Car;
    let h = Insurance::House;
    let car = Car{
        model:"BMW".to_string(),
    };

    let mut person = Person::new("Francisco".to_string(),car);
    person.add_insurance(c);
    person.add_insurance(h);
    person.show_insurance();

    //println!("{:?}",person);
}

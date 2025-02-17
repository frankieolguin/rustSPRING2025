// freezing point of water in celsius
const FREEZING_POINT : f64 = 32.0;

fn main() {
    println!("5 temperature convertions incoming!\n");
    let mut temp : f64 = 27.0;
    for _ in 0..5{
        println!("{} fahrenheit in celsius is: {}",temp,fahr_to_cel(temp));
        println!("{} celsius in fahrenheit is: {}\n",temp,cel_to_fahr(temp));
        temp+=1.0;
    }
}

// takes float input and converts to fahrenheit
fn cel_to_fahr(c:f64)->f64{
    return(9.0/5.0)*c + FREEZING_POINT;
}

// takes float input and converts to celsius
fn fahr_to_cel(f:f64)->f64{
    return(5.0/9.0)*(f-FREEZING_POINT);
}
use std::io;

fn main() {
    let answer:i32 = 45;
    let mut guess:i32;
    let mut flag:i32=1;
    let mut counter:i32=0;
    // loops until we get a 0 from check_guess()
    while flag!=0{
        counter +=1;
        let mut buffer = String::new();
        // user input from io::stdin()
        println!("Enter an integer guess: ");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading input");
        // we parse the buffer into int32 guess
        guess = 
        buffer
            .trim()
            .parse()
            .unwrap_or(0);
        // we compare our guess to the answer
        flag = check_guess(guess,answer);
    }
    println!("Number of tries: {}\n",counter);
}
fn check_guess(guess:i32,secret:i32)->i32{
    let result:i32; 
    if guess==secret {
        result=0;
        println!("You guessed it!");
    }
    else if guess>secret{
        result=1;
        println!("You guessed too high!");
    }
    else {
        result=-1;
        println!("You guessed too low.");
    }
    println!("\n");
    return result
}
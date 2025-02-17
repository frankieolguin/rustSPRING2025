fn main() {
    // change these 5 values to different integers
    let arr : [i32;5]=[11,15,32,42,55];
    let mut _largest_num = 0;
    let mut _sum_of_arr = 0;
    // arr.iter return &i32 so i need to pass *num as
    // a parameter instead because &i32 is a reference to 
    // an integer and not the actual value itself
    for num in arr.iter(){
        let _by_3 = *num%3 ==0;
        let _by_5 = *num%5 ==0;
        let _is_even = is_even(*num);
        _sum_of_arr += *num;
        // print is num is even or odd
        println!("{} is {}.",num, if _is_even {"even"} else {"odd"});
        // FizzBuzz checker
        match(_by_3,_by_5){
            (true,false)=>println!("Fizz\n"),
            (false,true)=>println!("Buzz\n"),
            (true,true)=>println!("FizzBuzz\n"),
            _=>println!("simply {}\n ",*num),
        }
        // check for highest number
        if *num>_largest_num {_largest_num=*num}
    }
    println!("The largest number is your array is {}.\n",_largest_num);
    println!("The sum of all the integers in your array is {}.",_sum_of_arr);
    println!("[ {} + {} + {} + {} + {} ]",arr[0],arr[1],arr[2],arr[3],arr[4]);
}
 fn is_even(n:i32)->bool{
     return n%2==0;
 }
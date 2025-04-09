// in class assignment

//create a strucy student (major)

// hIGHER ORDER functions update majors
// first order functions, assign_major(student,major_declared)
// create a vectror of students 1,2,3 and update all students major
struct Student {
    major: String,
}

 fn update_major(data: &mut [Student], operation: fn (&mut Student, String)){
    for item in data.iter_mut() {
        let old_major = item.major.clone();
        operation(item,old_major);
    }
 }

 fn assign_major(s: &mut Student,major:String){
    s.major=major.to_string();
 }

 fn print_students(majors: &[Student]){
    print!("Majors: ");
    for (i, major) in majors.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", major.major);
    }
    println!();
 }



fn main() {
    let mut students = vec![
        Student {major: "Electrical Engineering".to_string()},
        Student {major: "Computer Science".to_string()},
        Student {major: "Business".to_string()},
        Student {major: "Computer Engineering".to_string()},
        Student {major: "Computer Science".to_string()},
    ];
    update_major(&mut students ,assign_major);
    // print outcome
    print!("After assigning majors: ");
    print_students(&students);

}

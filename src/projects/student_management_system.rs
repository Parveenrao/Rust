use std::collections::HashMap;

struct Student {
    marks : i32
}

enum ResultStatus{
    Pass,
    Fail
}

fn add_student(students: &mut HashMap<String , Student> , name:&str , marks : i32,) {

    students.insert(name.to_string() , Student {marks});

}

fn update_marks(students : &mut HashMap<String , Student> , name : &str , new_marks : i32){
    if let  Some(student) = students.get_mut(name){
        student.marks = new_marks;
    }

}


fn remove_students(students : &mut HashMap<String , Student> , name : &str ) {
    students.remove(name);
}

fn get_result(marks : i32) -> ResultStatus{
    if marks > 40 {
        ResultStatus::Pass
    }
    else{
        ResultStatus::Fail
    }
}

fn print_students(students :&HashMap<String , Student>){
    for (name , student) in students {
        let status = match get_result(student.marks){
            ResultStatus::Pass => "Pass",
            ResultStatus::Fail => "Fail",
        };

        println!("{} -> {} ({})" , name , student.marks , status);

    }
}

fn main(){
    let mut students : HashMap<String , Student> = HashMap::new();

    add_student(&mut students , "Alice" , 75);
    add_student(&mut students , "Bob" , 32);
    add_student(&mut students , "Charlie" , 55);

    update_marks(&mut students, "Bob", 45);

    remove_students(&mut students, "Charlie");

    print_students(&students);
}


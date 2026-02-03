// Structs own thier fields 

struct User{
    name : String,
    age:u32,
    

// WHen we create this 

let user = User{
    name: String::from("Parveen"),
    age :22,
    
}

// ownership move out to 
let name = user.name   

// user.name --> invlaid 


// Correct way to borrow fields
 
let name = &user.name 
println!("{}" , name);
println!("{}" , user.name)



//----------------------------------------------------------------------------------------

// Mutable borrow entire struct --> You can read all fileds but cant modify anything 

// Multiple immutable borrows allowed

// No mutation allowed

// Ownership stays with caller

struct User{
    name : String,
    age:u32,
}

fn print_user(&u : User) {
    println!("{}" , u.name);
    println!("{}" , u.age);
}

fn main(){

    let user = User{
        name : String::from("Parveen"),
        age :22
    };

    print_user(&user) // immutable borrow

    println!("{}" , user.name)

}


// Mutable borrow 

//   Only one mutable borrow at a time

//  No immutable borrows allowed at same time

//  Struct must be declared mut

fn birthday(u: &mut User) {
    u.age += 1;
}

fn main() {
    let mut user = User {
        name: String::from("Parveen"),
        age: 22,
    };

    birthday(&mut user); // mutable borrow
    println!("{}", user.age); // ✅
}

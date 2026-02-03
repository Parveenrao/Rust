// Struct IN Rust 

/*  A struct in a named fied of relative data with ownership rules
    
    Fileds have name 
    Code become self documenting 
    Long lived data   */


struct User{
    name : String,
    age : u32,
    active : bool,
}

/*fn main(){
    let user = User{
        name : String::from("HEllo"),
        age : 22,
        active : true,
    };

    // accessing field 

    println!("{}" , &user.name);      // if we dont use this & the ownership will move to println String doesnot implement copy
    println!("{}" , user.age);
    println!("{}" , user.active);
}*/ 


// Mutability in struct

fn main(){
    let mut  user = User{
        name : String::from("Parveen"),
        age : 22, 
        active : true,
    };

    user.age = 24;

    println!("{}"  , &user.age);
    println!("{}" ,  user.name); 
    println!("{}" , user.active);
}
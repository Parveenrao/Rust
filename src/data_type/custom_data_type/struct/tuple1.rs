// Passing Tuple by 

struct User{
    name : String ,
    age : u32,

}

fn conusme_user(u:User) {
    println!("{}" , u.name);
}

fn main(){
    let user = User{
        name : String::from("Parveen"),
        age : 22,
    };

    conusme_user(user);
}
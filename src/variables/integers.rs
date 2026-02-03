// in rust by default int are i32 
// in rust variables are immutable by default 

pub fn constant(){                                    // pub --> public means fn can be showed without pub fn cant be showed 
    println!("Variables in Rust");

    let x = 20;
    println!("x = {}"  , x);
}


// for mutable varible we should write let mut then we can change the value of y without mut shows error 


pub fn change(){
    println!("-----Variables in Rust-----");

    let x = 10;
    println!("x--{}" , x);

    let mut y = 20;
    println!("Y Before = {}" , y);

    y = 30;
    println!("Y After = {}" , y);
    

}
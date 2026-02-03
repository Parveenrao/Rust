/*  A function stored in a variable that can use from its surroundigs
    
    Function --> is a standlone 
    Closures --> function + environment 
    */

// Simple closure 

/* 
fn main(){
    let add = |a : i32 , b:i32 | -> i32 {
        a + b
    };

    println!("{}" , add(2, 3));


    // shorted syntx 

    let square = |x:i32| x * x;

    println!("{}" , square(5));

} */

// Capturing variable 


fn main(){
    let x = 10;

    let add_x = |a| a + x;

    println!("{}" , add_x(5));

    // x is NOT a parameter

    // Closure captures x from outside
}
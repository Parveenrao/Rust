/*  A Function is a block of code that has 
    1 A name 
    2 can take input 
    3 can return output  */

// Smallest function no input and output 


fn say_hello(){
    println!("Hello! , Rust");

}

fn main(){
    say_hello();
}



// Function with parameter 

fn greet(name : &str){
    println!("Hello {}" , name);
}

fn main(){
    greet("Parveen");
}



// Function that return value 

fn add(a : i32 , b: i32)-> i32 {
    a + b
}

fn main(){
    let sum = add(3, 4);
    println!("{}" , sum);    // return is optional

}



// Function with mulitple if statement 

fn is_even(n : i32) -> bool{

    if n% 2== 0 {
        true
    }else{
        false
    }

}

fn main(){
    let x = is_even(4);
    println!("{}" , x);
}



// Function returing result 

fn divide(a : i32 , b :i32) ->Result<i32 , String> { 
    
    if b == 0 { 
        Err("Divison by zero".to_string())
    }
    else{
        Ok(a/b)
    }

}

fn main(){
    let y = divide(2 , 0);
    println!("{:?}" , y);
}
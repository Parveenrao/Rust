/*  Rust does not crash byy default 
    Rust retuns value that represent failure


    => There are only two types 

    1. Options<T> ---------> maybe some value 
    2. Result <T> ---------> success or error 
*/


fn divide(a : i32 , b:i32) -> Result<i32 , String> {
    if b == 0 {
        Err("cannot divide by zero".to_string())
    }else {
        Ok(a / b)
    }
}



fn main () {

    let result = divide(10 , 0);


    match result {
        Ok(value) => println!("Result : {}" , value),
        Err(e) => println!("Error:{}" , e)
    }
}
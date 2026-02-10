// Error handling using  ? --> Remove match boilerplate


fn divide(a :i32 , b: i32) -> Result<i32 , String> {

    if b == 0 {
        Err("Cannot divide by zero".to_string())
    }else {
        Ok (a / b)
    }
}


fn calculate() -> Result<() , String>  {
    let result = divide(10 , 2)?;

    println!("{}" , result);
    Ok(())
}


fn main(){

    if let Err(e) = calculate() {
        println!("{}" , e);
    }
}
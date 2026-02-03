// Varibale shadowing IN RUST 

// Shadowing means redecalaring the same variable name using let  --> the new variable hides the old one 

pub fn shadow(){
    println!("----Variable Shadowing---");

    let x = 10;
    println!("x = {}" , x);

    let x = x + 5;
    println!("x after shadowing = {}" , x);

    let x = x * 2;
    println!("x after shadowing = {}" , x)


}

// Shadowing allows type change mut cannot 

pub fn type_change(){
    let x = 5;
    let x = "hello";
    println!("x after type change = {}" , x)
}


// mut  modify variable and shadowing crete new variable 

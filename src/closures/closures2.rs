// How closures capture variable 


// 1 Capture by immutable referce
 
 
fn main(){
    let x = 10;
    let show = || {
        println!("{}" , x);
    };

    show();
    show();

    // can be called many times
}



// 2 capture by mutable referce 


fn main(){
    let mut count = 0;

    let mut inc = ||{
        count+=1;
        println!("{}" , count);
    };

    inc();
    inc();


    // Implement Fnmut
}

     


// 3  Capture by value 

fn main(){
    let s  = String::from("Rust");

    let consume = ||{
        println!("{}" , s);
    };

    consume();

    // consume() cant be called again because ownership is moved to cosume and ownership is called only yonce 

    // to make call again we should borrow   (&s) so that we call again
}


// move keyword  Force closure to take ownership.


fn main(){
    let s = String::from("Rust");

    let consume = move || {
        println!("{}" , s);
    };

    print();

    // cannot be called again
}
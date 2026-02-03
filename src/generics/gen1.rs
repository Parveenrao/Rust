// Generics --- Write code once  and use it for many types 


// without generics 

// we have to do hard coding and duplication code and not scalable 
fn add_i32(a : i32 , b: i32) -> i32{         
    a + b
}

fn add_f64(a : f64 , b : f64) -> f64{
    a + b
}

//----------------------------------------------------------------------------------
// Solution --- >Generics 

// Basic Generic functio

fn identity <T>(x : T) -> T{
    x

    // For any value of type T , take value of T and return T

}

fn main(){
    let a = indentity(10);  // T:i32
    let b = indentity("hello"); // T:String
}



// Tuple is a group of multiple values of different data types into one compound value 

/*  Tuple is fixed value , 
    Can be mixed type, 
    access by index . 
    , short lived  */


// Creating Tuples 
/*
    let person = ("Parveen" , 22 , true);
    let point = (10 ,20);

    let empty = ();            //  ()  unit type 

// Acessing tuple element 

fn main(){
    let t  = (10 ,20 ,30);

    println!("{}" , t.0);
    println!("{}" , t.1);
    println!("{}" , t.2);

}



// Destructuring tuple

fn main(){

    let  t = (10 , "rust" , true);

    let (x , lang, flag) = t;

    println!("{}" , x);
    println!("{}" , lang);
    println!("{}" , flag);
}


//Mutability in Tuples (Entire tuple must be mut)

fn main(){

    let mut t = (10 ,20 , 30);

    t.0 = 99;
    
    println!("{:?}" , t)
}



// Tuple and Ownership 

fn main(){
    let t = (String::from("hello") , 10);

    let s  = t.0;

    // String is not Copy

    // Ownership moves out

    // println!("{:?}"  , t)  Error 
}



// Nested Tuples 

fn main(){
    let t = ((10 , 20 ,30) , (30 ,40 ,50));

    println!("{}"  , (t.0).1);
}


fn main(){
    let mut t = ((10, 20 ,30) , (40 , 50 ,60));

    (t.0).1 = 99;

    println!("{:?}" , t)
}

*/

// Return Tuple from funtion 

fn get_stats() -> (i32 , i32) {
    (10 ,20)
}

fn main(){
    let (min , max) = get_stats();

    println!("{} , {}" , min , max);
    }

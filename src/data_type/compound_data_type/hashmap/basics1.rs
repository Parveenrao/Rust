// Updating values in hashmap 

use std::collections::HashMap;


fn main(){
    let mut map : HashMap<String , u32> = HashMap::new();

    map.insert("maths".to_string() , 90);
    map.insert("maths".to_string() , 95); // overwrite old value

    println!("{:?}" , map);
}



// Update only if key exist 

fn main(){
    let mut map = HashMap::new();

    map.insert("math".to_string() , 90);

    if  let Some(score) = map.get_mut("math") {
        *score += 5
    }

    println!("{:?}"  , map);
}
   

// Insert or update using entry 


// If "math" exists → returns mutable reference

//If missing → inserts 0, then returns reference

// One lookup only (fast)

fn main(){

    let mut map = HashMap::new();

    let score = map.entry("Math").or_insert(0);
    *score+= 10;

    println!("{:?}" , map)
}

 


// update using iteration 

fn main(){
    let mut map = HashMap::new();

    map.insert("apple" , 80);
    map.insert("Banana" , 90);


    for prices in map.values_mut(){
        *prices*=2
    }

    println!("{:?}" , map);
}


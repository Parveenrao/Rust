// Looping    in hashmap 


// 1. loop through key and values 

use std::collections::HashMap;

fn main(){

    let mut map = HashMap::new();

    map.insert("Maths" ,  30);
    map.insert("Science" , 40);
    map.insert("Biology" , 50);

    for (key , values) in &map{
        println!("{} --> {}" , key , values);            // mutable borrows , map still usable after loop
    }

}


// looop and modify values 

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);

    for value in map.values_mut() {
        *value += 1;
    }

    println!("{:?}", map);
}


// loop over keys

for key in map.keys() {
    println!("{}", key);
}



// loop over values 

for key in map.values() {
    println!("{}", key);
}


// loop and take Ownership 

for (key, value) in map {
    println!("{} -> {}", key, value);
}


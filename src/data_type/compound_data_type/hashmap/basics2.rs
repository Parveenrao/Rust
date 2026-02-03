// Removing values in Hashmap 


use std::collections::HashMap;

fn main(){

    let mut map = HashMap::new();

    map.insert("maths" , 90);
    map.insert("Hindi" , 100);

    let removed = map.remove("maths");     // if not exist return none 

    //remove everything

    map.clear();
    
    println!("Removed : {:?}" ,removed);
    println!("Map Now : {:?}" , map);
}
   


// Condiition Removal using retain 

fn main(){

    let mut map = HashMap::new();
    map.insert("Alice" , 10);
    map.insert("Bob" , 20);
    map.insert("Charlie" , 30);


    map.retain(|_name , score| *score > 5);
    println!("{:?}" , map)
    
}
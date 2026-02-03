use std::collections::HashMap;

//  Hashmap store --> key - vlaue pair 

/*Example 

HashMap<String, i32>

HashMap<u32, User>

HashMap<Role, Status>

* Key properties

Keys are unique

Order is NOT guaranteed

Lookup is O(1) average

Stored on the heap   */


//---------------------------------------------------------------------------------------------






fn main(){
    let  map : HashMap<String ,u32> = HashMap::new();

    println!("{:?}", map)
}




// Basic INserting in hashmap 


fn main(){

    let mut map  = HashMap::<String , u32>::new();

    map.insert("Parveen".to_string(), 30);
    map.insert("Divya".to_string() , 40);

    println!("{:?}" , map);
}



// Basic inserting with string 


fn main(){

    let mut app:HashMap<String , u32> = HashMap::new();

    app.insert("Maths".to_string() , 90);
    app.insert(String::from("Physics") , 70);

    println!("{:?}" , app);
    

    // String ownership moved to map 

    // If key is new → value inserted

    // If key exists → value is replaced
}


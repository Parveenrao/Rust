
// String is HEAP  allocated , Growthable , Shirkable  , OWN itn on data
// String --> owned text stored in heap 

pub fn string_operation_demo(){
    // create heap allocated string 

    let mut s = String::from("par");

    // Push Operation 
    s.push('v');              // parv
    s.push_str("een");       // een


    // length and capacity 

    let length = s.len();                // bytes used
    let capacity = s.capacity();         // allocated

    println!("String : {}" , s);
    println!("Length {} , Capacity {}" , length , capacity);

    // insert operation 
    s.insert( 0 , 'X');
    s.insert_str(1 , "Yz");

    println!("After insertion {}" , s);

    // Remove operation 

    let removed_char =  s.remove(0);
    println!("Removed char {}" , removed_char);
    println!("After removed {}" , s);


    // Pop - last character 

    let last = s.pop();
    println!("Poppped char {:?}" , last);
    println!("After pop {}" , s);

    // Replace -- Replace creates new string 

    let replace = s.replace("par" , "PAR");
    println!("Replaced String {}" , replace);



    // 9️⃣ Truncate string
    s.truncate(4);
    println!("After truncate: {}", s);

    // 🔟 Clear string
    s.clear();
    println!("After clear: '{}'", s);


}

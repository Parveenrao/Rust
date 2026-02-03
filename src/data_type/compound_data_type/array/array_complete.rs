// Array complete example 

fn main(){

    // Array Intialization 

    let mut arr = [2, 4, 6 ,8];

    println!("Initial Array {:?}" , arr);

    // Read element

    let first = arr[0];
    let second = arr[1];

    println!("First element {:?}" , first);
    println!("Second element {:?}" , second);

    // Modify 

    arr[1] = 3;
    arr[2] = 7;

    println!("After modify element {:?}" , arr);

    // safe indexing using .get

    match arr.get(10) {
        Some(value) => println!("Value at index 10: {}" ,value),
        None => println!("Index 10 is out of  bound"),


    }

    // safe modifying using get_mut

    if let Some(value) = arr.get_mut(1){
        *value = 100;
    }

    println!("After get_mut  : {:?}", arr);


    // loop based modification 

    for i in 0..arr.len(){
        arr[i]  += 2;
    }

    println!("After loop : {:?}" , arr);


    // modification using get_mut 

    for x in arr.iter_mut(){
        *x *=2;
    }

    println!("Final Array: {:?}" , arr)



}
// len of vector -- > number of elements 

/* main(){

    let mut v = Vec::new();

    v.push(10);
    v.push(20);

    println!("Lenght : {}" , v.len());
}


// Capacity --> Allocated space 

fn main(){
    let mut v = Vec::new();

    println!("Capacity : {}" , v.capacity());

    v.push(10);
    println!("Capacity after 1 push : {}" , v.capacity());

    v.push(20);
    println!("Capacity after 2 push : {}" , v.capacity());
}
*/

// Clear --> remove element but keep memory 

fn main() {
    let mut v = vec![1, 2, 3];

    println!("Before clear: len={}, cap={}", v.len(), v.capacity());

    v.clear();

    println!("After clear: len={}, cap={}", v.len(), v.capacity());
}

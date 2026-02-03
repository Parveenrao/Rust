// Intialization 

fn main(){

    let mut v  = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}" , v);

}


fn main(){
    // create a new empty vector 

    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let y = v[1];
    println!("{}" , y);

    // safe acess using get 

    match v.get(2){
        Some(x) => println!("{}" , x),
        None => println!("Index out of bound"),
    }

    // modify element using mut 

    match v.get_mut(1){
        Some(val) => {
            *val = 99;
            println!("Modified index 2");
        }
        None => println!("Index not found"),

    }

    // print final vector 

    println!("Final vector: {:?}" , v)
}

// Pop ---> Remove last element and Remove --> index element whihc u want to remove

fn main(){
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let removed = v.pop();

    match removed {
        Some(val) => println!("Popped Value : {}" , val),
        None => println!("vector was empty")
    }

    println!("After pop {:?}" , v)



}

fn main(){

    let mut v = vec![10 ,20 ,30];

    let removed  = v.remove(0);

    println!("Removed value :{}" , removed);
    println!("After removal vector : {:?}" ,v);
}
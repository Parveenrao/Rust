
fn main(){
    let arr = [10 , 20 ,30 ,40];

    println!("{}" , arr[0]);
    println!("{}" , arr[1]);
}


// modify arr element 

fn main() {
    let mut arr = [10, 20, 30, 40];

    arr[0] = 11;
    arr[2] = 55;

    println!("{:?}", arr);
}
   

// Safe indexing using .get 

fn main(){
    let arr = [1, 2, 3, 4, 5];

    match arr.get(3) {

        Some(val) => println!("Value {}" , val),
        None => println!("Index out of bound"),

    }
}  


// Safe modifying with .get_mut

fn main(){
    let mut arr = [10 ,20, 30];
    if let Some(x) = arr.get_mut(1){
        *x = 99
    }

    println!("{:?}", arr); // [10, 99, 30]
}

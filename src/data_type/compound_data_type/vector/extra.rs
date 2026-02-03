// Filtering Elements Based on Condition 

fn main(){

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    v.retain(|x| x %2 != 0);
    println!("{:?}" , v)
}



fn main(){

    let mut  v = vec![12, 10 ,34, 12, 15];
    
    v.sort();

    println!("{:?}" , v);

    if v.is_empty(){
        println!("Vector is empty");
    }
}



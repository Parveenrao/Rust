// Loop based modification 

fn main(){
    let mut arr = [1 , 2, 3, 4, 5];

    for i in 0..arr.len(){
        arr[i] *= 2;
    }

    println!("{:?}" , arr)
}    


// using iter_mut 

fn main(){
    let mut arr = [2,4,6,8,10];

    for x in arr.iter_mut(){
        *x+= 1
    }

    println!("{:?}" , arr);
}
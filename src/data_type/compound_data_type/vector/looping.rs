// Read only loop 
// borrows element immutably  , no ownership move 

/*v was borrowed, not moved

Ownership still with v

Vector is still alive

Memory still allocated 
*/

fn main(){

    let v = vec![10 ,20 ,30];

    for x in &v{
        println!("{}" , x);
    }

    println!("After loop {:?}" , v);
} 


/*Mutable loop(Modify elements)  

Temporary mutable borrow ends

Ownership returns to v

Vector is alive and modified

*/

fn main(){
    let mut v = vec![10 ,20 ,30];

    for x in &mut v{
        *x += 1;
    }

    println!("{:?}" , v)
}




/*  Ownership move  
v was moved into the loop

Ownership transferred

Vector memory freed

v is dead
*/

fn main(){

    let v = vec![10 ,20 ,30];


    for x in v{
        println!("{}" , x)
    }
}



// Iterator is an object that produce a sequence of values ,  one at time , on demand 

// Values are produced lazily , One element at time , No work is done  unitl you ask for next value 

// ---------------------------------------------------------------------------------------------------

// 1.   What is iter()?

        // iter() creates an iterator that borrows elements immutably.

        // It does not take ownership.
        // It does not modify data.


fn main(){
    let v = vec![10 ,20 ,30];

    for x in v.iter(){
        println!("{}" , x);
    }
}


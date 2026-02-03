// Array in Rust 

/*  Fixed size allocation 
    All element are of same types 
    Size known at compile time 
    Stored on stack memory
*/

// Array are contigous in memory , each element is next to other 


// Global constants (must have type)
const ARR: [i32; 3] = [1, 2, 3];
const ARR1: [i32; 5] = [1, 2, 3, 4, 5];
const ZEROS: [i32; 5] = [0; 5];

fn main() {
    // Print arrays (use {:?} for arrays)
    println!("ARR   : {:?}", ARR);
    println!("ARR1  : {:?}", ARR1);
    println!("ZEROS : {:?}", ZEROS);
}





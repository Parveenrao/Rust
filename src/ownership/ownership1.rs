/* Most language memory in two ways 

1 . Garbage Collector -->  Python , Java ---> Runtime cost
2 . Manual memory -----> C/C++ ----> bugs crash


* Rust gives memory safety + performance ---> It does through ownership rules enforces at compile time 

---------------------------------------------------------------------------------------------------------
=> Rust enforce only three rules 

   1. Each value has exactly one owner 
   2. When owner goes out of scope , value is dropped
   3. There can't be mulitple owner without exlicipt rules

-------------------------------------------------------------------------------------------------------------

* Stack vs Heap Memory in Rust 

1. Stack => 
    Fixed size ,  Fast , value copied easily

    let x = 5;
    let y = x;  // copied value 

    // work becuase i32 is stored on  stack 

2. Heap => 
   Dynamic , Slow , Need explicit ownership


   let s1 = String::from("Parveen")
   let s2 = s1;  // ownership moved 

   s1 => is dropped

   String lives in heap 
   Rust move ownership  , not copies
   Prevent double free bugs

* Copy vs Move

=> data That lives on stack will be copied --> int , tuple , bool. char
=> data that lives on heap will be moved  --> vec<T> , hashmap<k ,V> , String




* Want to two heap values Clone(Explicity Copying)

  let s1 = String::from("Rust")
  let s2 = s1.clone();

  println1("{}" , s1)  // no error

 */


// Ownership in Function 
 
  fn take(s : String) {
    println!("{}" ,s);
  }

  fn main(){
    let s  = String::from("Rust");
    take(s);
    println!("{}" , s);     // invalid because the ownership goes to take()

  }


// Think of ownersship like this who owns this data 

   // if ownership change ----> original variable become invalid 


fn main(){
    let s1 = String::from("rust");
    let s2 = s1;

    let s3 = s2.clone();

    println!("{}" , s1);
}     


/*  Solution in this code  ---> s1 owns heap memory of string   , s1 is the owner
                            
                           --->   move happens s2 become owner , s1 in invalid after move 

                           --->   after let s3 = s2.clone() , clone() create a new heap allocatio , Deep copy of hello

*/
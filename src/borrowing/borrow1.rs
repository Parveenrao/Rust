
/* 
Borrowing Means => Temporay access to data without taking ownership  
                    Ownership stays with the original owner 
                    Borrowers cannot outlive the owner
                    
You can look or temporarily use data , but u dont get to own them

 => Two kind of borrowing
    
    1. Immutable borrow --> read only   (&T)

    2. Mutable borrow ---> read and write (& mut T)


 => Immutable borrow 

   fn borrow(s : &string) {
       println!("{}" , s);
       
    }

    fn main() {
       let s = String::from("Paveen");
       
       borrow(&s);
       
       println1("{}" , s)   /// still valid because the ownership isn;t moved only borroowed
       }   


       // s keep ownership 
          only read no write  , no move no clone 



    you can any number of immutable borrowers 

    let s = String::from("Parveen");
    let s1 = &s
    let s2 = &s   // only read no write 


 => Mutable borrow 

    fn change(s : &mut String) {
            s.push_str("hi")
            }   
    
    fn main() {
        
        let mut s = String::from("Parveen");
        change(&mut s);

        }
       

    only one mutable borrow at one time 




    => Not more tha one mutable borrows 

    because 

    let mut s = String::from(:Rust);

    let s1 = &mut s
    let s2 = &mut s


    two write ==> Data corruption 



                    */



/* 
🔥The Core Rule (Interview Gold)



At any time, you can have either one mutable reference or any number of immutable references — but not both.

This single rule:

Prevents data races

Prevents iterator invalidation

Enables fearless concurrency */                    



/*    

6️⃣ Mutable + Immutable Borrow Error (Classic)
let mut s = String::from("hi");

let r1 = &s;
let r2 = &mut s; // ❌ error


Why?

Reader exists

Writer requested

Compiler refuses to allow inconsistent view

This prevents use-after-mutation bugs. */




/*     

on-Lexical Lifetimes (NLL) — Clear Example


What is NLL?

Non-Lexical Lifetimes mean:

A borrow ends when it is last used, not when its scope ends.

Old Rust (pre-2018) used lexical lifetimes → borrow lasted until the end of the block.
Modern Rust uses NLL → borrow ends as soon as it’s no longer needed. 

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;            // immutable borrow
    println!("{}", r1);    // last use of r1

    let r2 = &mut s;        // mutable borrow (ALLOWED due to NLL)
    r2.push_str(" world");

    println!("{}", s);
}



*/
/*  A Dangling pointer/reference to that memory that is already freed 

    => In simple words
       Data is destroyed
       Pointer still exist
       Pointer tries to use dead memory --> crash / undefined behaviour
       
       
 
 
 => Exmample C++ 
     int* foo() {
        int x = 10;
        return *x                // returning address of local variable 
           }      


    int main() {
    int* p = foo()
    printf("%d" , *p)
    }       


    x lives inside foo 
    foo ended --> x destroyed 
    p still point to old memory 

//------------------------------------------------------------------------------

 => Rust solution 

    You cannot give someone the address of a house that will be demolished.


    * Example fn foo() -> i:32 { 
                 let x = 10;
                 &x 
                 }
        
        foo() ends → house destroyed → x destroyed


        Rust asks:

        “Will x still exist when someone uses this reference?”

        Answer: ❌ NO
        
        So Rust stops compilation.



        // Dangling Reference in Rust 



        fn main() {
           let r;

           {
              let x = 5;
               r = &x;   // ❌
            }

               println!("{}", r);
                }
  
  
  
  
  
 //  fn main() {
       let s = String::from("Rust");
       let r = &s;

       drop(s);
       println!("{}", r);

        r---> point to dead data


        }
 
  
  */

// Const is a compile time constant    
// value is known before compile 
// value is inlined everywhere 

const x : i32 = 5; // type is require in const 

const x = 6 ;  // error 

// must be compile time evaluable 
 const X : i32 = 2 + 3

// cannot be muted 

const A : i32 = 10 
A= 20 //error

// cannnot be shadowing 

const A :i32 = 20 
const A :i32 = 30  //error 


// Valid const expression 

const A: i32 = 10;
const B: i32 = A + 5;
const C: i32 = 2 * 3;


// vec<T> -->  Also called vector / dynamic array 

/*  Vector has  a three part 

┌────────┐
│ ptr    │ → points to heap memory
├────────┤
│ len    │ → number of elements
├────────┤
│ cap    │ → allocated capacity
└────────┘

Vec<T> = (pointer, length, capacity)
-------------------------------------------------------------------------------
* why Vector exist   

 arrays (Size known at compile time )
         stored on stack   

 Vector(are dynamic  , stored on head  , size known at runtime)

 can grow or shrink 
 ---------------------------------------------------------------------------------

Vec<T> lives partly on stack, partly on heap:

Metadata (ptr, len, cap) → stack

Actual elements → heap


vector can store different types of element but all most of same type 

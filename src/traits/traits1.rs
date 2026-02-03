// A Trait define behavior 

// If a type implement this trait  , it promises to provide these methods

// traits acheive compilte time polymorphism

trait Speak{
    fn speak(&self);

}


// implementing a trait for struct

struct Dog;
struct Cat;


impl Speak for Dog{
    fn speak(&self){
        println!("Dog barks")
    }
}


impl Speak for Cat{
    fn speak(&self){
        println!("Cat mewos")
    }
}


fn main(){
    let d = Dog;
    let c  = Cat;


    d.speak();
    c.speak();
}

/*  Anything that can start is called an Engine.”

We don’t care how it starts. Just that it can.

This rule = trait    */
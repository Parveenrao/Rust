// Example Anyone humand and animal can speak so we have speak function 

trait Speak{
    fn speak(&self);
}


struct Human;

struct Dog;


impl Speak for Human{
    fn speak(&self){
        println!("Human talks")
    }
}

impl Speak for Dog{
    fn speak(&self){
        println!("Dog barks")
    }
}



// Now we creat a function for any one that can speak

fn make_sound(s : &impl Speak){

    s.speak()
}



fn main(){
    let h = Human;
    let d = Dog;

    make_sound(&h);
    make_sound(&d);
}
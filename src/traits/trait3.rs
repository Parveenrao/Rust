// impl --- compilte time polymorphism 
// dyn  --- run time polymorphism


/* dyn traits 

differnt structs 

same behavior

store them together 

call method at runtime   */


// define trait 

trait Speak{
    fn speak(&self);

}

// define structs data 

struct Dog;
struct Human;


impl Speak for Dog{
    fn speak(&self){
        println!("Dog Barks");
    }
}


impl Speak for Human{
    fn speak(&self){
        println!("Human Talks");
    }
}


fn make_sound(x: &dyn Speak) {
    x.speak();
}


fn main() {
    let dog = Dog;
    let human = Human;

    make_sound(&dog);
    make_sound(&human);
}

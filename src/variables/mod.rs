mod integers;
mod shadowing;

pub fn run(){
    integers::constant();
    integers::change();
    shadowing::shadow();
    shadowing::type_change()
}
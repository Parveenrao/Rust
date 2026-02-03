
// &str ---> what actually is &str 
// &str is borrowed , read only text view 
// &str only point to text that already exist
// No mutable , no push 

pub fn run(){
    println!("&str Basics");

    let s : &str = "parveen";  // '' -> char , " " -> string
    println!("String = {}" , s);
    println!("Length (bytes) = {}" , s.len());

}
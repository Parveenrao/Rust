
pub fn run(){
    println!("Operation in &str");


    let s = "rust programming";

    println!("Is Rust ? {}" , s.starts_with("rust"));

    println!("Ending with ing ? {}" , s.ends_with("ing"));

    //search

    println!("Find pro ? : {:?}" , s.find("pro"));

    //splits

    for words in s.split(' '){
        println!("Words = {}" ,words);
    }

    // iterate characters 

    for ch in s.chars(){
        println!("Character {}" , ch);
    }
    println!();
}
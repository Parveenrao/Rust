

pub fn run(){
    println!("Slicing in &str");

    let s = "parveen";

    let a = &s[0..3];
    let b = &s[3..7];
    println!(" {} , {}" , a , b);


   

    let a = &s[..3];   // from start
    let b = &s[3..];   // till end
    let c = &s[..];    // full string

    println!("{} {} {}", a, b, c);

}


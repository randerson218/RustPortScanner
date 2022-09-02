use std::env;

fn main() {
    //get all args passed
    let args: Vec<String> = env::args().collect();

    //print out all the args
    for i in &args {
        println!("{}",i);
    }

}

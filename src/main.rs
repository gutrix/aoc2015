mod solutions;

fn main() {
    for i in 1..26 {
        println!("{} - {:?}", i, solutions::s(i));    
    } 
}

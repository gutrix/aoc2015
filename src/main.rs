mod solutions;

fn main() {
    for i in 1..=25 {
        let (a, b) = solutions::s(i).unwrap();
        println!("{} - {:>5?}\t{:>5?}", i, a.unwrap_or(0), b.unwrap_or(0));    
    } 
}

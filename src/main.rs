use std::error::Error;

mod solutions;

fn main() -> Result<(), Box<dyn Error>>{
    for i in 1..=25 {
        let (a, b) = solutions::s(i)?;
        println!("{} - {:>5?}\t{:>5?}", i, a.unwrap_or(0), b.unwrap_or(0));    
    }
    Ok(())
}

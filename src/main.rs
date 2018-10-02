fn main() {
    println!("Hello, world!");
}

use std::sync::Arc; // wtf?

#[cfg(test)]
mod tests {

    // it should be here
    use std::sync::Arc;

    fn test() {
        let a = Arc::new(0);
    }
}
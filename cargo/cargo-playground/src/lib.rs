pub fn greet() {
    println!("hellow world from lib.rs")

}

#[cfg(test)]
mod tests {
    #[test]
    fn library_test() {
        assert_eq!(2+2, 4);
    }
}




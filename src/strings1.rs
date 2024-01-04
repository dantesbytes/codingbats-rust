
pub mod string_utils {
    
    pub fn hello_name(s: &str) -> String {
        format!("Hello {}! ", s)
    
    }

}

#[cfg(test)]
mod tests {
    
    use super::string_utils;
    #[test]
    fn test_hello_name() {
        
        let result = string_utils::hello_name("Dante");
        assert_eq!(result, "Hello Dante!");
    }
    
    #[test]
    fn test_hello_name_without_space() {
        let result = string_utils::hello_name("Jack");
        assert_ne!(result, "HelloJack!");
    }
}
pub mod calc1;
pub mod calc2;

#[cfg(test)]
mod tests {
    use super::calc1::{add, sub};
    use super::calc2::{multiply, rate};
    
    #[test]
    fn test_add() {
        assert_eq!(add(10, 20), 30);
        assert_eq!(add(u32::MAX, 1), u32::MAX); //overflow no u32 satura no maximo
    }
    
    #[test]
    fn test_sub() {
        assert_eq!(sub(30, 10), 20);
        assert_eq!(sub(u32::MIN, 1), u32::MIN);
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(30, 2), 60);
        assert_eq!(multiply(u32::MAX, 2), u32::MAX);
    }
    
    #[test]
    fn test_rate() {
        assert_eq!(rate(30, 2), 15);
        assert_eq!(rate(30, 0), 0);
    }
}
use crate::is_root::Root;


pub struct CpuIsRoot;

impl Root for CpuIsRoot {
    fn is_root(squareroot: &Vec<f64>, input: &Vec<f64>, delta: f64) -> Option<bool> {
        if squareroot.len() != input.len() {
            return None;
        }
        for i in 0..squareroot.len(){
            let squareroot_squared = squareroot[i] * squareroot[i];
            let squareroot_squared_minus_input = squareroot_squared - input[i];
            if squareroot_squared_minus_input.abs() > delta  {
                return Some(false);
            }
        }
        Some(true)
    }
}

#[cfg(test)]
mod test {
    use crate::is_root::Root;
    use crate::cpu_is_root::CpuIsRoot;

    #[test]
    fn test_is_root_i32_positive(){
        let root: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let input: Vec<f64> = vec![1.0, 4.0, 9.0, 16.0];
        let is_root_result = CpuIsRoot::is_root(&root, &input, 0.001);
        assert!(is_root_result.is_some());
        assert_eq!(is_root_result.unwrap(), true);
    }
    #[test]
    fn test_is_root_f64_positive(){
        let root = vec![1.0, 2.0, 3.0, 4.0];
        let input = vec![1.0, 4.0, 9.0, 16.0];
        let is_root_result = CpuIsRoot::is_root(&root, &input, 0.001);
        assert!(is_root_result.is_some());
        assert_eq!(is_root_result.unwrap(), true);
    }
    #[test]
    fn test_is_root_returns_none_if_length_missmatched(){
        let root = vec![1.0];
        let input = vec![1.0, 4.0, 9.0];
        let is_root_result = CpuIsRoot::is_root(&root, &input, 0.001);
        assert!(is_root_result.is_none());
    }
}
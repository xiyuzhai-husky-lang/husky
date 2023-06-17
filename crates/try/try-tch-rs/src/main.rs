pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[test]
fn test_it() {
    let a = tch::Tensor::from_slice(&[0.0, 1.0]).set_requires_grad(true);
}

fn main() {
    let a = tch::Tensor::from_slice(&[0.0, 1.0]).set_requires_grad(true);
    // let  a = tch::Tensor::
}

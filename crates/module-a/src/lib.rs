#[cfg(feature = "x")]
pub fn feat_x() {
    println!("crate module-a with feature x");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "x")]
    #[test]
    fn it_works() {
        feat_x();
    }
}

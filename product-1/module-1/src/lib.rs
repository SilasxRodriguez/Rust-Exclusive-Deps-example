#[cfg(feature = "a")]
pub fn feat_a() {
    println!("feature a");
}

#[cfg(feature = "b")]
pub fn feat_b() {
    println!("feature b");
}

#[cfg(feature = "c")]
pub fn feat_c() {
    println!("feature c");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "a")]
    #[test]
    fn feature_a_works() {
        feat_a();
    }

    #[cfg(feature = "b")]
    #[test]
    fn feature_b_works() {
        feat_b();
    }

    #[cfg(feature = "c")]
    #[test]
    fn feature_c_works() {
        feat_c();
    }
}

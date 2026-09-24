#[cfg(all(feature = "a", feature = "c"))]
compile_error!("Unable to build product-1 with a and c features");

#[cfg(feature = "a")]
use module_a;

fn main() {
    println!("Entering product-1!");
    #[cfg(feature = "a")]
    {
        module_1::feat_a();
        module_a::feat_x();
    }

    #[cfg(feature = "b")]
    {
        module_1::feat_b();
    }

    #[cfg(feature = "c")]
    {
        module_1::feat_c();
    }
}

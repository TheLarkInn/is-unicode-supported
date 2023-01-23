extern crate is_unicode_supported;

fn main() {
    if is_unicode_supported::is_unicode_supported() {
        println!("Unicode is supported here!!! 🦀🦀🦀");
    } else {
        println!("Unicode is not supported here! ;(");
    }
}
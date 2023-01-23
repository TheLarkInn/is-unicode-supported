# is-unicode-supported
Detect whether the terminal supports unicode or not. Inspiration comes from [sindresorhus/is-unicode-supported](https://github.com/sindresorhus/is-unicode-supported). Useful for deciding whether to use Unicode or ASCII characters in command-line output.

Words from [Sindre](https://github.com/sindresorhus):

> Note that the check is quite naive. It just assumes all non-Windows terminals support Unicode and hard-codes which Windows terminals that do support Unicode. However, I have been using this logic in some popular packages for years without problems.

## Usage 
`$> cargo add is-unicode-supported`

_main.rs_
```rust

use is_unicode_supported::is_unicode_supported;

fn main() {
    if is_unicode_supported() {
        println!("Unicode is supported here!!! 🦀🦀🦀");
    } else {
        println!("Unicode is not supported here! ;(");
    }
}
```


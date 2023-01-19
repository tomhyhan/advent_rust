// use hex_literal::hex;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

fn main() {
    let mut hasher = Md5::new();
    // hasher.input_str("abcdef6");
    // let r = hasher.result_str();
    // println!("{r:?}");
    let text = "bgvyzdsv";
    let mut idx = 0;

    loop {
        hasher.input_str(&format!("{text}{idx}"));
        if hasher.result_str().starts_with("000000") {
            println!("{}", hasher.result_str());
            println!("{}", idx);
            break;
        } else {
            hasher.reset();
            idx += 1;
        }
    }
}

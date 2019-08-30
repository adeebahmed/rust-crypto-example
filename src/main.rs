extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use crypto::sha2::Sha512;
use crypto::sha2::Sha256;

fn main() {
    sha3_256();
    sha2_512();
    sha2_256()
}

fn sha3_256() {
    // create a SHA3-256 hasher
    let mut hasher = Sha3::sha3_256();

    // write input message
    hasher.input_str("some_random_message");

    // read hash digest
    let hex = hasher.result_str();
    let hex_15 = &hex[0..15];

    println!("SHA3-256 Hash is {:?}", hex);
    println!("SHA3-256 Hash (15) {:?}", hex_15);
}

fn sha2_512() {
    // create a SHA2-512 hasher
    let mut hasher = Sha512::new();

    // write input message
    hasher.input_str("some_random_message");

    // read hash digest
    let hex = hasher.result_str();
    let hex_15 = &hex[0..15];

    println!("SHA2-512 Hash is {:?}", hex);
    println!("SHA2-512 Hash (15) {:?}", hex_15);
}

fn sha2_256() {
    // create a SHA2-256hasher
    let mut hasher = Sha256::new();

    // write input message
    hasher.input_str("some_random_message");

    // read hash digest
    let hex = hasher.result_str();
    let hex_15 = &hex[0..15];

    println!("SHA2-256 Hash is {:?}", hex);
    println!("SHA2-256 Hash (15) {:?}", hex_15);
}
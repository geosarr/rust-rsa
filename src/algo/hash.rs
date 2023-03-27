pub struct Hasher {}

impl Hasher {
    pub fn hash(algo: String, msg: String) {
        if algo != "md5" {
            println!("The only supported hash is md5")
        }
        println!("Hashing not yet implemented ! Your message is: {}", msg);

        let mut s: [u8; 64] = [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20,
            5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
        ];
    }
}

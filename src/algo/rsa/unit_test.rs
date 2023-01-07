#[cfg(test)]
mod tests{
    use super::super::RSA;
    use num_bigint::{ToBigUint};

    #[test]
    fn test_rsa_signature(){
        let rsa = RSA::init(512, 3);
        for int_msg in 0..100{
            let msg = int_msg.to_biguint().unwrap();
            let sig = rsa.sign(&msg);
            assert_eq!(rsa.decrypt_sign(&sig), msg);
        }
    }
}
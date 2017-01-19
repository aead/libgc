
use super::gate::CopyRand;

pub trait Cipher<K: CopyRand>{

    fn encrypt(in_a: K, in_b: K, out: K) -> K;

    fn decrypt(in_a: K, in_b: K, enc: K) -> K;
}
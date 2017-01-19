
extern crate rand;

pub trait CopyRand : Copy + Sized {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self;
}

type Label128 = (u64,u64);

impl CopyRand for Label128 {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self{
        (rng.next_u64(), rng.next_u64())
    }
}

pub struct TrueTable<K: CopyRand> {
    entry0: [K; 3],
    entry1: [K; 3],
    entry2: [K; 3],
    entry3: [K; 3],
}

impl<K: CopyRand> TrueTable<K> {

    pub fn new_and<R: rand::Rng>(rng: &mut R) -> TrueTable<K> {
        let a_0: K = K::rand(rng);
        let a_1: K = K::rand(rng);
        let b_0: K = K::rand(rng);
        let b_1: K = K::rand(rng);
        let c_0: K = K::rand(rng);
        let c_1: K = K::rand(rng);
        TrueTable{
            entry0: [a_0, b_0, c_0],
            entry1: [a_0, b_1, c_0],
            entry2: [a_1, b_0, c_0],
            entry3: [a_1, b_1, c_1],
        }
    }

    pub fn new_or<R: rand::Rng>(rng: &mut R) -> TrueTable<K> {
        let a_0: K = K::rand(rng);
        let a_1: K = K::rand(rng);
        let b_0: K = K::rand(rng);
        let b_1: K = K::rand(rng);
        let c_0: K = K::rand(rng);
        let c_1: K = K::rand(rng);
        TrueTable{
            entry0: [a_0, b_0, c_0],
            entry1: [a_0, b_1, c_1],
            entry2: [a_1, b_0, c_1],
            entry3: [a_1, b_1, c_1],
        }
    }

}

pub type Register = u8;
pub type Gate = fn(a: Register, b: Register) -> Register;

fn xor(a: Register, b: Register) -> Register {
    a ^ b
}

fn and(a: Register, b: Register) -> Register {
    a & b
}

fn or(a: Register, b: Register) -> Register {
    a | b
}

pub fn And() -> Gate{ and }

pub fn Xor() -> Gate{ xor }

pub fn Or() -> Gate{ or }

pub fn Not(a: Register) -> Register{
    (a & 1) ^ 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (r0, r1, r2, r3, r4, r5) = (0, 1, 0, 1, 0, 1);

        let res0 = And()( r1, (Xor()(r0, r5)) );
        let res1 = Or()( res0, (And()(r2, r1)) );
        let res2 = Xor()( r4, (Xor()( res0, (And()(r5, res1)) ) ));
        let res3 = And()(Not(res1), r3);
        let result = Xor()( And()(res0, res1), Xor()(res2, res3) );

        println!("Result_0: {0} | Result_1: {1}",res0, res1);
        println!("Result_2: {0} | Result_3: {1}",res2, res3);
        println!("Result: {0}",result);
    }

    #[test]
    fn verify_and(){
        assert!(And()(0,0) == 0, "0 and 0 != 0");
        assert!(And()(0,1) == 0, "0 and 1 != 0");
        assert!(And()(1,0) == 0, "1 and 0 != 0");
        assert!(And()(1,1) == 1, "1 and 1 != 1");
    }

    #[test]
    fn verify_xor(){
        assert!(Xor()(0,0) == 0, "0 xor 0 != 0");
        assert!(Xor()(0,1) == 1, "0 xor 1 != 1");
        assert!(Xor()(1,0) == 1, "1 xor 0 != 1");
        assert!(Xor()(1,1) == 0, "1 xor 1 != 0");
    }

    #[test]
    fn verify_or(){
        assert!(Or()(0,0) == 0, "0 and 0 != 0");
        assert!(Or()(0,1) == 1, "0 and 1 != 1");
        assert!(Or()(1,0) == 1, "1 and 0 != 1");
        assert!(Or()(1,1) == 1, "1 and 1 != 1");
    }

    #[test]
    fn verify_not(){
        assert!(Not(0) == 1, "not 0 != 1");
        assert!(Not(1) == 0, "not 1 != 0");
    }
}

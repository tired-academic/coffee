//! Java-like LCG Randomization
//! 
//! Since Minecraft relies on the Java utility known as `java.util.Random`,
//! this project needed a way to consistently apply its flavour of random
//! without trying to build some cursed `Rust<-->C<-->Java` bridge. That's
//! how this module was born!
//! 
//! `seed = (seed * 0x5DEECE66D + 0xB) & ((1L << 48) - 1)`
//! 
//! This is not a 1:1 recreation however, this doesn't expose a complete
//! copy of the `.util.Random` functions, only what is strictly needed.
//! Whatever you do to optimize this module, **do not modify the formula.**
//! **Ever.** That will change RNG and will leave us no longer in line with
//! the target.
//! 
 
const MULT: u64 = 0x5DEECE66D;
const MASK: u64 = (1 << 48) - 1;
const ADDEND: u64 = 0xB;

pub struct JRandom {
    seed: u64
}

impl JRandom {
    /// Builds a new random instance, **must** be seeded! Unseeded new calls will
    /// not work at the current moment.
    pub fn new(seed: i64) -> Self {
        Self { seed: (seed as u64 ^ MULT) & MASK, }
    }

    /// Internal function that actually advances the Random state.
    fn next(&mut self, bits: u32) -> u32 {
        assert!((1..=32).contains(&bits));
        self.seed = self.seed.wrapping_mul(MULT).wrapping_add(ADDEND) & MASK;

        (self.seed >> (48 - bits)) as u32
    }

    /// Pulls the next 32 bits (int) from the JRandom state. Returns them as a signed int.
    pub fn next_int(&mut self) -> i32 {
        self.next(32) as i32
    }

    /// Pulls the next bit from the JRandom state and returns it as a boolean.
    pub fn next_boolean(&mut self) -> bool {
        self.next(1) != 0
    }

    /// Pulls 24 bits and casts them into a single precision float (`f32`).
    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1u32 << 24) as f32
    }

    /// Pulls 53 total bits, adds them together, shifts them around and returns as
    /// a double-precision double (or `f64`).
    pub fn next_double(&mut self) -> f64 {
        let high = (self.next(26) as u64) << 27;
        let low = self.next(27) as u64;

        (high + low) as f64 / (1u64 << 53) as f64
    }

    /// A variation of `.next_int()` that has an min and max. In Java itself
    /// this was an override function, in Rust we made it an entirely separate func.
    /// Note: **inclusive of min, exclusive of max.**
    pub fn next_int_bounded(min: i32, max: i32) -> i32 {
        todo!("This function is not yet implemented!");
    }

    pub fn next_long() -> i64 {
        todo!("This function is not yet implemented!");
    }

    pub fn set_seed() { 
        todo!("Setting seeds is not yet implemented!");
    }
}

#[cfg(test)]
mod tests {
    use crate::rng::JRandom;

    #[test]
    fn test_set_seed_256090() {
        let mut rng = JRandom::new(256090);
        let expected = vec![
            1653768002,-522036376,-1124392101,-1769604513,-1871514677,-1561388831,
            1016856482,-137645518,1654948460,608901017,-14985256,-1292069575,
            1825822843,605913081,966205216,-191636548,-1786083256,2038877817,
            -1852769855,1368439797,-1413925247,1953365809,769704746,-422506046,
            -454247976,-1212391761,-1153765323,-1857925739,-1581051678,1514476555,
            390353327,1337843374,
        ];

        for (i, v) in expected.into_iter().enumerate() {
            let r = rng.next_int();
            assert_eq!(v, r, "PRNG mismatch at idx {i}; {r} != {v}");
        }
    }

    #[test]
    fn test_set_seed_129176() {
       let mut rng = JRandom::new(129176);
        let expected = vec![
            117634210,1705911941,-1743612870,711095580,1258999986,2106289178,
            1691012432,1331227599,-389101505,1403541903,609833655,134233092,
            459869982,227186820,-517137324,-805917320,-1701720705,-910010093,
            1059201033,-928743184,-2139546182,1080653381,1062473491,-1068602411,
            740074062,-905187896,2123695539,-1885619962,-1336805339,1483972318,
            -760409840,1176314601,
        ];

        for (i, v) in expected.into_iter().enumerate() {
            let r = rng.next_int();
            assert_eq!(v, r, "PRNG mismatch at idx {i}; {r} != {v}");
        }
    }

    #[test]
    fn test_set_seed_178962() {
        let mut rng = JRandom::new(178962);
        let expected = vec![
            1427596945,-446779591,-314844063,458871071,-774804449,2050799733,
            -1779413004,-1104186943,-1228705655,1614608796,-268057937,-468902591,
            2027046856,-1225585337,-2129372836,1854850766,1996907127,-1494678361,
            222678479,-167716532,-343355977,-378588268,-1768338578,856929165,
            -963261102,1234446821,-1041625101,600716226,2046587938,546467330,
            -1078406032,1700333103,
        ];

        for (i, v) in expected.into_iter().enumerate() {
            let r = rng.next_int();
            assert_eq!(v, r, "PRNG mismatch at idx {i}; {r} != {v}");
        }
    }

    #[test]
    fn test_set_seed_184400() {
        let mut rng = JRandom::new(184400);
        let expected = vec![
            1855837795,-1418359584,-653362269,461198643,1325803378,-1353433375,
            -2147117373,1899474640,1420275180,-488889993,-1845174571,2100688612,
            -1915621691,1996056815,2121591781,-2050999155,-45006086,-1151796577,
            743546851,-1622586946,1008084950,997372366,-1167279486,1742107478,
            -1278554255,-1044480013,1609254754,244045421,907195769,-98640862,
            -1968616848,1965987568,
        ];

        for (i, v) in expected.into_iter().enumerate() {
            let r = rng.next_int();
            assert_eq!(v, r, "PRNG mismatch at idx {i}; {r} != {v}");
        }
    }
}
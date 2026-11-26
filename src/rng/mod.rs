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
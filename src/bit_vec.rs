/// A simple bit vector library
/// By: Brian A. Madden - brian.a.madden@gmail.com

/// Bit vector object. Allows for setting, unsetting, flipping of
/// bits.  Indexing beyond the bounds of the vector will raise an
/// error.

pub struct BitVec {
    // Just the bits, nothing but the bits...
    bits: Vec<u8>,
    // Size of the bit array for bounds checking
    pub size: usize,
}

impl BitVec {

    /// Create a new `bitvec` with `size` bits
    pub fn new(size: usize) -> BitVec {
        BitVec {
            bits: vec![0u8; size],
            size: size,
        }
    }

    pub fn is_set(&self, pos: usize) -> bool {
        if pos > self.size {
            panic!("Attempted to index beyond bounds of bit vector.");
        }
        if (1 << (pos % 8)) & self.bits[pos / 8] > 0 {
            true
        } else {
            false
        }
    }

    /// Set the bit at `pos` to 1
    pub fn set(&mut self, pos: usize) {
        if pos > self.size {
            panic!("Attempted to index beyond bounds of bit vector.");
        }
        self.bits[pos / 8] |= 1 << (pos % 8);
    }

    /// Set the bit at `pos` to 0
    #[allow(dead_code)]
    pub fn unset(&mut self, pos: usize) {
        if pos > self.size {
            panic!("Attempted to index beyond bounds of bit vector.");
        }
        self.bits[pos / 8] &= {
            0xFF ^ (1 << (pos % 8))
        }
    }

    /// Flip the bit at `pos`. If the bit is 0 it becomes 1; if
    /// the bit is 1 it becomes 0.
    #[allow(dead_code)]
    pub fn flip(&mut self, pos: usize) {
        if pos > self.size {
            panic!("Attempted to index beyond bounds of bit vector.");
        }
        self.bits[pos / 8] ^= 1 << (pos % 8);
    }

    /// Return the raw bytes of the bit vector
    #[allow(dead_code)]
    pub fn get_bytes<'a>(&'a self) -> &'a [u8] {
        &self.bits
    }

}

#[test]
fn bit_vec_create_test() {
    let tester: BitVec = BitVec::new(8);
    assert!(tester.bits[0] == 0);
}

#[test]
fn bit_vec_set_test() {
    let mut tester: BitVec = BitVec::new(8);
    tester.set(5);
    assert!(tester.bits[0] == 32);
    let res = tester.is_set(5);
    assert!(res == true);
}

#[test]
fn bit_vec_is_set_test() {
    let mut tester: BitVec = BitVec::new(8);
    tester.set(5);
    assert!(tester.is_set(5) == true);
    assert!(tester.is_set(6) == false);
}

#[test]
fn bit_vec_unset_test() {
    let mut tester: BitVec = BitVec::new(8);
    tester.set(5);
    assert!(tester.is_set(5) == true);
    tester.unset(5);
    assert!(tester.is_set(5) == false);
    assert!(tester.bits[0] == 0);
}

#[test]
fn bit_vec_flip_test() {
    let mut tester: BitVec = BitVec::new(8);
    tester.flip(5);
    assert!(tester.is_set(5) == true);
    tester.flip(5);
    assert!(tester.is_set(5) == false);
}

#[test]
#[should_panic]
fn bit_vec_out_of_bounds_test() {
    let mut tester: BitVec = BitVec::new(8);
    tester.set(15);
}

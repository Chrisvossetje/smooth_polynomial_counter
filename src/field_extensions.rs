use std::ops::{Add, Mul, AddAssign, MulAssign};



pub trait FieldTraits {
  fn zero() -> Self;
  fn mul_ntimes(self, n: u8) -> Self;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct F2_i<const N: u8> {
  element: u16,
}

impl<const N: u8> FieldTraits for F2_i<N> {
    fn zero() -> Self {
      F2_i { element: 0}
    }

    fn mul_ntimes(self, n: u8) -> Self {
      let mut res = F2_i { element: 1};
      for _ in 0..n {
        res *= self;
      }
      res
    }
}

impl<const N: u8> F2_i<N> {
  pub fn new(element: u16) -> F2_i<N> {
    F2_i {element}
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for n in (1..N).rev() {
      if (self.element >> n) & 1 == 1{
        print!("x^{} + ", n);
      }
    }
    if self.element & 1 == 1 {
      println!("1");
    } else {
      println!("0");
    }
  }

  
  fn internal_mul(lhs: u64, rhs: u64) -> u16 {
    const IRRED_PART: [u64; 11] = [0, 1, 0b11,0b11,0b11, 0b101, 0b11, 0b11, 0b11001, 0b11, 0b1001];
    let bitmask: u64 = !((!0) << N);
    let value = IRRED_PART[N as usize];

    // step 1: take clmul of the two numbers
    // step 2: take the N most least significant bits of the result
    // step 3: take remaining bits and clmul by the irreducible polynomial
    // step 4: xor the two results together
    // step 5: this might overflow we need to repeat the process
    
    let mut res = F2_i::<N>::clmul(lhs, rhs);
    while (res >> N) > 0 {
      let lsb = res & bitmask;
      let msb = res >> N;
      res = lsb ^ F2_i::<N>::clmul(msb, value);
    }
    res as u16 
  }

  fn clmul(lhs: u64, rhs: u64) -> u64 {
    let mut res = 0;
    for n in 0..N {
      if (lhs >> n) & 1 == 1 {
        res ^= rhs << n;
      }
    }
    res
  }

  pub fn is_zero(&self) -> bool {
    if self.element == 0 {
      true
    } else {
      false
    }
  }
}

impl<const N: u8> Add for F2_i<N> {
  type Output = Self;
  
  fn add(self, rhs: Self) -> Self::Output {
    F2_i {element: self.element ^ rhs.element}
  }
}

impl<const N: u8> Mul for F2_i<N> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    F2_i {element: F2_i::<N>::internal_mul(self.element as u64, rhs.element as u64)}
  }
}

impl<const N: u8> AddAssign for F2_i<N> {
  fn add_assign(&mut self, rhs: Self) {
    self.element = self.element ^ rhs.element;
  }
}

impl<const N: u8> MulAssign for F2_i<N> {
  fn mul_assign(&mut self, rhs: Self) {
    self.element = F2_i::<N>::internal_mul(self.element as u64, rhs.element as u64);
  }
}


// A polynomial over the field over 3 elements, represented in bits
// Each coefficient is represented by 2 bits, so the can be at most 16
#[allow(non_camel_case_types)]
pub struct F3_i {
  pub element: u16,
  pub degree: u32,
}

impl F3_i {
  #[allow(dead_code)]
  pub fn new(element: u16, degree: u32) -> F3_i {
    F3_i { element: element, degree: degree }
  }
  
  // print each coefficient as a number
  #[allow(dead_code)]
  pub fn print(&self) {
    for j in 0..self.degree {
      let i = self.degree - j - 1;
      let num = (self.element >> (2 * i)) & 0b11;
      if num >= 3 {
        panic!("Invalid number: {}, deg: {}", num, i);
      }
      print!("{}", (self.element >> (2 * i)) & 0b11);
    }
    println!(" ({})", self.degree);
  }

  fn internal_add(a: u64, b: u64) -> u64 {
    const M1: u64 = 0x5555;
    const M2: u64 = 0xAAAA;

    let xor = a^b;
    let and = a&b;

    let one = (and & M1) << 1;
    let two = (and & M2) >> 1;

    let ab = ((a&M2) >> 1) & b;
    let ba = ((b&M2) >> 1) & a;

    let mul = (ab | ba) * 0b11;

    (mul ^ xor) | one | two
  }

  fn clmul(lhs: u64, rhs: u64, N: u8) -> u64 {
    let mut result = 0;
    for i in 0..N {
      let factor = (lhs >> (2*i)) & 0b11;
      match factor {
          2 => {result = F3_i::internal_add(result, rhs << 2*i); result = F3_i::internal_add(result, rhs << 2*i);}
          1 => {result = F3_i::internal_add(result, rhs << 2*i);}
          _ => {}
      }
    }
    result
  }

  // We first convert the polynomials to vectors with coefficients in Z/3Z
  // Then we multiply them and reduce the result
  fn internal_mul(a: u64, b: u64, N: u8) -> u16 {
    const IRRED_POLY: [u64; 7] = [0b0000, 0b0001, 0b0010, 0b0110,0b1001, 0b0110, 0b1001];

    let bitmask: u64 = !((!0) << 2*N);
    let irred = IRRED_POLY[N as usize];

    let mut result = F3_i::clmul(a, b, N); 
    while (result >> N*2) > 0 {
      let lsb = result & bitmask;
      let msb = result >> 2*N;
      result = F3_i::internal_add(lsb, F3_i::clmul(msb, irred, N));
    }

    result as u16
  }

}

impl Add for F3_i {
  type Output = Self;
  
  fn add(self, rhs: Self) -> Self::Output {
    F3_i {element: F3_i::internal_add(self.element as u64, rhs.element as u64) as u16, degree: self.degree}
  }
}

impl Mul for F3_i {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    F3_i {element: F3_i::internal_mul(self.element as u64, rhs.element as u64, self.degree as u8), degree: self.degree}
  }
}

impl AddAssign for F3_i {
  fn add_assign(&mut self, rhs: Self) {
    self.element = F3_i::internal_add(self.element as u64, rhs.element as u64) as u16;
  }
}

impl MulAssign for F3_i {
  fn mul_assign(&mut self, rhs: Self) {
    self.element = F3_i::internal_mul(self.element as u64, rhs.element as u64, self.degree as u8);
  }
}

use std::{ops::{Add, Mul, AddAssign, MulAssign}, num::Wrapping};



pub trait FieldTraits: Sized + MulAssign + Copy {
  
  const ZERO: Self;
  const ONE: Self;
  const MAX: Self;
  // fn zero() -> Self;
  // fn max() -> Self;
  fn mul_ntimes(self, n: u8) -> Self {
    let mut res = Self::ONE;
    for _ in 0..n {
      res *= self;
    }
    res
  }

  fn next(self) -> Option<Self>;

  fn iterate_over_points() -> ProjectivePointIterator<Self> {
    ProjectivePointIterator::new()
  }
}


enum ProjectivePointPhase {
  START,
  Z_NULL,
  Z_ONE,
  FINISHED,
}

pub struct ProjectivePointIterator<T: FieldTraits> {
  phase: ProjectivePointPhase,
  x: T,
  y: T,
}

impl<T: FieldTraits> ProjectivePointIterator<T> { 
  pub fn new() -> ProjectivePointIterator<T>{
    ProjectivePointIterator {
        phase: ProjectivePointPhase::START,
        x: T::ZERO,
        y: T::ZERO,
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct F2_i<const N: u8> {
  element: u16,
}
impl<const N: u8> FieldTraits for F2_i<N> {
  const ZERO: Self = F2_i { element: 0};
  const ONE: Self = F2_i { element: 1};
  const MAX: Self =  F2_i {element: ((1<<N) - 1)};
  

  fn next(self) -> Option<F2_i<N>> {
    if self == F2_i::MAX {
      None
    } else {
      Some(F2_i { element: {self.element + 1}})
    }
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
    const IRRED_PART: [u64; 11] = [0, 1, 0b11,0b11,0b11, 0b101, 0b11, 0b11, 0b11011, 0b11, 0b1001];
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
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct F3_i<const N: u8> {
  pub element: u16,
}

impl<const N: u8> FieldTraits for F3_i<N> {
    const ZERO: F3_i<N> = F3_i {element: 0b0 };
    const ONE: F3_i<N> = F3_i {element: 0b1 };
    const MAX: F3_i<N> = F3_i {element: 0xAA & !((!0)<< (2*N))} ; // DON'T 

    fn next(self) -> Option<Self> {
      if self == Self::MAX {
        None 
      } else {
        let t = ((self.element ^ 0xaa) | 0x55) >> 1;
        let el = (Wrapping(self.element) - Wrapping(t)).0 & t;
        Some(F3_i {element: el as u16})
      }
    }
}

impl<const N: u8> F3_i<N> {
  #[allow(dead_code)]
  pub fn new(element: u16) -> F3_i<N> {
    F3_i { element: element }
  }
  
  // print each coefficient as a number
  #[allow(dead_code)]
  pub fn print(&self) {
    for j in 0..N {
      let i = N - j - 1;
      let num = (self.element >> (2 * i)) & 0b11;
      if num >= 3 {
        panic!("Invalid number: {}, deg: {}", num, i);
      }
      print!("{}", (self.element >> (2 * i)) & 0b11);
    }
    println!(" ({})", N);
  }

  pub fn internal_add(a: u64, b: u64) -> u64 {
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

  fn internal_add_fast(a: u64,b: u64) -> u64 {
    const M2: u64 = 0xAAAA; 
    let na=!a;
    let nb=!b;
    let a4= ((M2 & na) >> 1) & na;
    let b4= ((M2 & nb) >> 1) & nb;
    !((  (a4 << 1 | a4) | (b4 << 1 | b4))^(a|b))
  }

  fn clmul(lhs: u64, rhs: u64) -> u64 {
    let mut result = 0;
    for i in 0..N {
      let factor = (lhs >> (2*i)) & 0b11;
      match factor {
          2 => {result = F3_i::<N>::internal_add(result, rhs << 2*i); result = F3_i::<N>::internal_add(result, rhs << 2*i);}
          1 => {result = F3_i::<N>::internal_add(result, rhs << 2*i);}
          _ => {}
      }
    }
    result
  }

  // We first convert the polynomials to vectors with coefficients in Z/3Z
  // Then we multiply them and reduce the result
  fn internal_mul(a: u64, b: u64) -> u16 {
    const IRRED_POLY: [u64; 7] = [0b0000, 0b0001, 0b0010, 0b0110,0b1001, 0b0110, 0b1001];

    let bitmask: u64 = !((!0) << 2*N);
    let irred = IRRED_POLY[N as usize];

    let mut result = F3_i::<N>::clmul(a, b); 
    while (result >> N*2) > 0 {
      let lsb = result & bitmask;
      let msb = result >> 2*N;
      result = F3_i::<N>::internal_add(lsb, F3_i::<N>::clmul(msb, irred));
    }

    result as u16
  }

}

impl<const N: u8> Add for F3_i<N> {
  type Output = Self;
  
  fn add(self, rhs: Self) -> Self::Output {
    F3_i {element: F3_i::<N>::internal_add_fast(self.element as u64, rhs.element as u64) as u16}
  }
}

impl<const N: u8> Mul for F3_i<N> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    F3_i {element: F3_i::<N>::internal_mul(self.element as u64, rhs.element as u64)}
  }
}

impl<const N: u8> AddAssign for F3_i<N> {
  fn add_assign(&mut self, rhs: Self) {
    self.element = F3_i::<N>::internal_add_fast(self.element as u64, rhs.element as u64) as u16;
  }
}

impl<const N: u8> MulAssign for F3_i<N> {
  fn mul_assign(&mut self, rhs: Self) {
    self.element = F3_i::<N>::internal_mul(self.element as u64, rhs.element as u64);
  }
}



impl<T: FieldTraits + Copy> Iterator for ProjectivePointIterator<T> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.phase {
            ProjectivePointPhase::START => {self.phase = ProjectivePointPhase::Z_NULL;   Some((T::ONE, T::ZERO, T::ZERO))},
            ProjectivePointPhase::Z_NULL => {
                let number = (self.x, T::ONE, T::ZERO);
                let result = self.x.next();
                if let Some(res) = result {
                  self.x = res;
                } else {
                  self.phase = ProjectivePointPhase::Z_ONE;
                  self.x = T::ZERO;
                }
                Some(number)
              } ,
            ProjectivePointPhase::Z_ONE => {
              let number = (self.x, self.y, T::ONE);
              let x = self.x.next();
              if let Some(x_res) = x {
                self.x = x_res;
              } else {
                self.x = T::ZERO;
                let y = self.y.next();
                if let Some(y_res) = y {
                  self.y = y_res;
                } else {
                  self.phase = ProjectivePointPhase::FINISHED;
                }
              }
              Some(number)
            },
            ProjectivePointPhase::FINISHED => None,
        }
    }
}
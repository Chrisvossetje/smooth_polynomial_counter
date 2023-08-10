use std::ops::{AddAssign, MulAssign, Add, Mul};
use std::vec;

use crate::DPLUS2_CHOOSE_2;
use crate::DEGREE;
use crate::polynomials::{Term, Polynomial};


#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Matrix {
  pub data: [[u8;3];3]
}

impl Matrix {
  pub fn determinant(&self) -> u8 {
    self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1]) 
    - self.data[0][1] * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0])
    + self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
  }

  pub fn new(data: [[u8;3];3]) -> Matrix 
  {
    Matrix { data: data }
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    println!("Matrix, det: {}", self.determinant());
    for i in 0..3 {
      for j in 0..3 {
        print!("{} ", self.data[i][j]);
      }
      println!("");
    }
  }
}


pub struct Lookup <const N: u8> {
  pub normal: Vec<Vec<F2_i<N>>>,
  pub part_x: Vec<Vec<F2_i<N>>>,
  pub part_y: Vec<Vec<F2_i<N>>>,
  pub part_z: Vec<Vec<F2_i<N>>>,
} 

impl<const N: u8> Lookup<N> {
  pub fn create(normal: &Vec<Term>, part_x: &Vec<Term>, part_y: &Vec<Term>, part_z: &Vec<Term>) -> Lookup<N> {
    let n_res = Term::generate_points_for_multiple(&normal);
    let x_res = Term::generate_points_for_multiple(&part_x);
    let y_res = Term::generate_points_for_multiple(&part_y);
    let z_res = Term::generate_points_for_multiple(&part_z);
    
    Lookup { normal: n_res, part_x: x_res, part_y: y_res, part_z: z_res }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IsoPolynomial {
  pub representative: Polynomial,
  pub size: u32,
}

impl IsoPolynomial {
  pub fn deconstruct(self) -> (Polynomial, u32) {
    (self.representative, self.size)
  }
}


pub fn generate_iso_polynomials(transform_lut: &Vec<Vec<u32>>, lut: &Vec<Term>, pgl3_f2: &Vec<Matrix>) -> Vec<IsoPolynomial>{
  let mut things = vec![true; 1<<(DPLUS2_CHOOSE_2)];

  things[0] = false;

  let mut iso_polys = Vec::new();

  for i in 1..1<<(DPLUS2_CHOOSE_2) {
    if things[i] {
      things[i] = false;
      let poly = Polynomial::new(i as u32);
      let mut count = 1;
      for i in 0..transform_lut.len() { // loop over matrices
        let perm_poly = poly.transform_by_matrix(&transform_lut[i]);
        if perm_poly.bits == 0  {
          poly.print(lut);
          pgl3_f2[i].print();
        }
        if things[perm_poly.bits as usize] {
          count += 1;
          things[perm_poly.bits as usize] = false;
        }
      }
      iso_polys.push(IsoPolynomial { representative: poly, size: count});
    }
  }
  iso_polys
}

pub fn s3_lut(lut: &Vec<Term>) -> Vec<Vec<usize>> {
  let mut super_luts: Vec<Vec<usize>> = vec![Vec::new(); 6];
  for t in lut {
    let isos = t.generate_isomorphisms();
    for (super_index, a) in isos.iter().enumerate() {
      for (inside_index, b) in lut.iter().enumerate() {
        if *a == *b {
          super_luts[super_index].push(inside_index);
        }
      }
    }
  }
  super_luts
}

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

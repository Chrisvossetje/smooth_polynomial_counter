use std::ops::{AddAssign, MulAssign, Add, Mul};
use std::vec;

use crate::DPLUS2_CHOOSE_2;
use crate::DEGREE;


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

pub fn generate_single_number<const N: u8>(x: u64,y: u64,z: u64) -> u32 {
  ((x << (N+1))+ (y << 1) + z) as u32
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
  assert!(!v.is_empty());
  let len = v[0].len();
  let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
  (0..len)
      .map(|_| {
          iters
              .iter_mut()
              .map(|n| n.next().unwrap())
              .collect::<Vec<T>>()
      })
      .collect()
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


pub fn generate_iso_polynomials(lut: &Vec<Term>) -> Vec<IsoPolynomial>{
  let mut things = vec![true; 1<<(DPLUS2_CHOOSE_2)];

  let iso_luts = s3_lut(&lut);

  let mut iso_polys = Vec::new();

  for i in 1..1<<(DPLUS2_CHOOSE_2) {
    if things[i] {
      let mut count = 1;
      let poly = Polynomial::new(i as u32);
      let mut poly_elements = vec![poly];
      for iso_lut in &iso_luts {
        let perm_poly = poly.give_permutation(&iso_lut);
        if !poly_elements.contains(&perm_poly) {
          count += 1;
          poly_elements.push(perm_poly);
        }
        things[perm_poly.bits as usize] = false;
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polynomial {
  bits: u32
}

impl Polynomial {
  pub fn new(bits: u32) -> Polynomial {
    Polynomial { bits: bits }
  }

  #[allow(dead_code)]
  pub fn print(self, lut: &Vec<Term>) {
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        if lut[i].constant != 0 { 
          print!("{} + ", lut[i].str())
        }
      }
    }
    println!()
  }

  pub fn evaluate<const N: u8>(self, index: u32, lut: &Vec<Vec<F2_i<N>>>) -> F2_i<N> {
    let mut res = F2_i::zero();
    let index_lut = &lut[index as usize];
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        res += index_lut[i];
      }
    }
    res
  }

  pub fn has_singularity<const N: u8>(self, lookup: &Lookup<N>) -> bool {
    for x in 0..(1<<N) {
      for y in 0..(1<<N) {
        for z in 0..2 {
          if x | y | z == 0 {
            continue;
          }
          let index = generate_single_number::<N>(x, y, z);
          if self.evaluate(index, &lookup.normal).is_zero() {
            if self.evaluate(index, &lookup.part_x).is_zero() {
              if self.evaluate(index, &lookup.part_y).is_zero() {
                if self.evaluate(index, &lookup.part_z).is_zero() {
                  return true
                }
              }
            }
          }
        }
      }
    }
    false
  }

  pub fn generate_default_lut() -> Vec<Term> {
    (0..=DEGREE as u8)
        .flat_map(move |a| {
            (0..=(DEGREE as u8) - a).map(move |b| {
                let c = DEGREE as u8 - b - a;
                Term {
                    x_deg: a,
                    y_deg: b,
                    z_deg: c,
                    constant: 1,
                }
            })
        })
        .collect()
  }

  pub fn give_permutation(self, lut: &Vec<usize>) -> Polynomial {
    let mut result: u32 = 0;
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        result |= 1 << lut[i]; 
      }
    }
    Polynomial { bits: result }
  }

  pub fn generate_derative_luts(default_lut: &Vec<Term>) -> (Vec<Term>, Vec<Term>, Vec<Term>) {
    let mut lut_x: Vec<Term> = vec![];
    let mut lut_y: Vec<Term> = vec![];
    let mut lut_z: Vec<Term> = vec![];

    for term in default_lut {
      let (x,y,z) = term.generate_derivatives();
      lut_x.push(x);
      lut_y.push(y);
      lut_z.push(z);
    }

    (lut_x, lut_y, lut_z)
  }
}






#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Term { 
  pub x_deg: u8,
  pub y_deg: u8,
  pub z_deg: u8,
  pub constant: u8,
}


impl Term {
  pub fn zero() -> Term {
    Term { x_deg: 0, y_deg: 0, z_deg: 0, constant: 0 }
  }

  pub fn evaluate<const N: u8>(self, x: F2_i<N>, y: F2_i<N>, z: F2_i<N>) -> F2_i<N> {
    if self.constant == 0 {
      F2_i::zero()
    } else {
      x.mul_ntimes(self.x_deg) * y.mul_ntimes(self.y_deg) * z.mul_ntimes(self.z_deg)
    }
  }

  pub fn str(self) -> String {
    format!("X^{} Y^{} Z^{}", self.x_deg, self.y_deg, self.z_deg)
  }

  pub fn generate_isomorphisms(&self) -> [Term; 6] {
    let id = *self;
    let s_13 = Term{ x_deg: self.z_deg, y_deg: self.y_deg, z_deg: self.x_deg, constant: self.constant };
    let s_23 = Term{ x_deg: self.x_deg, y_deg: self.z_deg, z_deg: self.y_deg, constant: self.constant };
    let s_12 = Term{ x_deg: self.y_deg, y_deg: self.x_deg, z_deg: self.z_deg, constant: self.constant };
    let s_123 = Term{ x_deg: self.y_deg, y_deg: self.z_deg, z_deg: self.x_deg, constant: self.constant };
    let s_132 = Term{ x_deg: self.z_deg, y_deg: self.x_deg, z_deg: self.y_deg, constant: self.constant };
    [id,s_13,s_23,s_12,s_123,s_132]
  }
  
  pub fn generate_derivatives(self) -> (Term, Term, Term) {
    let mut term_x = self;
    let mut term_y = self;
    let mut term_z = self;

    if term_x.x_deg == 0 {
      term_x = Term::zero();
    } else {
      term_x.constant = (term_x.constant * term_x.x_deg) & 1;
      term_x.x_deg -= 1;
    }

    if term_y.y_deg == 0 {
      term_y = Term::zero();
    } else {
      term_y.constant = (term_y.constant * term_y.y_deg) & 1;
      term_y.y_deg -= 1;
    }
    
    if term_z.z_deg == 0 {
      term_z = Term::zero();
    } else {
      term_z.constant = (term_z.constant * term_z.z_deg) & 1;
      term_z.z_deg -= 1;
    }
    (term_x, term_y, term_z)
  }

  pub fn generate_precalculated_points<const N: u8>(self) -> Vec<F2_i<N>> {
    let mut results = Vec::new();
    for x in 0..(1<<N) {
      for y in 0..(1<<N) {
        for z in 0..2 {
          let p_x = F2_i::new(x);
          let p_y = F2_i::new(y);
          let p_z = F2_i::new(z);
          let result = self.evaluate(p_x, p_y, p_z);
          results.push(result);
        }
      }
    }
    results
  }

  
  pub fn generate_points_for_multiple<const N: u8>(terms: &Vec<Term>) -> Vec<Vec<F2_i<N>>> {
    let mut resultant_terms = Vec::new();
    for t in terms {
      resultant_terms.push(t.generate_precalculated_points());
    }
    transpose(resultant_terms)
  }
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

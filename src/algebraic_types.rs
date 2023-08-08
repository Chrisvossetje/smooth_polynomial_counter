use std::ops::{AddAssign, MulAssign, Add, Mul};
use std::vec;

use crate::DPLUS2_CHOOSE_2;
use crate::DEGREE;

pub fn generate_single_number(x: u64,y: u64,z: u64, N: u32) -> u32 {
  ((x << (N+1))+ (y << 1) + z) as u32
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

pub fn generate_lut_degrees(order: u8, degree: u32) -> Vec<FieldExtension> {
  let mut results = Vec::new();
  for x in 0..(1<<degree) {
    let p_x = FieldExtension::new(x, degree);
    let result = p_x.mul_ntimes(order);
    results.push(result);
  }
  results
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
  fn zero(n: u32) -> Self;
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

  // TODO: d+ 2 choose 2
  pub fn evaluate(self, index: u32, lut: &Vec<Vec<FieldExtension>>, n:u32) -> FieldExtension {
    let mut res = FieldExtension::zero(n);
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        res += lut[i][index as usize];
      }
    }
    res
  }

  pub fn evaluate_alt2(self, x: FieldExtension, y: FieldExtension, z: FieldExtension, orders: &Vec<Vec<FieldExtension>>, terms: &Vec<Term>) -> FieldExtension {
    let mut res = FieldExtension::zero(x.degree);
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        res += terms[i].evaluate_alt(x, y, z, orders);
      }
    }
    res
  }


  pub fn evaluate_alt(self, x: FieldExtension, y: FieldExtension, z: FieldExtension, lut: &Vec<Term>) -> FieldExtension {
    let mut res = FieldExtension::zero(x.degree);
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        res += lut[i].evaluate(x, y, z);
      }
    }
    res
  }



  #[allow(dead_code)]
  pub fn has_singularity_alt(self, normal: &Vec<Term>, part_x:  &Vec<Term>,  part_y:  &Vec<Term>,  part_z:  &Vec<Term>, N: u32) -> Option<(FieldExtension, FieldExtension, FieldExtension)> {
    for x in 0..(1<<N) {
      for y in 0..(1<<N) {
        for z in 0..(1<<N) {
          if x | y | z == 0 {
            continue;
          }
          let p_x = FieldExtension::new(x, N);
          let p_y = FieldExtension::new(y, N);
          let p_z = FieldExtension::new(z, N);
          if self.evaluate_alt(p_x, p_y, p_z, normal).is_zero() {
            if self.evaluate_alt(p_x, p_y, p_z, part_x).is_zero() {
              if self.evaluate_alt(p_x, p_y, p_z, part_y).is_zero() {
                if self.evaluate_alt(p_x, p_y, p_z, part_z).is_zero() {
                  return Some((p_x,p_y,p_z));
                }
              }
            }
          }
        }
      }
    }
    None
  }


  pub fn has_singularity(self, normal: &Vec<Vec<FieldExtension>>, part_x:  &Vec<Vec<FieldExtension>>,  part_y:  &Vec<Vec<FieldExtension>>,  part_z:  &Vec<Vec<FieldExtension>>, N: u32) -> bool {
    for x in 0..(1<<N) {
      for y in 0..(1<<N) {
        for z in 0..2 {
          if x | y | z == 0 {
            continue;
          }
          let index = generate_single_number(x, y, z, N);
          if self.evaluate(index, normal, N).is_zero() {
            if self.evaluate(index, part_x, N).is_zero() {
              if self.evaluate(index, part_y, N).is_zero() {
                if self.evaluate(index, part_z, N).is_zero() {
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

  pub fn has_singularity_alt2(self, terms: &Vec<Vec<FieldExtension>>, normal: &Vec<Term>, part_x:  &Vec<Term>,  part_y:  &Vec<Term>,  part_z:  &Vec<Term>, N: u32) -> bool {
    for x in 0..(1<<N) {
      for y in 0..(1<<N) {
        for z in 0..2 {
          if x | y | z == 0 {
            continue;
          }
          let p_x = FieldExtension::new(x, N);
          let p_y = FieldExtension::new(y, N);
          let p_z = FieldExtension::new(z, N);
          if self.evaluate_alt2(p_x, p_y, p_z, terms, normal).is_zero() {
            if self.evaluate_alt2(p_x, p_y, p_z, terms, part_x).is_zero() {
              if self.evaluate_alt2(p_x, p_y, p_z, terms, part_y).is_zero() {
                if self.evaluate_alt2(p_x, p_y, p_z, terms, part_z).is_zero() {
                  return true;
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

  pub fn evaluate(self, x: FieldExtension, y: FieldExtension, z: FieldExtension) -> FieldExtension {
    if self.constant == 0 {
      FieldExtension::zero(x.degree)
    } else {
      x.mul_ntimes(self.x_deg) * y.mul_ntimes(self.y_deg) * z.mul_ntimes(self.z_deg)
    }
  }

  pub fn evaluate_alt(self, x: FieldExtension, y: FieldExtension, z: FieldExtension, lut: &Vec<Vec<FieldExtension>>) -> FieldExtension {
    if self.constant == 0 {
      FieldExtension::zero(x.degree)
    } else {
      lut[self.x_deg as usize][x.element as usize] * lut[self.y_deg as usize][y.element as usize] * lut[self.z_deg as usize][z.element as usize] 
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

  pub fn generate_precalculated_points(self, degree: u32) -> Vec<FieldExtension> {
    let mut results = Vec::new();
    for x in 0..(1<<degree) {
      for y in 0..(1<<degree) {
        for z in 0..2 {
          let p_x = FieldExtension::new(x, degree);
          let p_y = FieldExtension::new(y, degree);
          let p_z = FieldExtension::new(z, degree);
          let result = self.evaluate(p_x, p_y, p_z);
          results.push(result);
        }
      }
    }
    results
  }

  pub fn generate_points_for_multiple(terms: &Vec<Term>, N: u32) -> Vec<Vec<FieldExtension>> {
    let mut resultant_terms = Vec::new();
    for t in terms {
      resultant_terms.push(t.generate_precalculated_points(N));
    }
    resultant_terms
  }
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldExtension {
  element: u32,
  degree: u32,
}

impl FieldTraits for FieldExtension {
    fn zero(degree: u32) -> Self {
      FieldExtension { element: 0, degree}
    }

    fn mul_ntimes(self, n: u8) -> FieldExtension {
      let mut res = FieldExtension { element: 1 ,degree: self.degree };
      for _ in 0..n {
        res *= self;
      }
      res
    }
}

impl FieldExtension {
  pub fn new(element: u32, degree: u32) -> FieldExtension {
    FieldExtension {element, degree}
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for n in (1..self.degree).rev() {
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

  
  fn internal_mul(lhs: u64, rhs: u64, N: u32) -> u32 {
    const IRRED_PART: [u64; 11] = [0, 1, 0b11,0b11,0b11, 0b101, 0b11, 0b11, 0b11001, 0b11, 0b1001];
    let bitmask: u64 = !((!0) << N);
    let value = IRRED_PART[N as usize];

    // step 1: take clmul of the two numbers
    // step 2: take the N most least significant bits of the result
    // step 3: take remaining bits and clmul by the irreducible polynomial
    // step 4: xor the two results together
    // step 5: this might overflow we need to repeat the process
    
    let mut res = FieldExtension::clmul(lhs, rhs, N);
    while (res >> N) > 0 {
      let lsb = res & bitmask;
      let msb = res >> N;
      res = lsb ^ FieldExtension::clmul(msb, value, N);
    }
    res as u32 
  }

  fn clmul(lhs: u64, rhs: u64, N: u32) -> u64 {
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

impl Add for FieldExtension {
  type Output = Self;
  
  fn add(self, rhs: Self) -> Self::Output {
    FieldExtension {element: self.element ^ rhs.element, degree: self.degree}
  }
}

impl Mul for FieldExtension {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    FieldExtension {element: FieldExtension::internal_mul(self.element as u64, rhs.element as u64, self.degree), degree: self.degree}
  }
}

impl AddAssign for FieldExtension {
  fn add_assign(&mut self, rhs: Self) {
    self.element = self.element ^ rhs.element;
  }
}

impl MulAssign for FieldExtension {
  fn mul_assign(&mut self, rhs: Self) {
    self.element = FieldExtension::internal_mul(self.element as u64, rhs.element as u64, self.degree);
  }
}

use std::ops::{AddAssign, MulAssign, Add, Mul};

use crate::DPLUS2_CHOOSE_2;
use crate::DEGREE;

pub trait FieldTraits {
  fn zero() -> Self;
  fn mul_ntimes(self, n: u8) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polynomial {
  bits: u64
}

impl Polynomial {
  pub fn new(bits: u64) -> Polynomial {
    Polynomial { bits: bits }
  }

  // TODO: d+ 2 choose 2
  pub fn evaluate<const N: usize>(self, x: FieldExtension<N>, y: FieldExtension<N>,z: FieldExtension<N>, lut: &Vec<Term>) -> FieldExtension<N> {
    let mut res = FieldExtension::zero();
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        res += lut[i].evaluate(x, y, z);
      }
    }
    res
  }

  pub fn has_singularity<const N: usize>(self, normal: &Vec<Term>, part_x:  &Vec<Term>,  part_y:  &Vec<Term>,  part_z:  &Vec<Term>) -> bool {
    for x in 0..(1<<N) {
      for y in 0..(1<<N) {
        for z in 0..(1<<N) {
          if x | y | z == 0 {
            continue;
          }
          let p_x = FieldExtension::<N>::new(x);
          let p_y = FieldExtension::<N>::new(y);
          let p_z = FieldExtension::<N>::new(z);
          if self.evaluate(p_x, p_y, p_z, normal).is_zero() {
            if self.evaluate(p_x, p_y, p_z, part_x).is_zero() {
              if self.evaluate(p_x, p_y, p_z, part_y).is_zero() {
                if self.evaluate(p_x, p_y, p_z, part_z).is_zero() {
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
                println!("{},{},{}", a,b,c);
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

  pub fn evaluate<const N: usize>(self, x: FieldExtension<N>, y: FieldExtension<N>, z: FieldExtension<N>) -> FieldExtension<N> {
    if self.constant == 0 {
      FieldExtension::zero()
    } else {
      x.mul_ntimes(self.x_deg) * y.mul_ntimes(self.y_deg) * z.mul_ntimes(self.z_deg)
    }
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
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldExtension<const N: usize> {
  element: u64,
}

impl<const N: usize> FieldTraits for FieldExtension<N> {
    fn zero() -> Self {
      FieldExtension { element: 0}
    }

    fn mul_ntimes(self, n: u8) -> FieldExtension<N> {
      let mut res = FieldExtension { element: 1 };
      for _ in 0..n {
        res *= self;
      }
      res
    }
}

impl<const N: usize> FieldExtension<N> {
  pub fn new(element: u64) -> FieldExtension<N> {
    FieldExtension { element: element}
  }

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

  fn internal_mul(element: u64, rhs: u64) -> u64 {
    let bitmask: u64 = !((!0) << N);

    // let mult = self.element * rhs.element;
    let mut sum = 0;
    for n in 0..N {
      if (element >> n) & 1  == 1{
        sum ^= rhs << n;
      }
    }

    let mut sum_2 = 0;
    for n in 0..N {
      if (sum >> N + n) & 1  == 1 {
        sum_2 ^= 0b11 << n;
      }
    }

    if (sum_2 >> N) & 1 == 1 {
      sum_2 ^= 0b11;
    }
    (sum ^ sum_2) & bitmask
  }

  pub fn is_zero(&self) -> bool {
    if self.element == 0 {
      true
    } else {
      false
    }
  }
}

impl<const N: usize> Add for FieldExtension<N> {
  type Output = Self;
  
  fn add(self, rhs: Self) -> Self::Output {
    FieldExtension {element: self.element ^ rhs.element }
  }
}

impl<const N: usize> Mul for FieldExtension<N> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    FieldExtension {element: FieldExtension::<N>::internal_mul(self.element, rhs.element)}
  }
}

impl<const N: usize> AddAssign for FieldExtension<N> {
  fn add_assign(&mut self, rhs: Self) {
    self.element = self.element ^ rhs.element;
  }
}

impl<const N: usize> MulAssign for FieldExtension<N> {
  fn mul_assign(&mut self, rhs: Self) {
    self.element = FieldExtension::<N>::internal_mul(self.element, rhs.element);
  }
}

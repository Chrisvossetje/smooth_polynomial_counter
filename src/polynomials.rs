
use crate::{DPLUS2_CHOOSE_2, algebraic_types::{Lookup, Matrix}, DEGREE, field_extensions::{F2_i, FieldTraits, F3_i}, FIELD_ORDER};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polynomial {
  pub bits: u64
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Singularity{
  Singular,
  NonSingular,
}

impl Polynomial {
  pub fn new(bits: u64) -> Polynomial {
    Polynomial { bits: bits }
  }

  pub fn str(&self, lut: &Vec<Term>) -> String {
    let mut poly_str = String::new();
    let mut empty = true;
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
        if lut[i].constant != 0 { 
          if empty {
            poly_str = format!("{}", lut[i].str());
            empty = false;
          } else {
            poly_str = format!("{} + {}", poly_str, lut[i].str());
          }
        }
      }
    }
    poly_str
  }

  #[allow(dead_code)]
  pub fn print(&self, lut: &Vec<Term>) {
    println!("{}", self.str(lut));
  }

  pub fn evaluate_f2<const N: u8>(self, index: usize, lut: &Vec<Vec<F2_i<N>>>) -> F2_i<N> {
    let mut res = F2_i::ZERO;
    let index_lut = &lut[index];
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> 2*i) & 1 == 1 {
        res += index_lut[i];
      }
    }
    res
  }

  pub fn evaluate_f3<const N: u8>(self, index: usize, lut: &Vec<Vec<F3_i<N>>>) -> F3_i<N> {
    let mut res = F3_i::ZERO;
    let index_lut = &lut[index];
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> (2*i)) & 1 == 1 {
        res += index_lut[i];
      }
      if self.bits >> (2*i) & 2 == 2 {
        res += index_lut[i];
        res += index_lut[i];
      }
    }
    res
  }


  pub fn has_singularity_point<const N: u8>(self, index: usize,lookup: &Lookup<N>, count: &mut usize) -> Singularity {
    if self.evaluate_f3(index, &lookup.normal) == F3_i::ZERO {
      *count += 1;
      if self.evaluate_f3(index, &lookup.part_x) == F3_i::ZERO {
        if self.evaluate_f3(index, &lookup.part_y) == F3_i::ZERO {
          if self.evaluate_f3(index, &lookup.part_z) == F3_i::ZERO {
            return Singularity::Singular
          }
        }
      }
    }
    Singularity::NonSingular
  }

  pub fn has_singularity<const N: u8>(self, lookup: &Lookup<N>) -> Option<usize> {
    let mut points_on_curve = 0;

    for (index, _) in F2_i::<N>::iterate_over_points().enumerate() {
      if self.has_singularity_point(index,lookup, &mut points_on_curve) == Singularity::Singular {
        return None
      }
    }

    // // (1,0,0)
    
    // // (x,1,0) 
    // for x in 0..(1<<N) {
    //   let y = 1;
    //   let z = 0;
    //   if self.has_singularity_point(x,y,z,lookup, &mut points_on_curve) == Singularity::Singular {
    //     return None
    //   }
    // }
    
    // // (x,y,1)
    // for x in 0..(1<<N) {
    //   for y in 0..(1<<N) {
    //     let z = 1;
    //     if self.has_singularity_point(x,y,z,lookup, &mut points_on_curve) == Singularity::Singular {
    //       return None
    //     }
    //   }
    // }
    Some(points_on_curve)
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

  pub fn from_string(input: &str, lut: &Vec<Term>) -> Polynomial {

    let mut poly: u64 = 0;

    for str_term in input.split_whitespace() {
      let mut iter = str_term.split("_");
      let constant = iter.next().unwrap().parse::<u64>().unwrap();
      let powers = iter.next().unwrap().parse::<u64>().unwrap();
      let z = powers % 10;
      let y = (powers / 10) % 10;
      let x = (powers / 100) % 10;
      
      let t = Term {
        x_deg: x as u8,
        y_deg: y as u8,
        z_deg: z as u8,
        constant: 1,
        };

      for (index, term) in lut.iter().enumerate() {
        if t == *term {
          match FIELD_ORDER {
            2 =>  {
                    match constant {
                      1 => poly += 1 << index,
                      _ => panic!("Invalid constant in imported file")
                    };
                  },
            3 =>  {
                    match constant {
                      1 => poly += 0b01 << 2*index,
                      2 => poly += 0b10 << 2*index,
                      _ => panic!("Invalid constant in imported file")
                    };
                  },
            _ => {},
          }
        }
      }
    }

    Polynomial { bits: poly }
  }

  pub fn transform_by_matrix(self, transform_lut: &Vec<u64>) -> Polynomial {
    let mut bits = 0;
    for i in 0..DPLUS2_CHOOSE_2 {
      if ((self.bits >> i) & 1) == 1 {
        bits ^= transform_lut[i];
      }
    }
    Polynomial { bits: bits }
  }
}



#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
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

  pub fn evaluate_f2<const N: u8>(self, x: F2_i<N>, y: F2_i<N>, z: F2_i<N>) -> F2_i<N> {
    if self.constant == 0 {
      F2_i::ZERO
    } else {
      x.mul_ntimes(self.x_deg) * y.mul_ntimes(self.y_deg) * z.mul_ntimes(self.z_deg)
    }
  }

  pub fn evaluate_f3<const N: u8>(self, x: F3_i<N>, y: F3_i<N>, z: F3_i<N>) -> F3_i<N> {
    if self.constant == 0 {
      F3_i::ZERO
    } else {
      x.mul_ntimes(self.x_deg) * y.mul_ntimes(self.y_deg) * z.mul_ntimes(self.z_deg)
    }
  }

  pub fn str(self) -> String {
    format!("X^{} Y^{} Z^{}", self.x_deg, self.y_deg, self.z_deg)
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

  pub fn generate_precalculated_points<const N: u8>(self) -> Vec<F3_i<N>> {
    let mut results = Vec::new();
    for (x,y,z) in F3_i::iterate_over_points() {
      let result = self.evaluate_f3(x, y, z);
      results.push(result);
    }
    results
  }

  pub fn generate_points_for_multiple<const N: u8>(terms: &Vec<Term>) -> Vec<Vec<F3_i<N>>> {
    let mut resultant_terms = Vec::new();
    for t in terms {
      resultant_terms.push(t.generate_precalculated_points());
    }
    transpose(resultant_terms)
  }

  pub fn transform_by_matrix(self, matrix: &Matrix, lut: &Vec<Term>) -> u64 {
    if self.constant == 0 {
      return 0;
    }
    let p1 = exponentiate_linear_polynomial(matrix.data[0][0], matrix.data[0][1], matrix.data[0][2], self.x_deg);
    let p2 = exponentiate_linear_polynomial(matrix.data[1][0], matrix.data[1][1], matrix.data[1][2], self.y_deg);
    let p3 = exponentiate_linear_polynomial(matrix.data[2][0], matrix.data[2][1], matrix.data[2][2], self.z_deg);
    let terms = polynomial_product(p1, p2, p3);

    let mut result = 0;
    for t in terms {
      for i in 0..DPLUS2_CHOOSE_2 {
        if lut[i].is_similar(t) {
          let bit = (t.constant as u64) << (i*2);
          result = F3_i::<1>::internal_add(result, bit);
        }
      }
    }
    result
  }

  fn is_similar(&self, t: Term) -> bool {
    self.x_deg == t.x_deg && self.y_deg == t.y_deg && self.z_deg == t.z_deg
  }
}

// (ax+by+cz)^m = sum_{k1+k2+k3=m} (m choose k1,k2,k3) a^k1 b^k2 c^k3 x^k1 y^k2 z^k3
// SLOW!
pub fn exponentiate_linear_polynomial(a: u8, b: u8, c: u8, m: u8) -> Vec<Term> {
  let mut terms: Vec<Term> = Vec::new();
  for k1 in 0..=m {
    for k2 in 0..=(m-k1) {
      let k3 = m-k1-k2;
      let coeff = binomial_coefficient(m, k1, k2, k3) % FIELD_ORDER as u8;
      if (coeff == 0)|| (k1>0 && a==0) || (k2>0 && b==0) || (k3>0 && c==0) {
        continue;
      }
      let term = Term { x_deg: k1, y_deg: k2, z_deg: k3, constant: coeff };
      terms.push(term);
    }
  }
  terms
}

pub fn polynomial_product(a: Vec<Term>, b: Vec<Term>, c: Vec<Term>) -> Vec<Term> {
  let mut result: Vec<Term> = Vec::new();
  for t1 in &a {
    for t2 in &b {
      for t3 in &c {
        let term = Term { x_deg: t1.x_deg + t2.x_deg + t3.x_deg, 
                          y_deg: t1.y_deg + t2.y_deg + t3.y_deg, 
                          z_deg: t1.z_deg + t2.z_deg + t3.z_deg, 
                          constant: (t1.constant * t2.constant * t3.constant) % FIELD_ORDER as u8};
        result.push(term);
      }
    }
  }
  result
}

pub fn binomial_coefficient(m: u8, k1: u8, k2: u8, k3:u8) -> u8 {
  (factorial(m) / (factorial(k1) * factorial(k2) * factorial(k3))) as u8
}

pub fn factorial(n: u8) -> u64 {
  let mut result: u64 = 1;
  for i in 1..=n as u64 {
    result *= i;
  }
  result
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

pub fn generate_transform_lut(pgl3: &Vec<Matrix>, lut: &Vec<Term>) -> Vec<Vec<u64>> {
  let mut result = vec![];
  for m in pgl3 {
    let mut result_for_m = vec![];
    for t in lut{
      let transformed = t.transform_by_matrix(&m, &lut);
      result_for_m.push(transformed);
    }
    result.push(result_for_m);
  }
  result
}

use crate::{DPLUS2_CHOOSE_2, algebraic_types::Lookup, DEGREE, field_extensions::{F2_i, FieldTraits, F3_i}, FIELD_ORDER};


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
  #[allow(dead_code)]
  pub fn new(bits: u64) -> Polynomial {
    Polynomial { bits: bits }
  }

  pub fn str(&self, lut: &Vec<Term>) -> String {
    let mut poly_str = String::new();
    let mut empty = true;
    for i in 0..DPLUS2_CHOOSE_2 {
      if FIELD_ORDER == 2 {
        if (self.bits >> i) & 1 == 1 {
          if lut[i].constant != 0 { 
            if empty {
              poly_str = format!("1_{}", lut[i].str());
              empty = false;
            } else {
              poly_str = format!("{} 1_{}", poly_str, lut[i].str());
            }
          }
        }
      } else if FIELD_ORDER == 3 {        
        if (self.bits >> 2*i) & 0b01 == 1 {
          if lut[i].constant != 0 { 
            if empty {
              poly_str = format!("1_{}", lut[i].str());
              empty = false;
            } else {
              poly_str = format!("{} 1_{}", poly_str, lut[i].str());
            }
          }
        }
        if (self.bits >> 2*i) & 0b10 == 2 {
          if lut[i].constant != 0 { 
            if empty {
              poly_str = format!("2_{}", lut[i].str());
              empty = false;
            } else {
              poly_str = format!("{} 2_{}", poly_str, lut[i].str());
            }
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

  #[allow(dead_code)]
  pub fn evaluate_f2<const N: u8>(self, index: usize, lut: &Vec<Vec<F2_i<N>>>) -> F2_i<N> {
    let mut res = F2_i::ZERO;
    let index_lut = &lut[index];
    for i in 0..DPLUS2_CHOOSE_2 {
      if (self.bits >> i) & 1 == 1 {
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


  // FIELD_ORDER_PROBLEM
  pub fn has_singularity_point<const N: u8>(self, index: usize,lookup: &Lookup<N>, count: &mut usize) -> Singularity {
    if self.evaluate_f2(index, &lookup.normal) == F2_i::ZERO {
      *count += 1;
      if self.evaluate_f2(index, &lookup.part_x) == F2_i::ZERO {
        if self.evaluate_f2(index, &lookup.part_y) == F2_i::ZERO {
          if self.evaluate_f2(index, &lookup.part_z) == F2_i::ZERO {
            return Singularity::Singular
          }
        }
      }
    }
    Singularity::NonSingular
  }

  pub fn has_singularity<const N: u8>(self, lookup: &Lookup<N>) -> Option<usize> {
    let mut points_on_curve = 0;

  // FIELD_ORDER_PROBLEM
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
        if t.is_similar(*term) {
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

  #[allow(dead_code)]
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
    format!("{}{}{}", self.x_deg, self.y_deg, self.z_deg)
  }
  
  pub fn generate_derivatives(self) -> (Term, Term, Term) {
    let mut term_x = self;
    let mut term_y = self;
    let mut term_z = self;

    if term_x.x_deg == 0 {
      term_x = Term::zero();
    } else {
      term_x.constant = (term_x.constant * term_x.x_deg) % FIELD_ORDER as u8;
      term_x.x_deg -= 1;
    }

    if term_y.y_deg == 0 {
      term_y = Term::zero();
    } else {
      term_y.constant = (term_y.constant * term_y.y_deg) % FIELD_ORDER as u8;
      term_y.y_deg -= 1;
    }
    
    if term_z.z_deg == 0 {
      term_z = Term::zero();
    } else {
      term_z.constant = (term_z.constant * term_z.z_deg) % FIELD_ORDER as u8;
      term_z.z_deg -= 1;
    }
    (term_x, term_y, term_z)
  }

  // FIELD_ORDER_PROBLEM
  pub fn generate_precalculated_points<const N: u8>(self) -> Vec<F2_i<N>> {
    let mut results = Vec::new();
    for (x,y,z) in F2_i::iterate_over_points() {
      let result = self.evaluate_f2(x, y, z);
      results.push(result);
    }
    results
  }

  // FIELD_ORDER_PROBLEM
  pub fn generate_points_for_multiple<const N: u8>(terms: &Vec<Term>) -> Vec<Vec<F2_i<N>>> {
    let mut resultant_terms = Vec::new();
    for t in terms {
      resultant_terms.push(t.generate_precalculated_points());
    }
    transpose(resultant_terms)
  }

fn is_similar(&self, term: Term) -> bool {
  self.x_deg == term.x_deg && self.y_deg == term.y_deg && self.z_deg == term.z_deg
}
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
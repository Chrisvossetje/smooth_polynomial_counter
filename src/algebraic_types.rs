use crate::MAX_FIELD_EXT;
#[allow(unused)]
use crate::field_extensions::{F2_i, F3_i};
use crate::polynomials::{Term, Polynomial};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PolynomialResult {
  pub poly: IsoPolynomial,
  pub points_on_curve: [usize; MAX_FIELD_EXT],
}

impl PolynomialResult {
  pub fn new(iso_poly: IsoPolynomial, points_on_curve: [usize; MAX_FIELD_EXT]) -> PolynomialResult {
    PolynomialResult { poly: iso_poly, points_on_curve }
  }

  pub fn to_string(&self, normal: &Vec<Term>) -> String {
    format!("{} | {} | {:?}", self.poly.representative.str(normal), self.poly.size, self.points_on_curve)
  }
}

pub struct Lookup <const N: u8> {
  pub normal: Vec<Vec<F3_i<N>>>,
  pub part_x: Vec<Vec<F3_i<N>>>,
  pub part_y: Vec<Vec<F3_i<N>>>,
  pub part_z: Vec<Vec<F3_i<N>>>,
} 

impl<const N: u8> Lookup<N> {
  pub fn create(normal: &Vec<Term>, part_x: &Vec<Term>, part_y: &Vec<Term>, part_z: &Vec<Term>) -> Lookup<N> {
    let n_res = Term::generate_points_for_multiple(&normal);
    let x_res = Term::generate_points_for_multiple(&part_x);
    let y_res = Term::generate_points_for_multiple(&part_y);
    let z_res = Term::generate_points_for_multiple(&part_z);
    println!("Made lookup tables for degree {N}");

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
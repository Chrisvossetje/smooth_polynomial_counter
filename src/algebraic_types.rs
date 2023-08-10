use std::vec;

use crate::DPLUS2_CHOOSE_2;
use crate::MAX_FIELD_EXT;
use crate::field_extensions::F2_i;
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


#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Matrix {
  pub data: [[u8;3];3]
}

impl Matrix {
  pub fn determinant(&self) -> u8 {
    ((self.data[0][0] as i32 * ((self.data[1][1] * self.data[2][2]) as i32 - (self.data[1][2] * self.data[2][1]) as i32) ) as i32 -
    (self.data[0][1] as i32 * ((self.data[1][0] * self.data[2][2]) as i32 - (self.data[1][2] * self.data[2][0]) as i32)) as i32 +
    (self.data[0][2] as i32 * ((self.data[1][0] * self.data[2][1]) as i32 -( self.data[1][1] * self.data[2][0]) as i32)) as i32) as u8
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

  pub fn generate_pgl3_f2() -> Vec<Matrix> {
    let mut pgl3_f2: Vec<Matrix> = Vec::new();
    for i in 0..(1<<9) {
      let mut data: [[u8;3];3] = [[0;3];3];
      for j in 0..9 {
        data[j/3][j%3] = ((i >> j) & 1) as u8;
      }
      let matrix = Matrix::new(data);
      if matrix.determinant() % 2 == 1 {
        pgl3_f2.push(matrix);
      }
    }
    pgl3_f2
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


pub fn generate_iso_polynomials(transform_lut: &Vec<Vec<u32>>) -> Vec<IsoPolynomial>{
  let mut things = vec![true; 1<<(DPLUS2_CHOOSE_2)];

  things[0] = false;

  let mut iso_polys = Vec::new();

  for i in 1..1<<(DPLUS2_CHOOSE_2) {
    if things[i] {
      things[i] = false;
      let poly = Polynomial::new(i as u32);
      let mut count = 1;
      let mut smallest_poly = poly;
      for i in 0..transform_lut.len() { // loop over matrices
        let perm_poly = poly.transform_by_matrix(&transform_lut[i]);
        if things[perm_poly.bits as usize] {
          count += 1;
          things[perm_poly.bits as usize] = false;
          if perm_poly.bits.count_ones() < smallest_poly.bits.count_ones() {
            smallest_poly = perm_poly;
          }
        }
      }
      iso_polys.push(IsoPolynomial { representative: smallest_poly, size: count});
    }
  }
  iso_polys
}
use std::ops::{Add, Mul};

use crate::MAX_FIELD_EXT;


// A polynomial over the field over 3 elements, represented in bits
// Each coefficient is represented by 2 bits, so the can be at most 16
#[allow(non_camel_case_types)]
pub struct F3_i {
  pub bits: u32,
  pub degree: u32,
}

impl F3_i {
  pub fn new(bits: u32, degree: u32) -> F3_i {
    F3_i { bits: bits, degree: degree }
  }

  // print each coefficient as a number
  pub fn print(&self) {
    for j in 0..self.degree {
      let i = self.degree - j - 1;
      let num = (self.bits >> (2 * i)) & 0b11;
      if num >= 3 {
        panic!("Invalid number: {}, deg: {}", num, i);
      }
      print!("{}", (self.bits >> (2 * i)) & 0b11);
    }
    println!(" ({})", self.degree);
  }


  // add two polynomials, reducing the result
  fn internal_add(a: u32, b: u32, degree: u32) -> u32 {
    // loop through all coefficients
    let mut result = 0;
    for i in 0..degree {
        // get the coefficient of a and b
        let a_coeff = (a >> (2 * i)) & 0b11;
        let b_coeff = (b >> (2 * i)) & 0b11;
    
        // add them together
        let mut result_coeff = a_coeff + b_coeff;
    
        // if the result is bigger than 2, subtract 3
        if result_coeff > 2 {
            result_coeff -= 3;
        }
    
        // set the result coefficient
        result |= result_coeff << (2 * i);
    }
    result
  }

  // We first convert the polynomials to vectors with coefficients in Z/3Z
  // Then we multiply them and reduce the result
  fn internal_mul(a: u32, b: u32, degree: u32) -> u32 {

    const IRRED_POLY: [[u16;2]; 7] = [[0,0], [1,0], [2,0], [2,1], [2,2], [2,1], [2,2]];

    // convert a and b to vectors
    let mut a_vec: Vec<u16> = Vec::new();
    let mut b_vec: Vec<u16> = Vec::new();
    for i in 0..degree {
        a_vec.push(((a >> (2 * i)) & 0b11) as u16);
        b_vec.push(((b >> (2 * i)) & 0b11) as u16);
    }
    
    // multiply them
    let mut result_vec = vec![0; (2 * degree - 1) as usize];
    for i in 0..degree {
        for j in 0..degree {
            result_vec[(i + j) as usize] += a_vec[i as usize] * b_vec[j as usize];
        }
    }

    // while still has coefficients of index greater than degree
    // use the identity x^degree = irred_poly[degree]

    while (result_vec.len() as u32) > degree {
        let mut index = result_vec.len() - 1;
        let mut coeff = result_vec[index];
        while coeff == 0 {
            index -= 1;
            coeff = result_vec[index];
        }
        let mut irred_poly = IRRED_POLY[index - degree as usize].clone();
        for i in 0..irred_poly.len() {
            irred_poly[i] = (irred_poly[i] * coeff) % 3;
        }
        for i in 0..irred_poly.len() {
            result_vec[index - degree as usize + i] += irred_poly[i];
        }
        result_vec.truncate(index);
    }
    
    todo!();

  }

}

impl Add for F3_i {
  type Output = Self;
  
  fn add(self, rhs: Self) -> Self::Output {
    debug_assert_eq!(self.degree, rhs.degree);
    F3_i {bits: F3_i::internal_add(self.bits, rhs.bits, self.degree), degree: self.degree}
  }
}

impl Mul for F3_i {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    debug_assert_eq!(self.degree, rhs.degree);
    F3_i {bits: F3_i::internal_mul(self.bits, rhs.bits, self.degree), degree: self.degree}
  }
}
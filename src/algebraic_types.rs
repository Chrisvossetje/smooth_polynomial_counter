use std::ops::{AddAssign, MulAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Term { 
  pub x_deg: u8,
  pub y_deg: u8,
  pub z_deg: u8,
  pub constant: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartialDerivatives {
  pub term_x: Term,
  pub term_y: Term,
  pub term_z: Term,
}

impl Term {
  pub fn zero() -> Term {
    Term { x_deg: 0, y_deg: 0, z_deg: 0, constant: 0 }
  }
  pub fn generate_derivatives(self) -> PartialDerivatives {
    let mut term_x = self;
    let mut term_y = self;
    let mut term_z = self;

    if term_x.x_deg == 0 {
      term_x = Term::zero();
    } else {
      term_x.constant = term_x.x_deg;
      term_x.x_deg -= 1;
    }

    if term_y.y_deg == 0 {
      term_y = Term::zero();
    } else {
      term_y.constant = term_y.y_deg;
      term_y.y_deg -= 1;
    }
    
    if term_z.z_deg == 0 {
      term_z = Term::zero();
    } else {
      term_z.constant = term_z.z_deg;
      term_z.z_deg -= 1;
    }
    PartialDerivatives { term_x, term_y, term_z }
  }
}

impl PartialDerivatives {
  
}




pub struct FieldExtension<const N> {
  element: u64,
}

impl FieldExtension<const N: usize> {
  fn new(element: u64, n: u64) -> FieldExtension {
    FieldExtension { element: element, dimension: n}
  }
}

impl AddAssign for FieldExtension {
  fn add_assign(&mut self, rhs: Self) {
    self.element = self.element ^ rhs.element;
  }
}

impl MulAssign for FieldExtension {
  fn mul_assign(&mut self, rhs: Self) {
    let mult = self.element * rhs.element;

  }
}
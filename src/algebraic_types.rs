use std::ops::{AddAssign, MulAssign, Add, Mul};

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

  pub fn evaluate<T: Mul>(self, x: T, y: T, z: T) {
    let mut x_res = 1;
    let mut x_res = 1;
    let mut x_res = 1;
    
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



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldExtension<const N: usize> {
  element: u64,
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

use std::time::Instant;

use algebraic_types::Polynomial;

use crate::algebraic_types::{Term, FieldExtension, generate_iso_polynomials};

mod algebraic_types;

const DEGREE: usize = 5;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;

const MAX_FIELD_EXT: usize = 7;



fn main() {
  let now = Instant::now();

  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);

  // Each term now has for each degree for each term for each points calculated its fieldextension.
  let mut normal_results = Vec::new();
  let mut part_x_results = Vec::new();
  let mut part_y_results = Vec::new();
  let mut part_z_results = Vec::new();
  
  for n in 1..=MAX_FIELD_EXT as u32 {
    normal_results.push(Term::generate_points_for_multiple(&normal, n));
    part_x_results.push(Term::generate_points_for_multiple(&part_x, n));
    part_y_results.push(Term::generate_points_for_multiple(&part_y, n));
    part_z_results.push(Term::generate_points_for_multiple(&part_z, n));
  }
  println!("Generated lookup stuff");
  
  let iso_polys = generate_iso_polynomials(&normal);
  println!("Generated isomorphic polynomials");
  
  println!("Start counting!");
  
  let mut smooth: usize = 0;
  'outer: for iso_poly in iso_polys { 
    let (poly, size) = iso_poly.deconstruct();
    for n in 1..=MAX_FIELD_EXT as usize {
      if poly.has_singularity(&normal_results[n-1], &part_x_results[n-1], &part_y_results[n-1], &part_z_results[n-1], n as u32) {continue 'outer;}       
    }
    smooth += size as usize;
  }

  println!("{smooth}");

  println!("{:?}", now.elapsed());
}


// 1: 823543
// 2: 724136
// 3: 712880
// 4: 693056
// 5: 693056
// 6: 690648
// 7:
// 8:
// 9:
// 10:
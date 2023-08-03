use std::time::Instant;

use algebraic_types::Polynomial;

use crate::algebraic_types::{Term, FieldExtension, s6_lut, generate_iso_polynomials};

mod algebraic_types;

const DEGREE: usize = 4;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;




fn main() {
  let now = Instant::now();

  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);

  let iso_polys = generate_iso_polynomials(&normal);
  // println!("{:?}", iso_polys);
  


  let mut smooth: usize = 0;
  for iso_poly in iso_polys {
    let (poly, size) = iso_poly.deconstruct();
    if poly.has_singularity::<1>(&normal, &part_x, &part_y, &part_z) {continue;}        // 823543
    if poly.has_singularity::<2>(&normal, &part_x, &part_y, &part_z) {continue;}        // 724136
    if poly.has_singularity::<3>(&normal, &part_x, &part_y, &part_z) {continue;}        // 712880
    if poly.has_singularity::<4>(&normal, &part_x, &part_y, &part_z) {continue;}        // 693056
    if poly.has_singularity::<5>(&normal, &part_x, &part_y, &part_z) {continue;}
    if poly.has_singularity::<6>(&normal, &part_x, &part_y, &part_z) {continue;}
    // if poly.has_singularity::<7>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("7");
    // if poly.has_singularity::<8>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("8");
    // if poly.has_singularity::<9>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("9");
    // if poly.has_singularity::<10>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("10");

    // polynomial.print(&normal);
    smooth += size as usize;
  }

  println!("{smooth}");

  println!("{:?}", now.elapsed());
}

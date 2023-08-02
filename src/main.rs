use algebraic_types::Polynomial;

use crate::algebraic_types::{Term, FieldExtension};

mod algebraic_types;

const DEGREE: usize = 3;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;

fn main() {
  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);

  let mut smooth: usize = 0;
  for n in 0..(1<<(DPLUS2_CHOOSE_2)) {
    let polynomial: Polynomial = Polynomial::new(n);
    if polynomial.has_singularity::<1>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<2>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<3>(&normal, &part_x, &part_y, &part_z) {continue;}
    // if polynomial.has_singularity::<4>(&normal, &part_x, &part_y, &part_z) {continue;}
    // if polynomial.has_singularity::<5>(&normal, &part_x, &part_y, &part_z) {continue;}
    // if polynomial.has_singularity::<6>(&normal, &part_x, &part_y, &part_z) {continue;}
    // if polynomial.has_singularity::<7>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("7");
    // if polynomial.has_singularity::<8>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("8");
    // if polynomial.has_singularity::<9>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("9");
    // if polynomial.has_singularity::<10>(&normal, &part_x, &part_y, &part_z) {continue;}
    // println!("10");

    polynomial.print(&normal);
    smooth += 1;
    println!("{smooth}");
  }

  println!("{smooth}");

}

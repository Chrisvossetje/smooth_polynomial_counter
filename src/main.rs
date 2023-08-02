use algebraic_types::Polynomial;

use crate::algebraic_types::{Term, FieldExtension};

mod algebraic_types;

const DEGREE: usize = 3;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;

fn generate_terms() -> Vec<Term> {
    (0..=DEGREE as u8)
        .flat_map(move |a| {
            (0..=(DEGREE as u8) - a).map(move |b| {
                let c = DEGREE as u8 - b - a;
                println!("{},{},{}", a,b,c);
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

fn main() {
  let terms = generate_terms();


  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);

  let mut smooth: usize = 0;
  for n in 0..(1<<(DPLUS2_CHOOSE_2)) {
    let polynomial: Polynomial = Polynomial::new(n);
    if polynomial.has_singularity::<1>(&normal, &part_x, &part_y, &part_z) {polynomial.print(&normal); continue;}
    if polynomial.has_singularity::<2>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<3>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<4>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<5>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<6>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<7>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<8>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<9>(&normal, &part_x, &part_y, &part_z) {continue;}
    if polynomial.has_singularity::<10>(&normal, &part_x, &part_y, &part_z) {continue;}

    smooth += 1;
  }

  println!("{smooth}");

}

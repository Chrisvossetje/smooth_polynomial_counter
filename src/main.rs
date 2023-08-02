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
  // println!("{:?}",terms[3].generate_derivatives());
  for n in 0..(1<<(DPLUS2_CHOOSE_2)) {
    // let polynomial: Polynomial = Polynomial::new();

    // for N in 0..11 {
    //   if polynomial.has_singularity(normal, part_x, part_y, part_z) {
    //     break
    //   }
    // }
  }


  for n in 0..(1<<3) {
    FieldExtension::<3>::new(n).print();
  }
}

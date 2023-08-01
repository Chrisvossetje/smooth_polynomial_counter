use algebraic_types::PartialDerivatives;

use crate::algebraic_types::Term;

mod algebraic_types;

const DEGREE: usize = 2;

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
  println!("{:?}",terms[3].generate_derivatives());

    

}

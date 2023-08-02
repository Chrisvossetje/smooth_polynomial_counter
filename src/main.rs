use algebraic_types::Polynomial;

use crate::algebraic_types::{Term, FieldExtension};

mod algebraic_types;

const DEGREE: usize = 5;

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
  for term in terms {
    println!("{} {} {}", term.x_deg, term.y_deg, term.z_deg)
  }

  let f = Polynomial::new(0b1);
  

}

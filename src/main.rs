use std::{time::Instant, sync::{mpsc, Arc}, thread};

use algebraic_types::{Polynomial, IsoPolynomial, Lookup};

use crate::algebraic_types::generate_iso_polynomials;

#[allow(non_snake_case)]
mod algebraic_types;

const DEGREE: usize = 5;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;

const MAX_FIELD_EXT: usize = 8;

const NUM_THREADS: usize = 16;

// THIS COULD BE HIGHER ????
type SuperType = (Lookup<1>,Lookup<2>,Lookup<3>,Lookup<4>,Lookup<5>,Lookup<6>,Lookup<7>,Lookup<8>);


fn main() {
  let now = Instant::now();

  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);
 
  let super_lookup: SuperType = ( Lookup::<1>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<2>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<3>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<4>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<5>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<6>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<7>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<8>::create(&normal, &part_x, &part_y, &part_z),
                                );
  println!("Generated lookup stuff");

  
  let iso_polys = generate_iso_polynomials(&normal);
  println!("Generated isomorphic polynomials");
  
  println!("Generate threads and start counting!");

  let chunk_size = (iso_polys.len() + NUM_THREADS - 1) / (NUM_THREADS);
  
  let arc_super_lookup = Arc::new(super_lookup);
  let arc_iso_polys = Arc::new(iso_polys);

  let (tx, rx) = mpsc::channel();

  for i in 0..NUM_THREADS {
    // Clone the sender to move into each thread
    let a_tx = tx.clone();

    // Clone the recomputed results to move into each thread locally
    let local_super_lookup = arc_super_lookup.clone();

    let local_iso_polys = arc_iso_polys.clone();

    let start_index = i*chunk_size;

    // Spawn a new thread
    thread::spawn(move || {
      let result =  
        is_smooth(&local_iso_polys, start_index, chunk_size, &local_super_lookup);
      a_tx.send(result).unwrap();      
    });
  }

  drop(tx);

  let mut smooth: [usize; MAX_FIELD_EXT] = [0; MAX_FIELD_EXT];
  for received in rx {
    for i in 0..MAX_FIELD_EXT {
      smooth[i] += received[i];
    }
  }

  for i in 0..MAX_FIELD_EXT {
    println!("{}: {}", i+1, smooth[i]);
  }
  println!();
  println!("Degree: {}, Final /168: {}", DEGREE, smooth[MAX_FIELD_EXT-1] as f32 / 168.0);
  println!("Total time: {:?}", now.elapsed());
}



fn is_smooth(iso_polys: &Vec<IsoPolynomial>, start: usize, len: usize, super_lut: &SuperType) -> [usize; MAX_FIELD_EXT] {
  let mut count: [usize; MAX_FIELD_EXT] = [0; MAX_FIELD_EXT];

  'outer: for i in start..(start+len) {
    if i >= iso_polys.len() {break;}
    let iso_poly = &iso_polys[i];
    let (poly, size) = iso_poly.deconstruct();
    // for n in 1..=MAX_FIELD_EXT as usize {
    if poly.has_singularity(&super_lut.0) {continue 'outer;}
    count[0] += size as usize;
    if poly.has_singularity(&super_lut.1) {continue 'outer;}
    count[1] += size as usize;
    if poly.has_singularity(&super_lut.2) {continue 'outer;}
    count[2] += size as usize;
    if poly.has_singularity(&super_lut.3) {continue 'outer;}
    count[3] += size as usize;
    if poly.has_singularity(&super_lut.4) {continue 'outer;}
    count[4] += size as usize;
    if poly.has_singularity(&super_lut.5) {continue 'outer;}
    count[5] += size as usize;
    if poly.has_singularity(&super_lut.6) {continue 'outer;}
    count[6] += size as usize;
    if poly.has_singularity(&super_lut.7) {continue 'outer;}
    count[7] += size as usize;
  }
  count
}
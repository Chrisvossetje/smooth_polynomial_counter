use std::{time::Instant, sync::{mpsc, Arc, Mutex}, thread};

use algebraic_types::{Polynomial, IsoPolynomial, Lookup};

use crate::algebraic_types::generate_iso_polynomials;

#[allow(non_snake_case)]
mod algebraic_types;

const DEGREE: usize = 5;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;

const NUM_THREADS: usize = 16;

const PRINTING: bool = false;

type SuperType = (Lookup<1>,Lookup<2>,Lookup<3>,Lookup<4>,Lookup<5>,Lookup<6>,Lookup<7>,Lookup<8>);

#[derive(Debug,Clone,Copy,PartialEq)]
struct CustomChunk {
  pub start: usize,
  pub end: usize,
}

fn main() {
  let start_time = Instant::now();

  println!("Generate lookup stuff");
  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);
 
  // Lookup Tables
  // CHANGE THIS: 
  let super_lookup: SuperType = ( Lookup::<1>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<2>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<3>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<4>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<5>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<6>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<7>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<8>::create(&normal, &part_x, &part_y, &part_z),
                                );

  let lookup_time = Instant::now();
  println!("Generating took: {:?}", (lookup_time-start_time));
  println!();


  // Polynomials
  println!("Generate isomorphic polynomials");
  let iso_polys = generate_iso_polynomials(&normal);
  let poly_time = Instant::now();
  println!("Generating took: {:?}", (poly_time-lookup_time));
  println!();

  
  
  println!("Generate chunks and start counting smooth polynomials!");
  // let chunk_size = (iso_polys.len() + NUM_THREADS - 1) / (NUM_THREADS);
  let chunk_size = 1024;
  
  // Chunk generation so threads get fed evenly
  let mut chunks = Vec::new();
  let mut start = 0;
  while start < iso_polys.len() {
    chunks.push(CustomChunk {
        start,
        end: std::cmp::min(start + chunk_size, iso_polys.len()), // end is exclusive
    });
    start += chunk_size;
  }
  let chunk_length = chunks.len();

  println!("Amount of chunks: {} | Amount of threads: {}", chunks.len(), NUM_THREADS);
  println!();
                       
  // Thread arc stuff
  let (tx, rx) = mpsc::channel();


  let arc_super_lookup = Arc::new(super_lookup);
  let arc_iso_polys = Arc::new(iso_polys);
  let arc_chunks = Arc::new(Mutex::new(chunks));
  
  for _ in 0..NUM_THREADS {
    // Clone the sender to move into each thread
    let a_tx = tx.clone();

    // Clone the recomputed results to move into each thread locally
    let local_super_lookup = arc_super_lookup.clone();
    let local_iso_polys = arc_iso_polys.clone();
    let local_chunks = arc_chunks.clone();

    // Spawn a new thread
    thread::spawn(move || {
      // let lol = &local_chunks.lock().unwrap().pop();
      loop {
        let (start,end, index);
        {
          let chunk_vec = &mut local_chunks.lock().unwrap();
          let chunk = chunk_vec.pop();
          match chunk {
            Some(t) => { start = t.start; end = t.end; index = chunk_vec.len()}
            None => {return;}
          }
        }
        if PRINTING {
          println!("Chunks left: {index} | Total Chunks: {chunk_length} | Estimated time: {}", index as f64 * (Instant::now() - poly_time).as_secs_f64() / (chunk_length - index) as f64);
        }

        let result =  
          is_smooth(&local_iso_polys, start, end, &local_super_lookup);
        a_tx.send(result).unwrap();      
      }

    });
  }

  drop(tx);

  let mut smooth: [usize; 10] = [0; 10];
  for received in rx {
    for i in 0..10 {
      smooth[i] += received[i];
    }
  }

  for i in 0..10 {
    println!("{}: {}", i+1, smooth[i]);
  }
  println!();
  println!("Degree: {}", DEGREE);
  println!("Total time: {:?}", start_time.elapsed());
}



fn is_smooth(iso_polys: &Vec<IsoPolynomial>, start: usize, end: usize, super_lut: &SuperType) -> [usize; 10] {
  let mut count: [usize; 10] = [0; 10];

  'outer: for i in start..end {
    if i >= iso_polys.len() {break;}
    let iso_poly = &iso_polys[i];
    let (poly, size) = iso_poly.deconstruct();

    // CHANGE THIS: 
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
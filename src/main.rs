use std::{time::Instant, sync::{mpsc, Arc}, thread, vec, cmp::Ordering};

use algebraic_types::{Polynomial, IsoPolynomial};

use crate::algebraic_types::{Term, FieldExtension, generate_iso_polynomials, generate_lut_degrees};

#[allow(non_snake_case)]
mod algebraic_types;

const DEGREE: usize = 5;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;

const MAX_FIELD_EXT: usize = 6;

const NUM_THREADS: usize = 16;

const SECOND_HALF: bool = false;

fn main() {
  let now = Instant::now();

  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);

  let mut order_results = Vec::new();

  for n in 1..=MAX_FIELD_EXT as u32 {
    let mut temp = Vec::new();
    for order in 0..=DEGREE as u8{
      temp.push(generate_lut_degrees(order, n));
    }
    order_results.push(temp);
  }


  println!("Generated lookup stuff");
  let iso_polys = generate_iso_polynomials(&normal);
  
  let arc_normal_results = Arc::new(normal);
  let arc_part_x_results = Arc::new(part_x);
  let arc_part_y_results = Arc::new(part_y);
  let arc_part_z_results = Arc::new(part_z);
  
  let arc_order_results = Arc::new(order_results);

  println!("Generated isomorphic polynomials");
  
  println!("Start counting!");

  let chunk_size = (iso_polys.len() + NUM_THREADS - 1) / (NUM_THREADS);
  let half = chunk_size * NUM_THREADS;

  

  let arc_iso_polys = Arc::new(iso_polys);


  let (tx, rx) = mpsc::channel();

  for i in 0..NUM_THREADS {
    // Clone the sender to move into each thread
    let a_tx = tx.clone();

    // Clone the recomputed results to move into each thread locally
    let local_normal_results = arc_normal_results.clone();
    let local_part_x_results = arc_part_x_results.clone();
    let local_part_y_results = arc_part_y_results.clone();
    let local_part_z_results = arc_part_z_results.clone();



    let local_iso_polys = arc_iso_polys.clone();
    let local_order_results = arc_order_results.clone();
    
    let mut start_index = i*chunk_size;

    if SECOND_HALF {
      start_index += half;
    }

    // Spawn a new thread
    thread::spawn(move || {
      let result =  
      is_smooth(&local_iso_polys, start_index, chunk_size, &local_order_results, &local_normal_results, &local_part_x_results, &local_part_y_results, &local_part_z_results);
        // is_smooth(&local_iso_polys, start_index, chunk_size, &local_normal_results, &local_part_x_results, &local_part_y_results, &local_part_z_results);
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



fn is_smooth(iso_polys: &Vec<IsoPolynomial>, start: usize, len: usize, orders: &Vec<Vec<Vec<FieldExtension>>>, normal: &Vec<Term>, part_x:  &Vec<Term>,  part_y:  &Vec<Term>,  part_z:  &Vec<Term>) -> [usize; MAX_FIELD_EXT] {
  let mut count: [usize; MAX_FIELD_EXT] = [0; MAX_FIELD_EXT];

  'outer: for i in start..(start+len) {
    if i >= iso_polys.len() {break;}
    let iso_poly = &iso_polys[i];
    let (poly, size) = iso_poly.deconstruct();
    for n in 1..=MAX_FIELD_EXT as usize {
      if poly.has_singularity_alt2(&orders[n-1], normal, part_x, part_y, part_z, n as u32) {continue 'outer;}        // 823543
      // if poly.has_singularity(&normal_results[n-1], &part_x_results[n-1], &part_y_results[n-1], &part_z_results[n-1], n as u32) {continue 'outer;}        // 823543
      count[n-1] += size as usize;
    }
  }
  count
}
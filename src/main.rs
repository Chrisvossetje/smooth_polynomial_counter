use std::{time::Instant, sync::{mpsc, Arc, Mutex}, thread, string, fs};

use algebraic_types::{Polynomial, IsoPolynomial, Lookup, Term};

use crate::algebraic_types::{generate_iso_polynomials, Matrix};

#[allow(non_snake_case)]
mod algebraic_types;

const DEGREE: usize = 5;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;

const MAX_FIELD_EXT: usize = 8;


const NUM_THREADS: usize = 16;
const CHUNK_SIZE: usize = 1024;
const PRINTING: bool = false;

const FILE_NAME: &str = "./output.txt";

// CHANGE THIS:
type SuperType = (Lookup<1>,Lookup<2>,Lookup<3>,Lookup<4>,Lookup<5>,Lookup<6>,
                  Lookup<7>,Lookup<8>,
                  // Lookup<9>,Lookup<10>,
                  );

#[derive(Debug,Clone,Copy,PartialEq)]
struct CustomChunk {
  pub start: usize,
  pub end: usize,
}

fn main() {
  let start_time = Instant::now();

  // loop over all possible binary matrices
  let mut pgl3_f2: Vec<Matrix> = vec![];
  for i in 0..(1<<9) {
    let mut data: [[u8;3];3] = [[0;3];3];
    for j in 0..9 {
      data[j/3][j%3] = ((i >> j) & 1) as u8;
    }
    let matrix = Matrix::new(data);
    if matrix.determinant() % 2 == 1 {
      pgl3_f2.push(matrix);
    }
  }

  println!("Number of matrices: {}", pgl3_f2.len());
  println!();


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
                                  // Lookup::<9>::create(&normal, &part_x, &part_y, &part_z),
                                  // Lookup::<10>::create(&normal, &part_x, &part_y, &part_z),
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

  
  
  // Chunk generation so threads get fed evenly
  println!("Generate chunks, start threads and count smooth polynomials!");
  
  let mut chunks = Vec::new();
  let mut start = 0;
  while start < iso_polys.len() {
    chunks.push(CustomChunk {
        start,
        end: std::cmp::min(start + CHUNK_SIZE, iso_polys.len()), // end is exclusive
    });
    start += CHUNK_SIZE;
  }
  let chunk_length = chunks.len();

  println!("Amount of Isomorphic polynomials: {} | Amount of chunks: {} | Amount of threads: {}",iso_polys.len(), chunks.len(), NUM_THREADS);
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

  let mut smooth: [usize; MAX_FIELD_EXT] = [0; MAX_FIELD_EXT];
  let mut results = Vec::new();
  for (count, mut result) in rx {
    for i in 0..MAX_FIELD_EXT {
      smooth[i] += count[i];
    }
    results.append(&mut result);
  }
  

  let a: Vec<String> = results.iter().map(|t| t.to_string(&normal)).collect();
  let b = a.join("\n");
  fs::write(FILE_NAME, b).expect("Unable to write file");
  

  for i in 0..MAX_FIELD_EXT {
    println!("{}: {}", i+1, smooth[i]);
  }
  println!();
  println!("Amount of isomorphism classes: {}. Polynomials had Degree: {}", results.len(),  DEGREE);
  println!("Total time: {:?}", start_time.elapsed());
}


struct PolynomialResult {
  poly: IsoPolynomial,
  points_on_curve: [usize; MAX_FIELD_EXT],
}

impl PolynomialResult {
  pub fn new(iso_poly: IsoPolynomial, points_on_curve: [usize; MAX_FIELD_EXT]) -> PolynomialResult {
    PolynomialResult { poly: iso_poly, points_on_curve }
  }

  pub fn to_string(&self, normal: &Vec<Term>) -> String {
    format!("{} | {} | {:?}", self.poly.representative.str(normal), self.poly.size, self.points_on_curve)
  }
}


fn is_smooth(iso_polys: &Vec<IsoPolynomial>, start: usize, end: usize, super_lut: &SuperType) -> ([usize; MAX_FIELD_EXT], Vec<PolynomialResult>) {
  let mut count: [usize; MAX_FIELD_EXT] = [0; MAX_FIELD_EXT];
  let mut results: Vec<PolynomialResult> = Vec::new();
  for i in start..end {
    if i >= iso_polys.len() {break;}
    let iso_poly = &iso_polys[i];
    let (poly, size) = iso_poly.deconstruct();
    let mut points_on_curve = [0; MAX_FIELD_EXT];

    // CHANGE THIS: 
    let result = poly.has_singularity(&super_lut.0);
    if result == None {continue;}
    count[0] += size as usize;
    points_on_curve[0] += result.unwrap();

    let result = poly.has_singularity(&super_lut.1);
    if result == None {continue;}
    count[1] += size as usize;
    points_on_curve[1] += result.unwrap();

    let result = poly.has_singularity(&super_lut.2);
    if result == None {continue;}
    count[2] += size as usize;
    points_on_curve[2] += result.unwrap();

    let result = poly.has_singularity(&super_lut.3);
    if result == None {continue;}
    count[3] += size as usize;
    points_on_curve[3] += result.unwrap();

    let result = poly.has_singularity(&super_lut.4);
    if result == None {continue;}
    count[4] += size as usize;
    points_on_curve[4] += result.unwrap();

    let result = poly.has_singularity(&super_lut.5);
    if result == None {continue;}
    count[5] += size as usize;
    points_on_curve[5] += result.unwrap();

    let result = poly.has_singularity(&super_lut.6);
    if result == None {continue;}
    count[6] += size as usize;
    points_on_curve[6] += result.unwrap();

    let result = poly.has_singularity(&super_lut.7);
    if result == None {continue;}
    count[7] += size as usize;
    points_on_curve[7] += result.unwrap();

    // let result = poly.has_singularity(&super_lut.8);
    // if result == None {continue;}
    // count[8] += size as usize;
    // points_on_curve[8] += result.unwrap();

    // let result = poly.has_singularity(&super_lut.9);
    // if result == None {continue;}
    // count[9] += size as usize;
    // points_on_curve[9] += result.unwrap();


    results.push(PolynomialResult::new(*iso_poly, points_on_curve))
  }
  (count,results)
}
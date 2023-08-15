use std::{time::Instant, sync::{mpsc, Arc, Mutex}, thread, fs};

use algebraic_types::{IsoPolynomial, Lookup, PolynomialResult};

use crate::{algebraic_types::generate_iso_polynomials, polynomials::{Polynomial, generate_transform_lut}, algebraic_types::Matrix, field_extensions::{F2_i, FieldTraits}};




#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod algebraic_types;
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod polynomials;
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod field_extensions;

const DEGREE: usize = 5;
const FIELD_ORDER: usize = 3;


const FIELD_EXT_LUT: [usize; 7] = [1,1,2,3,4,6,10];
const MAX_FIELD_EXT: usize = FIELD_EXT_LUT[DEGREE];

// Q^21 - 1 / 2
const POLYNOMIALS: usize = (FIELD_ORDER.pow(21) - 1) / 2;
const DPLUS2_CHOOSE_2: usize = ((DEGREE+2) * (DEGREE+1)) / 2;


const NUM_THREADS: usize = 16;
const CHUNK_SIZE: usize = 50;
const PRINTING: bool = false;

const FILE_NAME: &str = "./output.txt";

// CHANGE THIS:
type SuperType = (Lookup<1>,Lookup<2>,Lookup<3>, Lookup<4>,Lookup<5>,Lookup<6>,
                  // Lookup<7>,Lookup<8>, Lookup<9>,Lookup<10>,
                  );

#[derive(Debug,Clone,Copy,PartialEq)]
struct CustomChunk {
  pub start: usize,
  pub end: usize,
}



fn main() {
  let start_time = Instant::now();

  println!("Generate terms");
  let normal = Polynomial::generate_default_lut();
  let (part_x, part_y, part_z) = Polynomial::generate_derative_luts(&normal);
  
  println!("Importing file");
  let input = fs::read_to_string("input.txt").expect("Unable to open file");
  let mut lines = input.lines();
  // Verifying file validity against program 
  {
    lines.next();
    lines.next();
    let degree_field_order = lines.next().expect("Incorrect format. Expected the following: \n#Blablabla \n# Homogenous Degree | Field Order \n5 | 3 \n# Constant_(xpower)(ypower)(zpower) ... Constant_(xpower)(ypower)(zpower)  | Isomorphism Class size \n1_023 2_500 | 4 ..."); 
    
    let splits: Vec<&str> = degree_field_order.split("|").collect::<Vec<&str>>();
    let degree = splits[0].split_ascii_whitespace().next().unwrap().parse::<usize>().unwrap();
    let field_order = splits[1].split_ascii_whitespace().next().unwrap().parse::<usize>().unwrap();
    lines.next();
    
    if degree != DEGREE {
      panic!("Degree of input file not equal to compiled program")
    }

    if field_order != FIELD_ORDER {
      panic!("Field order of input file not equal to compiled program")
    }
  }

  let iso_polys = {
    let mut iso_polys = vec![];

    for l in lines {
      let mut line_iter = l.split("|");
      let polynomial = line_iter.next().unwrap();
      let iso_size = line_iter.next().unwrap().split_ascii_whitespace().next().unwrap().parse::<u32>().unwrap();

      iso_polys.push(IsoPolynomial {
          representative: Polynomial::from_string(polynomial, &normal),
          size: iso_size,
      });
    } 
    iso_polys
  };

  let import_time = Instant::now();
  println!("Generating took: {:?}", (import_time-start_time));
  println!();  

  // return;


  // Generating Lookup Tables
  // CHANGE THIS: 
  println!("Generating Lookup tables");
  let super_lookup: SuperType = ( Lookup::<1>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<2>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<3>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<4>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<5>::create(&normal, &part_x, &part_y, &part_z),
                                  Lookup::<6>::create(&normal, &part_x, &part_y, &part_z),
                                  // Lookup::<7>::create(&normal, &part_x, &part_y, &part_z),
                                  // Lookup::<8>::create(&normal, &part_x, &part_y, &part_z),
                                  // Lookup::<9>::create(&normal, &part_x, &part_y, &part_z),
                                  // Lookup::<10>::create(&normal, &part_x, &part_y, &part_z),
                                );

  let lookup_time = Instant::now();
  println!("Generating took: {:?}", (lookup_time-start_time));
  println!();  
  

  //
  // Chunk generation so threads get fed evenly
  //
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
          println!("Chunks left: {index} | Total Chunks: {chunk_length} | Estimated time: {:.2}", index as f64 * (Instant::now() - lookup_time).as_secs_f64() / (chunk_length - index) as f64);
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
  println!("Amount of isomorphism classes: {}",results.len());
  println!("Polynomials had Degree: {}",  DEGREE);
  println!("Total time: {:?}", start_time.elapsed());
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

    // let result = poly.has_singularity(&super_lut.6);
    // if result == None {continue;}
    // count[6] += size as usize;
    // points_on_curve[6] += result.unwrap();

    // let result = poly.has_singularity(&super_lut.7);
    // if result == None {continue;}
    // count[7] += size as usize;
    // points_on_curve[7] += result.unwrap();

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
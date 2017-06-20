extern crate rand;

use std::cmp::{Ord, Ordering, PartialOrd, PartialEq, Eq};
use std::ops::Mul;
use self::rand::distributions::{IndependentSample, Range};
use self::rand::ThreadRng;

#[derive(Clone)]
struct AlgInfo {
    pub squars:        i32,
    pub rng:           ThreadRng,
    pub sqr_rng:       Range<i32>,
    pub mut_prob_rng:  Range<f64>,
    pub mutation_prob: f64,
}

struct Gen<'a> {
    pub info: &'a mut AlgInfo,
    pub data: Vec<(i32, i32)>,
    pub val: i32,
}

impl<'a> Ord for Gen<'a> {
    fn cmp(&self, other: &Gen<'a>) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl<'a> PartialOrd for Gen<'a> {
    fn partial_cmp(&self, other: &Gen<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Gen<'a> {
    fn eq(&self, other: &Gen<'a>) -> bool {
        self.val == other.val
    }
}

impl<'a> Eq for Gen<'a> {}

impl<'a> Gen<'a> {
    fn new(info: &'a mut AlgInfo) -> Self {
        let mut data = vec![(0i32, 0i32); info.squars as usize];
        for i in 0..info.squars as usize {
            data[i].0 = info.sqr_rng.ind_sample(&mut info.rng);
            data[i].1 = info.sqr_rng.ind_sample(&mut info.rng);
        }
        Gen {
            info: info,
            data: data,
            val: 0,
        }
    }

    fn eval(&mut self) {
        self.val = 0;
        for i in 0..self.data.len() {
            let r = self.data[i].0;
            let c = self.data[i].1;
            for j in i..self.data.len() {
                let rr = self.data[j].0;
                let cc = self.data[j].1;
                if
                    r == rr ||
                    c == cc ||
                    r - c == rr - cc ||
                    r + c == rr + cc {
                    self.val += 1;
                }
            }
        }
    }

    fn crossover<'b, 'c>(&mut self, o: &Gen<'b>) -> Gen<'c> {
        Gen::new(self.info)
    }
}

pub fn execute(squars: i32) {
    let mut rng = rand::thread_rng();
    let sqr_rng = Range::new(0i32, squars);
    let mut_prob_rng = Range::new(0.025f64, 0.0375);
    let pop_num_rng = Range::new(75usize, 125);
    let mutation_prob = mut_prob_rng.ind_sample(&mut rng);
    let population_number = pop_num_rng.ind_sample(&mut rng);
    let termination_loop = 100i32;
    let mut alg_info = AlgInfo {
        squars:        squars,
        rng:           rng,
        sqr_rng:       sqr_rng,
        mut_prob_rng:  Range::new(0f64, 1.0),
        mutation_prob: mutation_prob,
    };
    let mut population = vec![Gen::new(&mut alg_info); population_number];
    for i in 0..population_number {
        population[i] = Gen::new(squars);
    }
    for _ in 0..termination_loop {
        for p in &mut population {
            println!("{:?}", p);
            p.eval();
        }
        population.sort();
        let pop_len = population.len();
        population = population[pop_len - population_number..pop_len].to_vec();
        // do cross
    }

}

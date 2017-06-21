extern crate rand;

use std::cmp::{Ord, Ordering, PartialOrd, PartialEq, Eq};
use self::rand::distributions::{IndependentSample, Range};
use self::rand::ThreadRng;

struct AlgInfo {
    pub squars:        i32,
    pub rng:           ThreadRng,
    pub sqr_rng:       Range<i32>,
    pub mut_prob_rng:  Range<f64>,
    pub mutation_prob: f64,
}

#[derive(Clone, Debug)]
struct Gen {
    pub data: Vec<(i32, i32)>,
    pub val: i32,
}

impl Ord for Gen {
    fn cmp(&self, other: &Gen) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Gen {
    fn partial_cmp(&self, other: &Gen) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Gen {
    fn eq(&self, other: &Gen) -> bool {
        self.val == other.val
    }
}

impl Eq for Gen {}

impl Gen {
    fn new(info: &mut AlgInfo) -> Self {
        let mut data = vec![(0i32, 0i32); info.squars as usize];
        for i in 0..info.squars as usize {
            data[i].0 = info.sqr_rng.ind_sample(&mut info.rng);
            data[i].1 = info.sqr_rng.ind_sample(&mut info.rng);
        }
        Gen {
            data: data,
            val: 0,
        }
    }

    fn eval(&mut self) {
        self.val = 0;
        for i in 0..self.data.len() {
            let r = self.data[i].0;
            let c = self.data[i].1;
            for j in (i + 1)..self.data.len() {
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

    fn crossover(&self, o: &Gen, info: &mut AlgInfo) -> Gen {
        let mut data = vec![(0i32, 0i32); info.squars as usize];
        let cross_point = info.sqr_rng.ind_sample(&mut info.rng) as usize;
        for i in 0..info.squars as usize {
            if i < cross_point {
                data[i] = self.data[i];
            } else {
                data[i] = o.data[i];
            }
            let mutate = info.mut_prob_rng.ind_sample(&mut info.rng);
            if mutate < info.mutation_prob {
                data[i].0 = info.sqr_rng.ind_sample(&mut info.rng);
                data[i].1 = info.sqr_rng.ind_sample(&mut info.rng);
            }
        }
        let mut child = Gen {
            data: data,
            val:  0,
        };
        child.eval();
        return child;
    }

    fn print(&self) {
        print!("  ");
        for i in 0..self.data.len() {
            print!("{}", i + 1);
        }
        println!("  ");
        for i in 0..self.data.len() {
            print!("{}-", i + 1);
            let mut v = Vec::new();
            for j in 0..self.data.len() {
                if self.data[j].1 == i as i32 {
                    v.push(self.data[j].0);
                }
            }
            for j in 0..self.data.len() {
                if v.contains(&(j as i32)) {
                    print!("X");
                } else {
                    print!("O");
                }
            }
            println!("-{}", i + 1);
        }
        print!("  ");
        for i in 0..self.data.len() {
            print!("{}", i + 1);
        }
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
        population[i] = Gen::new(&mut alg_info);
        population[i].eval();
    }
    for _ in 0..termination_loop {
        population.sort();
        if population[0].val == 0 {
            break;
        }
        population = population[0..population_number].to_vec();
        let mut new_generation = Vec::new();
        let population_number = population_number - 1;
        for i in 0..population_number {
            new_generation.push(population[i].crossover(&population[i + 1], &mut alg_info));
        }
        population.append(&mut new_generation);
    }
    println!("Best solution found in pure genetic algorithm {:?}", population[0]);
    population[0].print();
}

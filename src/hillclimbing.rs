extern crate rand;

use self::rand::distributions::{IndependentSample, Range};

pub fn heuristic(bits: u64, pos: u64, queens: u64) -> u64 {
    let mask = (1 << bits) - 1;
    let mut val = 0u64;
    for i in 0..queens {
        let r = ((pos >> (bits * i * 2)) & mask) as i64;
        let c = ((pos >> (bits * ((i * 2) + 1))) & mask) as i64;
        for j in (i + 1)..queens {
            let rr = ((pos >> (bits * j * 2)) & mask) as i64;
            let cc = ((pos >> (bits * ((j * 2) + 1))) & mask) as i64;
            if
                r == rr ||
                c == cc ||
                r - c == rr - cc ||
                r + c == rr + cc {
                val += 1;
            }
        }
    }
    val
}

pub fn paint(squars: u64, bits: u64, pos: u64, queens: u64) {
    let mask = (1 << bits) - 1;
    let mut poses = Vec::new();
    for i in 0..queens {
        let r = (pos >> (bits * i * 2)) & mask;
        let c = (pos >> (bits * ((i * 2) + 1))) & mask;
        poses.push((c, r));
    }
    print!("  ");
    for i in 0..squars {
        print!("{}", i);
    }
    println!("");
    for i in 0..squars {
        print!("{}-", i);
        let mut rs = Vec::new();
        for p in &poses {
            if p.1 == i {
                rs.push(p.0);
            }
        }
        for j in 0..squars {
            if rs.contains(&j) {
                print!("X");
            } else {
                print!("O");
            }
        }
        println!("-{}", i);
    }
    print!("  ");
    for i in 0..squars {
        print!("{}", i);
    }
    println!("");
}

pub fn execute(queens: u64, squars: u64, temp_rate: f64) {
    let mut rng = rand::thread_rng();
    let mut bits = 0u64;
    for i in 0..32u64 {
        if (squars - 1) >> i == 0 {
            bits = i;
            break;
        }
    }
    println!("Bits sample is: {}", bits);
    let position_mask = (1u64 << (queens * 2 * bits)) - 1;
    let start_rng = Range::new(1u64 << (queens * 2 * bits), u64::max_value());
    let mut current_speed = start_rng.ind_sample(&mut rng);
    let mut current_position = start_rng.ind_sample(&mut rng) & position_mask;
    let mut current_score = heuristic(bits, current_position, queens);
    let mut best_position = current_position;
    let mut best_score = current_score;
    let max_iteration = 30000000u64;
    for _ in 0..max_iteration {
        if current_speed == 0 || best_score == 0 {
            break;
        }
        let neighbor1 = current_position.wrapping_add(current_speed) & position_mask;
        let neighbor1_val = heuristic(bits, neighbor1, queens);
        let neighbor2 = current_position.wrapping_sub(current_speed) & position_mask;
        let neighbor2_val = heuristic(bits, neighbor2, queens);
        if neighbor1_val < current_score && neighbor1_val < neighbor2_val {
            current_score = neighbor1_val;
            current_position = neighbor1;
        } else /*if neighbor2_val < current_score && neighbor2_val < neighbor1_val*/ {
            current_score = neighbor2_val;
            current_position = neighbor2;
        }
        if current_score < best_score {
            best_position = current_position;
            best_score = current_score;
        }
        current_speed = (temp_rate * (current_speed as f64)) as u64;
    }
    println!("Current speed is: {}", current_speed);
    println!("Best score is: {}", best_score);
    paint(squars, bits, best_position, queens);

}

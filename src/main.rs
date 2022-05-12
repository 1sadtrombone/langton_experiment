use rand::prelude::*;
use std::fs;

fn random_ics<const N: usize>(k: usize) -> [u8; N] {
    let mut rng = thread_rng();
    let mut arr = [0u8; N];
    for item in arr.iter_mut() {
        *item = rng.gen_range(0..k) as u8;
    }
    arr
}

fn random_rule<const RULE_NUM: usize>(k: usize) -> [u8; RULE_NUM] {
    let mut rng = thread_rng();
    let mut rules = [0u8; RULE_NUM];
    for rule in rules.iter_mut() {
        *rule = rng.gen_range(0..k) as u8;
    }
    rules
}

fn random_rule_no_dead<const RULE_NUM: usize>(k: usize) -> [u8; RULE_NUM] {
    let mut rng = thread_rng();
    let mut rules = [0u8; RULE_NUM];
    for rule in rules.iter_mut() {
        *rule = rng.gen_range(1..k) as u8;
    }
    rules
}

fn get_rule_with_bjs<const RULE_NUM: usize, const R: usize, const OPT_NUM: usize, const K: usize>(bjs: [f32; OPT_NUM]) -> [u8; RULE_NUM] {
    // if invalid bjs were given, they will be rounded to valid ones!

    let mut num_zeros = [0usize; RULE_NUM]; // can save time by making this global or passed
    let mut denoms = [0u8; OPT_NUM]; // total num of rules with [index] zeros
    let mut picked_inds = [0u8; RULE_NUM];
    let mut max_counts = [0u8; OPT_NUM];
    let mut rules = [0u8; RULE_NUM];

    // converting bjs back to counts
    for j in 0..R {
        for i in 0..RULE_NUM {
            if i%(K.pow(j as u32 + 1)) < K.pow(j as u32) {
                num_zeros[i] += 1;
            }
        }
    }

    for i in 0..RULE_NUM {
        denoms[num_zeros[i]] += 1;
    }

    for j in 0..OPT_NUM {
        max_counts[j] = (bjs[j] * denoms[j]).round() as u8; // THE ROUNDING
    }

    for j in 0..OPT_NUM {
        // while count < max_counts[j]
            // randomly pick an index with j zeros that isn't in picked_inds
            // make the rule assign to a random live state
            // increment count
    }

    rules
}

fn get_nu<const R: usize, const OPT_NUM: usize>(bjs: [f32; OPT_NUM]) -> f32 {
    let mut nu: f32 = 0.;

    for j in 0..OPT_NUM {
        // nu += 
        // urgh it's defined implicitly... I hate math
    }
    0f32
}

fn get_rule_bjs<const RULE_NUM: usize, const R: usize, const OPT_NUM: usize, const K: usize>(rules: &[u8; RULE_NUM]) -> [f32; OPT_NUM] {
    
    // the bjs are the fraction of neighbourhoods with j 0's which map to 0
    // OPT_NUM MUST be R+1. For eg. R=3, can have 0, 1, 2, or 3 zeros: 4 options
    let mut bjs = [0f32; OPT_NUM];
    let mut num_zeros = [0usize; RULE_NUM]; // get num zeros at each index analytically?
    let mut denoms = [0u8; OPT_NUM]; // total num of rules with [index] zeros

    // getting num_zeros at each index
    for j in 0..R {
        for i in 0..RULE_NUM {
            if i%(K.pow(j as u32 + 1)) < K.pow(j as u32) {
                num_zeros[i] += 1;
            }
        }
    }
   
    // getting denoms
    for i in 0..RULE_NUM {
        denoms[num_zeros[i]] += 1;
    }

    for i in 0..RULE_NUM {
        if rules[i] == 0 {
            bjs[num_zeros[i]] += 1.;
        }
    }

    for j in 0..OPT_NUM {
        bjs[j] /= denoms[j] as f32;
    }
    bjs
}

fn write_series<const RULE_NUM: usize, const N: usize, const T: usize, const R: usize, const K: usize>(write_path: &str, rules: [u8; RULE_NUM], ics: [u8; N]) -> () {
    /* 
        Rules are understood to be an exhaustive list of possible results
        where each entry corresponds to a certain permutation of possible
        cell states, set by k and r.
        The order is like counting up in base k: 00, 01 .. 0k, 10 .. kk.
        There must be k^r entries.
    */ 

    let mut series = [[0u8; N]; T];
    let mut applied_rule: u8;

    series[0] = ics;

    let mut write_content: String = "".to_owned();

    for t in 0..T-1 {
        for i in 0..N {
            if  i != 0 {
                write_content.push_str(&",");
            } 
            write_content.push_str(&series[t][i].to_string());
            
            applied_rule = 0;
            // find which rule applies by calculating index (where in the order of increasing base-K the number is formed by the neighbourhood)
            for j in 0..R {
                // check for boundaries
                if (i as i64)-((R as i64-1)/2)+(j as i64) < 0 {
                    applied_rule += series[t][i+j+N-(R-1)/2]*(K as u8).pow((R-j-1usize) as u32);
                }
                else if i+j-(R-1)/2 >= N {
                    applied_rule += series[t][i+j-(R-1)/2-N]*(K as u8).pow((R-j-1usize) as u32);
                }
                else {
                    applied_rule += series[t][i+j-(R-1)/2]*(K as u8).pow((R-j-1usize) as u32); // for R=3,K=3, we have [1]*K*K+[2]*K+[3] = index of rule. We start at -(R-1)/2, ie on the left end
                }
            }
            series[t+1][i] = rules[applied_rule as usize];
        }
        write_content.push_str(&"\n");
    }

    // IO time
    fs::write(write_path, write_content).expect("Unable to write file...");
}

fn get_series<const RULE_NUM: usize, const N: usize, const T: usize, const R: usize, const K: usize>(rules: [u8; RULE_NUM], ics: [u8; N]) -> [[u8;N];T] {
    /* 
        Rules are understood to be an exhaustive list of possible results
        where each entry corresponds to a certain permutation of possible
        cell states, set by k and r.
        The order is like counting up in base k: 00, 01 .. 0k, 10 .. kk.
        There must be k^r entries.
    */ 

    let mut series = [[0u8; N]; T];
    let mut applied_rule: u8;

    series[0] = ics;

    for t in 0..T-1 {
        for i in 0..N {      
            applied_rule = 0;
            // find which rule applies by calculating index (where in the order of increasing base-K the number is formed by the neighbourhood)
            for j in 0..R {
                // check for boundaries
                if (i as i64)-((R as i64-1)/2)+(j as i64) < 0 {
                    applied_rule += series[t][i+j+N-(R-1)/2]*(K as u8).pow((R-j-1usize) as u32);
                }
                else if i+j-(R-1)/2 >= N {
                    applied_rule += series[t][i+j-(R-1)/2-N]*(K as u8).pow((R-j-1usize) as u32);
                }
                else {
                    applied_rule += series[t][i+j-(R-1)/2]*(K as u8).pow((R-j-1usize) as u32); // for R=3,K=3, we have [1]*K*K+[2]*K+[3] = index of rule. We start at -(R-1)/2, ie on the left end
                }
            }
            series[t+1][i] = rules[applied_rule as usize];
        }
    }
    series
}

fn main() {
    const N: usize = 500; // size of box
    const T: usize = 1000; // steps to run
    const K: usize = 2; // number of cell states (incl. 0, quiescent)
    const R: usize = 3; // neighbourhood size
    const RULE_NUM: usize = 8; // MUST BE k^r. Easier to use a calculator than get this to work in a low-level language

    
    let mut rules = [0u8, 1u8, 0u8, 1u8, 1u8, 0u8, 0u8, 0u8];
    let mut ics = [0u8; N];
    ics[N/2] = 1;
    let write_path = "../../target/series.csv";

    let mut bjs = get_rule_bjs::<RULE_NUM, R, {R+1}, K>(&rules);
    println!("{:?}", bjs);

    //write_series::<RULE_NUM, N, T, R, K>(write_path, rules, ics);
    

    //nu_mfs = (0..100 as f32)/100.;
    //println!("{}", nu_mfs);

    println!("{}", ((2u8 as f32)/3.).round() as u8);
}


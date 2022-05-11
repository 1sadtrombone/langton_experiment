use rand::prelude::*;
use std::fs;

fn random_ics<const N: usize>(k: usize) -> [u8; N]{
    let mut rng = thread_rng();
    let mut arr = [0u8; N];
    for item in arr.iter_mut() {
        *item = rng.gen_range(0..k) as u8;
    }
    arr
}

fn main() {
    const N: usize = 500; // size of box
    const T: usize = 1000; // steps to run
    const K: usize = 2; // number of cell states (incl. 0, quiescent)
    const R: usize = 3; // neighbourhood size
    const RULE_NUM: usize = 8; // MUST BE k^r. Easier to use a calculator than get this to work in a low-level language

    /* 
        Rules are understood to be an exhaustive list of possible results
        where each entry corresponds to a certain permutation of possible
        cell states, set by k and r.
        The order is like counting up in base k: 00, 01 .. 0k, 10 .. kk.
        There must be k^r entries.
    */ 

    //let mut rules = [0usize; RULE_NUM];
    let mut rules = [0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8];
    let mut series = [[0u8; N]; T];
    let mut applied_rule: u8;

    //series[0] = random_ics(K);
    series[0][N/2] = 1;
    println!("{:?}", series[0]);

    let write_path = "../../target/series.csv";
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


use std::collections::HashSet;

// Function to compute the divisors of n
fn divisors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            divs.push(i);
        }
    }
    divs
}

// Function to generate a subgroup of Z_n given a generator
fn generate_subgroup(n: u64, generator: u64) -> HashSet<u64> {
    let mut subgroup = HashSet::new();
    let mut current = 0;
    while !subgroup.contains(&current) {
        subgroup.insert(current);
        current = (current + generator) % n;
    }
    subgroup
}

// Function to generate all subgroups of Z_n
fn generate_all_subgroups(n: u64) -> Vec<HashSet<u64>> {
    let mut subgroups = Vec::new();
    for d in divisors(n) {
        let generator = n / d;
        let subgroup = generate_subgroup(n, generator);
        subgroups.push(subgroup);
    }
    subgroups
}

// Function to generate all subgroups of Z_n x Z_m
fn generate_all_subgroups_product(n: u64, m: u64) -> Vec<HashSet<(u64, u64)>> {
    let subgroups_n = generate_all_subgroups(n);
    let subgroups_m = generate_all_subgroups(m);

    let mut subgroups_product = Vec::new();

    for subgroup_n in &subgroups_n {
        for subgroup_m in &subgroups_m {
            let mut subgroup_product = HashSet::new();
            for &a in subgroup_n {
                for &b in subgroup_m {
                    subgroup_product.insert((a, b));
                }
            }
            subgroups_product.push(subgroup_product);
        }
    }

    subgroups_product
}

fn main() {
    let n = 6; // Replace with the desired value of n
    let m = 4; // Replace with the desired value of m

    let subgroups = generate_all_subgroups_product(n, m);
    println!("Subgroups of Z_{} x Z_{}:", n, m);
    for subgroup in subgroups {
        let mut subgroup_vec: Vec<(u64, u64)> = subgroup.into_iter().collect();
        subgroup_vec.sort_unstable();
        println!("{:?}", subgroup_vec);
    }
}


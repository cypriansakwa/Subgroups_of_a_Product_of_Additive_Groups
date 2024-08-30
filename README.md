## Overview
This Rust program computes and displays all subgroups of the direct product of two cyclic groups, $\mathbb{Z}_n\times\mathbb{Z}_m$. Here, $\mathbb{Z}_n$ and $\mathbb{Z}_m$ are the additive groups of integers modulo $n$ and $m$, respectively. To effectively produce subgroups, the program takes advantage of cyclic group structure.
## Features
- **Divisors Calculation:** The program computes all divisors of a given number $n$, which are required to generate subgroups.
-  **Subgroup Generation for $\mathbb{Z}_n$:** For each divisor, a corresponding subgroup of $\mathbb{Z}_n$ is generated using an appropriate generator.
-  **Subgroup Generation for $\mathbb{Z}_n\times\mathbb{Z}_m$:** Forms the direct product of subgroups from $\mathbb{Z}_n$ and $\mathbb{Z}_m$ to generate subgroups of $\mathbb{Z}_n\times\mathbb{Z}_m$.
-  **Display Subgroups:** The program prints all distinct subgroups in a sorted manner.

## How It Works
- **divisors(n: u64) -> Vec<u64>:** Computes and returns a vector of all divisors of the integer $n$.
- **generate_subgroup(n: u64, generator: u64) -> HashSet<u64>:** Generates and returns the subgroup of $\mathbb{Z}_n$ using a given generator. The subgroup is returned as a HashSet to ensure uniqueness of elements.
- **generate_all_subgroups(n: u64) -> Vec<HashSet<u64>>:** Generates and returns a vector of all distinct subgroups of $\mathbb{Z}_n$. Each subgroup is computed by iterating over all divisors of $n$ and generating subgroups using appropriate generators.
- **generate_all_subgroups_product(n: u64, m: u64) -> Vec<HashSet<(u64, u64)>>:** Generates all subgroups of $\mathbb{Z}_n\times\mathbb{Z}_m$
  by taking the Cartesian product of subgroups from $\mathbb{Z}_n$ and $\mathbb{Z}_m$.
- The main function defines a value for $n$ and $m$ and uses the above functions to compute all subgroups of $\mathbb{Z}_n\times\mathbb{Z}_m$. The subgroups are then printed in sorted order for clarity.
 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- You can change the values of $n$ and $m$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
- This will compile and run the program, printing the subgroups of the specified $\mathbb{Z}_n\times\mathbb{Z}_m$.
## Customizing
- To compute subgroups for a different value of $n$ and $m$, simply modify the value of $n$ and $m$ in the main function.
## Example Output, Given $n=4$ and $m=3$:
>```
>Subgroups of Z_4 x Z_3:
>[(0, 0)]
>[(0, 0), (0, 1), (0, 2)]
>[(0, 0), (2, 0)]
>[(0, 0), (0, 1), (0, 2), (2, 0), (2, 1), (2, 2)]
>[(0, 0), (1, 0), (2, 0), (3, 0)]
>[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2), (3, 0), (3, 1), (3, 2)]
- This output lists all subgroups of $\mathbb{Z}_4\times\mathbb{Z}_3$, with each subgroup represented as a sorted list of elements.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Subgroups_of_a_Product_of_Additive_Groups.git
   cd Subgroups_of_a_Product_of_Additive_Groups

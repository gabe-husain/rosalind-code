
mod solutions;
use solutions::DNA::{count_nucleotide, count_nucleotide_onepass};


fn main() {
    let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let sol1 = count_nucleotide(dna);
    let sol2 = count_nucleotide_onepass(dna);
    println!("{} {} {} {}", sol1[0], sol1[1], sol1[2], sol1[3]);
    println!("{} {} {} {}", sol2[0], sol2[1], sol2[2], sol2[3]);
}


mod solutions;


//Problem 1
//use solutions::DNA::{count_nucleotide, count_nucleotide_onepass};
//Problem 2
use solutions::RNA::{transcribe};


fn main() {
    let string = "CTCGTGGCTGAGTCCCCGCGAAGTCGTCCTTAGGTTCGTATATAAATAGTCGTCTAGCAAGTAAGGAAATCTGTAACGATTCTCCTCAGGGAGCGCAATAGCGACACTCGCTGTTGCAATCGGTGCCGGGGACGGCGCAAGGACTTGTCCTCGCGACTAAGCTTCGCGCATCCTCACACATTAATGCGGGACGATCATGAATAGTTAAGAGGGAATCTGAATAATCCGTGCTTTTGTTCCCCTCTTCACTAGGGAAATGGTGAATGATACTCTCAAATGCGTTCAGGCATCGCGGACAAGGGAACCATATCACGTATTAAGCTATTTATCTAATCTAACTATCATGCTCCCTCAGCGTCCCACGGGGTTACAGGATTCAACTGTTTGTTGGCGGCCCACAAGGCAAACATATACCGTATTTGGTTGTGCCCCGCTACGCCTGACTGCGATACTTTATGTACCTACGTCGATCCTTAAGGGTCCTCGAGTATAACTCAGCCTTTCTTTCCGTTCTGCGTTAACACCCTACAGGCATTTTTTCAAACAACTACCGCATGCACATGAGCCTCATCGGCGGAATCTGAGTCGCCGCTCACTAATGGACGGCTCTCCCGAAGGCCAGTCCAATTACTAGGAAGTAGATCACCAAAGAGATTTGGTACCCCAGTTCAGCGCACCGAGATGATAGCCAGCGAAGTACAGCTCGAACCGAGATAAGGTCGACTTTCGCTATCCAAGTAAGAGTCAATCTGCGTGTTCTGCATTTACTTCTAGTAGTTCAAGCTTGAGGGACGTTTAGCCACTTGGGAAAAACGGGAACTGTGATGACCCAGCTACAAGTCTAAGAACACTTGGCAAAGGTCGTCTGGTTCCTCAAGCTCTCACTGAGACCCGGTACATCCCGCATAACGAGCCCTTGCGAGGACACGGAGTATAGCGTTAAATCTGGCCGTCCCATGGCCGTTCTAGTACTCGATTGTTAGA";
    let RNA: String = transcribe(string);
    println!("{}", RNA);
}

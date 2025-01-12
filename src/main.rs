//Counting DNA Nucleotides

fn main() {
    let string = "CAAAACATTGCTGGTAAGCAAAAGAAAGGGATCCTCACGTATGATCATGACATGGGGAATATGGGGTATTCCCCGTTTTTTGACACTTCAGTCCTAATAGGTACGAGGCGTCGGCCCCCATTAAAGATAAACTACGGAATAACCGGCGCAACTACCGTATGCGGTAAGCGCTCGGAGAACCCTCCCTAATTTCTGGCACTAGGGTCCCTGGCCAGCTTGCCAATATTATTACGATTGCGACAATCATGGGTCGAATAATAAAATTAGGTGGTGCAGGGGACGAGTGTGGGTGTTGTAAGGGCAGCAGGGGGCGCGGAATCTAGACCCCCCCAGAAATCCGACTCACTACTTAACAGCGTTTAAGCTCCGCCCAATGCGCTTTGTCTTAATGATGAAGACCGGTCACATATGTGACGAAGCGCTACCGGCTCGGCGCAAATTAATTATGTAGCATCCGCCTGCTATACGAATGTTCTCGGCCGCACCCATCCAGGTCGGCAGACGGACAGCGTGTCATCCGGTAGTCGAACGAAGTTAGGTGTTCCCCCTTACTGCAAAATTAATGGGGTTACTACACTGACTCATCCGTTGCAAGGATCTTCTGAAAGACAAGACGAGGCTCCATAATCGTTGATTAAATCAGTTCCCTATCAATAGACCGAACGGGTACTCACATTCTATATCGGTAGTATTTATTCCCATATCCCAAGGAAAACTGACCTAATGGATGATATTGAGCATGAGCTCAGTCGAGTTACGGTAGAGGACGACCTCACTCGATGTCGCCATTACCCTTTGGTGGGCTTGGATTACACGTGTGAGCTCCCTAACTGCACTATGCCCCGCGTACAACAATGGTCCACGCGCCCCTACAGTAACGTGCGCGCAACGCTCCTACGACCGTCAGCCACTCCCCGCGTAGACATGACCAGTGATTATACACGCGGCCCCTATAGTCACGCTAACAGTGCGGC";
    let counts = count_nucleotide(string);
    println!("{} {} {} {}", counts[0], counts[1], counts[2], counts[3]);
}

fn count_nucleotide(s: &str) -> [i32; 4] {
    let mut counts: [i32;4] = [0, 0, 0, 0];
    counts[0] = s.chars().filter(|c| *c == 'A').count() as i32;
    counts[1] = s.chars().filter(|c| *c == 'C').count() as i32;
    counts[2] = s.chars().filter(|c| *c == 'G').count() as i32;
    counts[3] = s.chars().filter(|c| *c == 'T').count() as i32;
    counts
}

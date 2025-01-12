
// For Problem 1

// Other solutions

pub fn count_nucleotide_onepass(s: &str) -> [i32; 4] {
    let mut counts: [i32; 4] = [0, 0, 0, 0]; 
    for nucleotide in s.chars() {
        match nucleotide {
	    'A' => counts[0] += 1,
	    'C' => counts[1] += 1,
            'G' => counts[2] += 1,
            'T' => counts[3] += 1,
	    _ => (),
        }
    }
    counts
}

// I wanted to write this in a way that can be easily parallelized in the future, 
// for example with a larger dataset that might begin to be more important

pub fn count_nucleotide(s: &str) -> [i32; 4] {
    let mut counts: [i32;4] = [0, 0, 0, 0];
    counts[0] = s.chars().filter(|c| *c == 'A').count() as i32;
    counts[1] = s.chars().filter(|c| *c == 'C').count() as i32;
    counts[2] = s.chars().filter(|c| *c == 'G').count() as i32;
    counts[3] = s.chars().filter(|c| *c == 'T').count() as i32;
    counts
}

// Main Code for Problem 1

//    let string = "CAAAACATTGCTGGTAAGCAAAAGAAAGGGATCCTCACGTATGATCATGACATGGGGAATATGGGGTATTCCCCGTTTTTT$
//    let counts = count_nucleotide(string);
//    println!("{} {} {} {}", counts[0], counts[1], counts[2], counts[3]);

pub fn complement(s: &str) -> String {
    let mut complement: String = String::new();
    for nucleotide in s.chars().rev() {
        match nucleotide {
	    'A' => complement.push('T'),
	    'T' => complement.push('A'),
        'C' => complement.push('G'),
        'G' => complement.push('C'),
	    _ => (),
        }
    }
    complement
}
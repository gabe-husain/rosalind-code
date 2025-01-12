use std::collections::HashMap;

// now we break through fastas...


pub fn GC(s: &str) -> HashMap<String, f32> {
    // let's split the string into sequences:

    let sequences = s.split(">")
        .filter(|s| !s.is_empty());

    let mut gc_dict: HashMap<String, f32> = HashMap::new();

    // now we need to iterate through the sequences
    for fasta in sequences {

        // first we need to find the sequence id start position
        if let Some(seq_id_end) = fasta.find("\n") {
            let seq_id: String = fasta[..seq_id_end].to_string();

            // now we collect the rest of the sequence:
            let seq = fasta[seq_id_end + 1..]
                .replace('\n', "");

            let gc_count: i32 = seq.chars()
                .filter(|&c| c == 'G' || c == 'C')
                .count() as i32;
            let gc_percent: f32 = (gc_count as f32 / seq.len() as f32) * 100.0;

            // now we need to count the G's and C's
            gc_dict.insert(seq_id, gc_percent);
        }

    }
    gc_dict
}
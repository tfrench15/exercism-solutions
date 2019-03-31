use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let valid_nucs = vec!['A', 'C', 'G', 'T'];
        let strand = DNA {
            nucleotides: String::from(dna)
        };
        for (i, nuc) in strand.nucleotides.chars().enumerate() {
            if !valid_nucs.contains(&nuc) {
                return Err(i)
            }
        }
        Ok(strand)
    }

    pub fn to_rna(self) -> RNA {
        let mut rna = RNA {
            nucleotides: String::new()
        };
        let map: HashMap<char, char> =
            [('A', 'U'),
             ('C', 'G'),
             ('G', 'C'),
             ('T', 'A')]
             .iter().cloned().collect();
        for nuc in self.nucleotides.chars() {
            rna.nucleotides.push(map[&nuc]);
        }
        rna
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let valid_nucs = vec!['A', 'C', 'G', 'U'];
        let strand = RNA {
            nucleotides: String::from(rna)
        };
        for (i, nuc) in strand.nucleotides.chars().enumerate() {
            if !valid_nucs.contains(&nuc) {
                return Err(i)
            }
        }
        Ok(strand)
    }
}



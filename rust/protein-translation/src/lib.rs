use std::collections::HashMap;
use std::borrow::Borrow;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        match self.map.get(codon) {
            None => { None },
            Some(v) => { Some(v) },
        }
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        if rna.len() == 0 || rna.len() % 3 != 0 {
            return None
        }

        let mut proteins = Vec::new();

        let owned_strand: Vec<char> = rna
            .to_owned()
            .chars()
            .collect();

        for chunk in owned_strand.chunks(3) {
            let encoding: String = chunk.iter().collect();
            match self.map.get(&encoding.borrow()) {
                None => { return None },
                Some(v) => { 
                    if *v == "stop codon" {
                        break
                    } else {
                        proteins.push(*v); 
                    }
                }
            }
        }

        Some(proteins)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut mappings = HashMap::new();

    for (codon, protein) in pairs {
        mappings.insert(codon, protein);
    }

    CodonsInfo {
        map: mappings
    }
}

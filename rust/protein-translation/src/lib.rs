use std::collections::HashMap;

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
        let mut proteins = Vec::new();
        if rna.len() % 3 != 0 {
            return None
        } else {
            let chars: Vec<char> = rna
                .chars()
                .collect();
            for chunk in chars.chunks(3) {
                match self.map.get(&chunk) {
                    None => { return None },
                    Some(v) => { 
                        proteins.push(*v);
                    },
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

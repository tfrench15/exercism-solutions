use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'static str, &'static str>
}

impl<'a> CodonsInfo<'a> {
    pub fn new() -> Self {
        let pairs = make_pairs();
        let mut mappings = HashMap::new();

        for (codon, protein) in pairs {
            mappings.insert(codon, protein);
        }

        CodonsInfo {
            map: mappings
        }
    }
    
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins = Vec::new();
        if rna.len() % 3 != 0 {
            return None
        } else {
            for chunk in rna.chars().chunks(3) {
                match self.map.get(chunk) {
                    None => { return None },
                    Some(v) => { 
                        proteins.push(v);
                    },
                }
            }
        }

        proteins
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

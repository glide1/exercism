use std::collections::BTreeMap;

pub struct ProteinInfo<'a> {
    mappings: BTreeMap<&'a str, &'a str>,
}

pub fn parse<'a>(codon_info: Vec<(&'a str, &'a str)>) -> ProteinInfo<'a> {
    ProteinInfo::new(codon_info.into_iter().fold(BTreeMap::new(), |mut btree, (codon, name)| {
        btree.insert(codon, name);
        btree
    }))
}

impl<'a> ProteinInfo<'a> {
    fn new(codon_info: BTreeMap<&'a str, &'a str>) -> Self {
        ProteinInfo { mappings: codon_info }
    }

    pub fn name_for(&self, codon: &str) -> Result<&'a str, ()> {
        let mapped_codon = codon.replace("Y", "T")
            .replace("N", "T")
            .replace("H", "T")
            .replace("M", "C")
            .replace("R", "A");
        match self.mappings.get(mapped_codon.as_str()) {
            Some(s) => Ok(s),
            None => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct RibonucleicAcid {
    strand: String,
}

impl RibonucleicAcid {
    pub fn new(s: &str) -> Self {
        RibonucleicAcid { strand: String::from(s) }
    }
}

pub struct DeoxyribonucleicAcid {
    strand: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(strand: &str) -> Self {
        DeoxyribonucleicAcid { strand: String::from(strand) }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid::new(self.strand
                                 .chars()
                                 .map(|c| dna_to_rna(&c))
                                 .collect::<String>()
                                 .as_str())
    }
}

fn dna_to_rna(a: &char) -> char {
    match *a {
        'C' => 'G',
        'G' => 'C',
        'A' => 'U',
        'T' => 'A',
        n => n,
    }
}
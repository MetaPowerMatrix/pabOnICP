struct Architect {
    pub name: String,
}

impl Architect {
    pub fn new(name: String) -> Architect {
        Architect {
            name,
        }
    }
    pub fn choose_pato() -> String {
        let pato_dirs = find_pato_dirs();
        let mut pato_dir = String::new();
        if !pato_dirs.is_empty() {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..pato_dirs.len());
            pato_dir = pato_dirs[index].clone();
        }
        pato_dir
    }
    
}

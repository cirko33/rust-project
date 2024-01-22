use std::{fs::File, io::Write};

pub fn write_factors(factors: Vec<Vec<String>>, max_w: usize, max_h: usize) {
    let file = File::create("picture.txt");
    let mut f = match file {
        Ok(file) => file,
        Err(_) => {
            panic!("Couldn't create file!");
        }
    };

    for i in 0..factors.len() {
        for j in 0..factors[i].len() {
            let s = format!("{},{},{}\n", factors[i][j], j * max_w, i * max_h);
            _ = f.write(s.as_bytes());
        }
    }
}

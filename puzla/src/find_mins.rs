use std::collections::HashSet;

use crate::factor::PictureMap;

pub fn find_mins_and_execute(factors: &mut PictureMap, grid_h: usize, grid_w: usize) -> Vec<Vec<String>> {
    let mut ret = vec![vec![String::from("empty"); grid_w]; grid_h];
    let mut used = HashSet::new();

    let mut cnt = 0;
    let goal = grid_h * grid_w;

    while cnt < goal {
        let mut min = (f64::MAX, String::from("empty"), 0, 0);
        let mut non_found = true;
        for i in 0..grid_h {
            for j in 0..grid_w {
                if factors[i][j].len() == 0 {
                    continue;
                }

                let fact = &factors[i][j][0];
                if fact.1 < min.0 && !used.contains(&fact.0) && ret[i][j].contains("empty") {
                    min = (fact.1, fact.0.clone(), i, j);
                    non_found = false;
                }
            }
        }

        if non_found {
            for i in 0..grid_h {
                for j in 0..grid_w {
                    if factors[i][j].len() == 0 {
                        continue;
                    }
    
                    factors[i][j].remove(0);
                }
            }
            continue;
        }

        ret[min.2][min.3] = min.1.clone();
        used.insert(min.1);
        factors[min.2][min.3].remove(0);
        cnt += 1;
    }
    ret
}
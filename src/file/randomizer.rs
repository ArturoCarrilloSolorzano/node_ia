use std::fmt::Debug;

use rand::{thread_rng, Rng};

pub fn no_duplicates_random<T: Clone + Debug>(data: &Vec<T>) -> Vec<T> {
    let mut res = Vec::<Option<T>>::from_iter(data.iter().map(|_| None));
    let mut rng = thread_rng();
    for value in data.iter() {
        let random_pos = rng.gen_range(0..data.len());
        let mut iter_random_pos = random_pos.clone();
        while let Some(_) = res[iter_random_pos] {
            iter_random_pos += 1;
            if iter_random_pos >= data.len() {
                iter_random_pos = 0;
            }
            if iter_random_pos == random_pos {
                panic!("error");
            }
        }
        res[iter_random_pos] = Some(value.clone());
    }
    res.iter()
        .map(move |opt| match opt {
            Some(v) => v.clone(),
            None => panic!("error"),
        })
        .collect()
}

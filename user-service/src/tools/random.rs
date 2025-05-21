use rand::{Rng, distr::Alphanumeric};

pub fn generate_random_string(length: usize) -> String {
    let rand_string: String =
        rand::rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect();
    rand_string
}

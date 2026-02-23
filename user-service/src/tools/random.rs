use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};

pub fn generate_random_string(length: usize) -> String {
    let mut rng = thread_rng(); // Crée un RNG thread-safe
    let rand_string: String =
        (&mut rng).sample_iter(&Alphanumeric).take(length).map(char::from).collect();
    rand_string
}

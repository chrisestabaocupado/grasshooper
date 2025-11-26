use rand::Rng;

pub fn get_random_color() -> (u8, u8, u8) {
    let mut rng = rand::rng();

    let tuple: (u8, u8, u8) = (
        rng.random_range(0..=255),
        rng.random_range(0..=255),
        rng.random_range(0..=255),
    );
    
    return tuple;
}

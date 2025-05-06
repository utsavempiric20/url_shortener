use nanoid::nanoid;

pub fn generate() -> String {
    nanoid!(6)
}

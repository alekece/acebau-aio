use uuid::Uuid;

pub trait UuidExt {
    fn new_fake(n: usize) -> Uuid;
}

impl UuidExt for Uuid {
    fn new_fake(n: usize) -> Uuid {
        Uuid::parse_str(&format!("00000000-0000-0000-0000-{n:0>12}")).unwrap()
    }
}

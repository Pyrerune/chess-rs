
pub trait ToPosition {
    fn to_position(&self) -> (usize, usize);
}

pub trait Coords {
    fn to_rank_file(&self) -> String;
}
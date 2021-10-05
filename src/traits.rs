pub trait ToPosition {
    fn to_position(&self) -> (usize, usize);
}

pub trait Coords {
    fn to_rank_file(&self) -> String;
}
impl ToPosition for String {
    fn to_position(&self) -> (usize, usize) {
        let xy = self.split("").collect::<Vec<&str>>();
        let mut vec = vec![];
        for x in xy {
            if !x.is_empty() {
                vec.push(x);
            }
        }
        let xy = vec;
        let x = &xy[0].to_string();
        let y = &xy[1];
        let rank: usize = (x.chars().collect::<Vec<char>>()[0] as u8 - 65) as usize;
        let file: usize = y.parse().unwrap_or(0) - 1;
        (rank, file)
    }
}

impl Coords for (usize, usize) {
    fn to_rank_file(&self) -> String {
        let rank = self.0 as u8 + 65;
        if rank > 72 {
            panic!("Invalid Coord");
        }
        format!("{}{}", rank as char, self.1 + 1)
    }
}

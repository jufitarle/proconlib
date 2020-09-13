trait VecUtil {
    fn compress(&self) -> Self;
}

impl VecUtil for Vec<char> {
    fn compress(&self) -> Self {
        let mut res = vec![self[0]];
        let mut count = 1;
        for i in 1..self.len() {
            if self[i - 1] == self[i] {
                count += 1;
            } else {
                res.append(&mut count.to_string().chars().collect());
                res.push(self[i]);
                count = 1;
            }
        }
        res.append(&mut count.to_string().chars().collect());
        res
    }
}

trait Coordinate {
    fn euclid_distance(&self, other: Self) -> f64;
    fn manhattan_distance(&self, other: Self) -> isize;
}

impl Coordinate for (isize, isize) {
    fn euclid_distance(&self, other: Self) -> f64 {
        use std::cmp::{min, max};
        let xdis = (max(self.0, other.0) - min(self.0, other.0)).abs() as f64;
        let ydis = (max(self.1, other.1) - min(self.1, other.1)).abs() as f64;
        let res = (xdis * xdis + ydis * ydis).sqrt();
        res
    }

    fn manhattan_distance(&self, other: Self) -> isize {
        (max(self.0, other.0) - min(self.0, other.0)).abs() + (max(self.1, other.1) - min(self.1, other.1)).abs()
    }
}

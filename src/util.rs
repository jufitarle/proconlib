trait VecUtil {
    fn compress(&self) -> Self;
}

impl VecUtil for Vec<char> {
    fn compress(&self) -> Self {
        let n = self.len();
        let mut res = vec![self[0]];
        let mut count = 1;
        for i in 1..n {
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

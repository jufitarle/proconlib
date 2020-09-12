

trait VecUtil {
    fn compress(&self) -> Self;
}

impl VecUtil for Vec<char> {
    fn compress(&self) -> Self {
        let n = self.len();
        let mut res = vec![];
        let mut count = 1;
        for i in 1..n {
            if self[i - 1] != self[i] {
                res.push(self[i - 1]);
                res = [res, count.to_string().chars().collect()].concat();
                count = 1;
                if i == n - 1 {
                    res.push(self[i]);
                    res.push('1');
                }
            } else {
                count += 1;
                if i == n - 1 {
                    res.push(self[i - 1]);
                    res = [res, count.to_string().chars().collect()].concat();
                }
            }
        }
        res
    }
}

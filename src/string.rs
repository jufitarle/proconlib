trait StringUtil {
    fn run_length(&self) -> String;
}

impl StringUtil for String {
    fn run_length(&self) -> String {
        let n = self.len();
        let v: Vec<char> = self.chars().collect();
        let mut res = String::new();
        let mut count = 1;
        for i in 1..n {
            if v[i - 1] != v[i] {
                res.push(v[i - 1]);
                res.push_str(count.to_string().as_str());
                count = 1;
                if i == n - 1 {
                    res.push(v[i]);
                    res.push('1');
                }
            } else {
                count += 1;
                if i == n - 1 {
                    res.push(v[i - 1]);
                    res.push_str(count.to_string().as_str());
                }
            }
        }
        res
    }
}

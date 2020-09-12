#[allow(dead_code)]
#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        (0..n).for_each(|i| parent[i] = i);
        let rank = vec![0; n];
        UnionFind { parent, rank }
    }

    fn root(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            self.parent[x] = self.root(self.parent[x]);
            self.parent[x]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn merge(&mut self, x: usize, y: usize) {
        let x = self.parent[x];
        let y = self.parent[y];

        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}

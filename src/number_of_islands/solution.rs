pub struct Solution {}

struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> DisjointSet {
        let mut parents: Vec<usize> = Vec::with_capacity(size);
        for i in 0..size {
            parents.push(i);
        }
        DisjointSet {
            parents: parents,
            ranks: vec![0; size],
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if i != self.parents[i] {
            self.parents[i] = self.find(self.parents[i])
        }

        return self.parents[i];
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let parent_i = self.find(i);
        let parent_j = self.find(j);

        if parent_i == parent_j {
            return;
        }

        if self.ranks[parent_i] > self.ranks[parent_j] {
            self.parents[parent_j] = parent_i;
        } else {
            self.parents[parent_i] = parent_j;
            if self.ranks[parent_i] == self.ranks[parent_j] {
                self.ranks[parent_j] += 1;
            }
        }
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let height = grid.len();
        if height == 0 {
            return 0;
        }
        let width = grid[0].len();
        if width == 0 {
            return 0;
        }
        let size: usize = width * height;

        let mut set = DisjointSet::new(size);

        for (row_id, row) in grid.iter().enumerate() {
            for (col_id, &col) in row.iter().enumerate() {
                let id: usize = row_id * width + col_id;
                if col == '1' {
                    if col_id > 0 && grid[row_id][col_id - 1] == '1' {
                        set.union(id, id - 1);
                    }

                    if col_id < width - 1 && grid[row_id][col_id + 1] == '1' {
                        set.union(id, id + 1);
                    }

                    if row_id > 0 && grid[row_id - 1][col_id] == '1' {
                        set.union(id, id - width);
                    }

                    if row_id < height - 1 && grid[row_id + 1][col_id] == '1' {
                        set.union(id, id + width);
                    }
                }
            }
        }
        let mut counter = 0;
        for (idx, &i) in set.parents.iter().enumerate() {
            let col_id = i % width;
            let row_id = i / width;
            if i == idx && grid[row_id][col_id] == '1' {
                counter += 1
            }
        }
        counter
    }
}

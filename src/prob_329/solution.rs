// Given an integer matrix, find the length of the longest increasing path.

// From each cell, you can either move to four directions: left, right, up or down. You may NOT move diagonally or move outside of the boundary (i.e. wrap-around is not allowed).

// Example 1:

// Input: nums =
// [
//   [9,9,4],
//   [6,6,8],
//   [2,1,1]
// ]
// Output: 4
// Explanation: The longest increasing path is [1, 2, 6, 9].

// Example 2:

// Input: nums =
// [
//   [3,4,5],
//   [3,2,6],
//   [2,2,1]
// ]
// Output: 4
// Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.

pub struct Solution {}

struct Node {
    x: usize,
    y: usize,
    children: Vec<Point>,
    is_root: bool,
}

impl Node {
    pub fn new(x: usize, y: usize) -> Node {
        Node {
            x: x,
            y: y,
            children: vec![],
            is_root: true,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let height = matrix.len();
        if height == 0 {
            return 0;
        }

        let width = matrix[0].len();
        if width == 0 {
            return 0;
        }

        let node_matrix = Self::build_node_matrix(&matrix, height, width);
        let mut post_order = Self::dfs(&node_matrix, height, width);
        let mut max_count = 0;
        for order in post_order.iter_mut() {
            order.reverse();
            let mut dist: Vec<Vec<i32>> = vec![vec![-1; width]; height];
            dist[order[0].x][order[0].y] = 0;
            for point in order {
                let node = &node_matrix[point.x][point.y];
                for next in &node.children {
                    if dist[next.x][next.y] <= dist[node.x][node.y] + 1 {
                        dist[next.x][next.y] = dist[node.x][node.y] + 1
                    }
                    if dist[next.x][next.y] >= max_count {
                        max_count = dist[next.x][next.y];
                    }
                }
            }
        }

        max_count + 1
    }

    fn build_node_matrix(matrix: &Vec<Vec<i32>>, height: usize, width: usize) -> Vec<Vec<Node>> {
        let mut node_matrix: Vec<Vec<Node>> = Vec::with_capacity(height);

        for row in 0..height {
            let mut cur_row: Vec<Node> = Vec::with_capacity(width);
            for col in 0..width {
                let value = matrix[row][col];
                let mut cur_node = Node::new(row, col);
                if col > 0 {
                    if value < matrix[row][col - 1] {
                        cur_node.children.push(Point { x: row, y: col - 1 });
                    } else if value > matrix[row][col - 1] {
                        cur_node.is_root = false;
                    }
                }

                if col < width - 1 {
                    if value < matrix[row][col + 1] {
                        cur_node.children.push(Point { x: row, y: col + 1 });
                    } else if value > matrix[row][col + 1] {
                        cur_node.is_root = false;
                    }
                }

                if row > 0 {
                    if value < matrix[row - 1][col] {
                        cur_node.children.push(Point { x: row - 1, y: col });
                    } else if value > matrix[row - 1][col] {
                        cur_node.is_root = false;
                    }
                }

                if row < height - 1 {
                    if value < matrix[row + 1][col] {
                        cur_node.children.push(Point { x: row + 1, y: col });
                    } else if value > matrix[row + 1][col] {
                        cur_node.is_root = false;
                    }
                }

                cur_row.push(cur_node);
            }

            node_matrix.push(cur_row);
        }

        node_matrix
    }

    fn dfs(node_matrix: &Vec<Vec<Node>>, height: usize, width: usize) -> Vec<Vec<Point>> {
        let mut post_orders: Vec<Vec<Point>> = Vec::new();
        for row in node_matrix {
            for node in row {
                if !node.is_root {
                    continue;
                }
                let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
                let root = Point {
                    x: node.x,
                    y: node.y,
                };
                let post_order = Self::visit(node_matrix, &mut visited, &root);

                post_orders.push(post_order);
            }
        }

        post_orders
    }

    fn visit(
        node_matrix: &Vec<Vec<Node>>,
        visited: &mut Vec<Vec<bool>>,
        point: &Point,
    ) -> Vec<Point> {
        let mut order: Vec<Point> = Vec::new();
        let cur_node = &node_matrix[point.x][point.y];
        visited[point.x][point.y] = true;
        for child in &cur_node.children {
            if visited[child.x][child.y] {
                continue;
            }
            let mut o = Self::visit(node_matrix, visited, child);
            order.append(&mut o);
        }

        order.push(Point {
            x: point.x,
            y: point.y,
        });
        order
    }
}

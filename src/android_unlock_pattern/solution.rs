pub struct Solution{}

macro_rules! set_skip_arr {
    ($skip_arr:expr, $i:expr, $j:expr, $value:expr) => {
        $skip_arr[$i][$j] = $value;
        $skip_arr[$j][$i] = $value;
    };
}

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut skip_arr: [[usize; 10]; 10] = [[0; 10]; 10];

        set_skip_arr!(skip_arr, 1, 3, 2);
        set_skip_arr!(skip_arr, 4, 6, 5);
        set_skip_arr!(skip_arr, 7, 9, 8);

        set_skip_arr!(skip_arr, 1, 7, 4);
        set_skip_arr!(skip_arr, 2, 8, 5);
        set_skip_arr!(skip_arr, 3, 9, 6);

        set_skip_arr!(skip_arr, 1, 9, 5);
        set_skip_arr!(skip_arr, 3, 7, 5);

        println!("{:?}", skip_arr);

        let mut visited = [false; 10];
        let mut num_patterns = 0;
        for i in m..(n+1) {
            num_patterns += Self::dfs(1, &mut visited, &skip_arr, i-1) * 4;
            num_patterns += Self::dfs(2, &mut visited, &skip_arr, i-1) * 4;
            num_patterns += Self::dfs(5, &mut visited, &skip_arr, i-1);
        }

        return num_patterns
    }

    fn dfs(node: i32, visited: &mut [bool; 10], skip_arr: &[[usize; 10]; 10], remaining_steps: i32 ) -> i32 {
        if remaining_steps < 0 {
            return 0
        }

        if remaining_steps == 0 {
            return 1
        }

        let u_node = node as usize;
        visited[u_node] = true;
        let mut num_patterns = 0;
        for i in 1..10 {
            if !visited[i] && (skip_arr[u_node][i] == 0 || visited[skip_arr[u_node][i]]) {
                num_patterns += Self::dfs(i as i32, visited, skip_arr, remaining_steps - 1);
            }
        }
        visited[u_node] = false;

        return num_patterns
    }
}
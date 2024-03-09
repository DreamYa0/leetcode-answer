/**
 * 51. N 皇后
困难
相关标签
相关企业
按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。

n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。

给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。

每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

 
```
示例 1：


输入：n = 4
输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
解释：如上图所示，4 皇后问题存在两个不同的解法。
示例 2：

输入：n = 1
输出：[["Q"]]
 

提示：

1 <= n <= 9
```
 */
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    let mut chessboard = vec![vec!['.'; n as usize]; n as usize];
    do_solve_n_queens(&mut chessboard, n as usize, 0, &mut res);
    res
}

fn do_solve_n_queens(chessboard: &mut Vec<Vec<char>>,n:usize, row:usize, res: &mut Vec<Vec<String>>) {
    if row == n {
        // row == n说明已经到了最后一行，收集结果
        let mut temp = Vec::new();
        for i in 0..n {
            temp.push(chessboard[i].iter().collect());
        }
        res.push(temp);
        return;
    }
    // col表示列
    for col in 0..n {
        // 检查棋盘位置是否合法
        if is_valid(chessboard, row, col, n) {
            chessboard[row][col] = 'Q';
            do_solve_n_queens(chessboard, n, row + 1, res);
            chessboard[row][col] = '.';
        }
    }
}

fn is_valid(chessboard: &Vec<Vec<char>>, row: usize, col: usize, n: usize) -> bool {
    // 检查列
    for i in 0..row {
        if chessboard[i][col] == 'Q' {
            return false;
        }
    }
    // 检查 45度角是否有皇后
    let mut i = row as i32 - 1;
    let mut j = col as i32 - 1;
    while i >= 0 && j >= 0 {
        if chessboard[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }
    // 检查 135度角是否有皇后
    let mut i = row as i32 - 1;
    let mut j = col as i32 + 1;
    while i >= 0 && j < n as i32 {
        if chessboard[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j += 1;
    }
    true
}

/**
 * 37. 解数独
困难
相关标签
相关企业
编写一个程序，通过填充空格来解决数独问题。

数独的解法需 遵循如下规则：

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）
数独部分空格内已填入了数字，空白格用 '.' 表示。



示例 1：

<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714svg.png" style="width: 258px; height: 258px;" />
```
输入：board = [
["5","3",".",".","7",".",".",".","."],
["6",".",".","1","9","5",".",".","."],
[".","9","8",".",".",".",".","6","."],
["8",".",".",".","6",".",".",".","3"],
["4",".",".","8",".","3",".",".","1"],
["7",".",".",".","2",".",".",".","6"],
[".","6",".",".",".",".","2","8","."],
[".",".",".","4","1","9",".",".","5"],
[".",".",".",".","8",".",".","7","9"]
]
输出：[
["5","3","4","6","7","8","9","1","2"],
["6","7","2","1","9","5","3","4","8"],
["1","9","8","3","4","2","5","6","7"],
["8","5","9","7","6","1","4","2","3"],
["4","2","6","8","5","3","7","9","1"],
["7","1","3","9","2","4","8","5","6"],
["9","6","1","5","3","7","2","8","4"],
["2","8","7","4","1","9","6","3","5"],
["3","4","5","2","8","6","1","7","9"]
]
解释：输入的数独如上图所示，唯一有效的解决方案如下所示：
```

<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714_solutionsvg.png" style="width: 258px; height: 258px;" />

```
提示：

board.length == 9
board[i].length == 9
board[i][j] 是一位数字或者 '.'
题目数据 保证 输入数独仅有一个解
```
 */
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    do_solve_sudoku(board);
}

fn do_solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    // 控制行
    for row in 0..board.len() {
        // 控制列
        for col in 0..board.len() {
            // 如果对应位置棋盘为空，则进行放数
            if board[row][col] == '.' {
                for val in '1'..='9' {
                    // 校验是否能够放入
                    if is_vaild(row, col, val, board) {
                        board[row][col] = val;
                        if do_solve_sudoku(board) {
                            return true;
                        }
                        // 回溯
                        board[row][col] = '.';
                    }
                }
                // 如果1-9都不能放入，说明之前的放数有问题
                return false;
            }
        }
    }
    // 说明位置已经放满了
    true
}

fn is_vaild(row: usize, col: usize, val: char, board: &Vec<Vec<char>>) -> bool {
    // 验证行，控制列不变
    for i in 0..9 {
        if board[i][col] == val {
            // 说明出现重复数
            return false;
        }
    }
    // 验证列，控制行不变
    for i in 0..9 {
        if board[row][i] == val {
            // 说明出现重复数
            return false;
        }
    }
    // 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;
    for i in start_row..(start_row + 3) {
        for j in start_col..(start_col + 3) {
            if board[i][j] == val {
                return false;
            }
        }
    }
    return true;
}

/**
 * 36. 有效的数独
中等
相关标签
相关企业
请你判断一个 9 x 9 的数独是否有效。只需要 根据以下规则 ，验证已经填入的数字是否有效即可。

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）


注意：

一个有效的数独（部分已被填充）不一定是可解的。
只需要根据以上规则，验证已经填入的数字是否有效即可。
空白格用 '.' 表示。


示例 1：

<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714svg.png" style="width: 258px; height: 258px;" />

```
输入：board =
[["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
输出：true
示例 2：

输入：board =
[["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
输出：false
解释：除了第一行的第一个数字从 5 改为 8 以外，空格内其他数字均与 示例1 相同。 但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。


提示：

board.length == 9
board[i].length == 9
board[i][j] 是一位数字（1-9）或者 '.'
```
 */
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // 控制行
    for row in 0..board.len() {
        // 控制列
        for col in 0..board.len() {
            // 如果对应位置棋盘为空，则进行放数
            if board[row][col] != '.' {
                let val = board[row][col];
                // 校验是否能够放入
                if !is_sudoku(row, col, val, &board) {
                    return false;
                }
            }
        }
    }
    // 说明位置已经放满了
    true
}

fn is_sudoku(row: usize, col: usize, val: char, board: &Vec<Vec<char>>) -> bool {
    // 验证行，控制列不变
    for i in 0..9 {
        // 需要排除掉自己
        if board[i][col] == val && i != row {
            // 说明出现重复数
            return false;
        }
    }
    // 验证列，控制行不变
    for i in 0..9 {
        // 需要排除掉自己
        if board[row][i] == val && i != col {
            // 说明出现重复数
            return false;
        }
    }
    // 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;
    for i in start_row..(start_row + 3) {
        for j in start_col..(start_col + 3) {
            // 需要排除掉自己
            if board[i][j] == val && i != row && j != col {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_n_queens() {
        let n = 4;
        let res = solve_n_queens(n);
        assert_eq!(
            res,
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
    }
    
    #[test]
    fn test_solve_sudoku() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );
    }

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = is_valid_sudoku(board);
        assert_eq!(res, true);
    }
}
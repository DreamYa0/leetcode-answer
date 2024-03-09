pub mod checkerboard;
pub mod combination;
pub mod permutation;
pub mod segmentation;
pub mod subset;

/**
 * 79. 单词搜索

给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。


```
示例 1：


输入：board = [
    ["A","B","C","E"],
    ["S","F","C","S"],
    ["A","D","E","E"]
], word = "ABCCED"
输出：true
示例 2：


输入：board = [
    ["A","B","C","E"],
    ["S","F","C","S"],
    ["A","D","E","E"]
], word = "SEE"
输出：true
示例 3：


输入：board = [
    ["A","B","C","E"],
    ["S","F","C","S"],
    ["A","D","E","E"]
], word = "ABCB"
输出：false


提示：

m == board.length
n = board[i].length
1 <= m, n <= 6
1 <= word.length <= 15
board 和 word 仅由大小写英文字母组成


进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下可以更快解决问题？
```
 */
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if do_exist(
                &mut board.clone(),
                &word.chars().collect(),
                row as i32,
                col as i32,
                0,
            ) {
                return true;
            }
        }
    }
    false
}

fn do_exist(
    board: &mut Vec<Vec<char>>,
    word: &Vec<char>,
    row: i32,
    col: i32,
    start: usize,
) -> bool {
    if row >= board.len() as i32
        || row < 0
        || col >= board[0].len() as i32
        || col < 0
        || board[row as usize][col as usize] != word[start]
    {
        return false;
    }
    if start == word.len() - 1 {
        // 说明单词已经匹配完成
        return true;
    }
    board[row as usize][col as usize] = '\0';
    let result = do_exist(board, word, row + 1, col, start + 1)
        || do_exist(board, word, row - 1, col, start + 1)
        || do_exist(board, word, row, col + 1, start + 1)
        || do_exist(board, word, row, col - 1, start + 1);
    board[row as usize][col as usize] = word[start];
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(exist(board, "ABCCED".to_string()), true);
    }
}
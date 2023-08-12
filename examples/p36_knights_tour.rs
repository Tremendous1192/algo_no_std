// き: 騎士巡歴の問題 - Knight's Tour
// 上手くいかなかったので一時停止

// 盤面のサイズ
const N: usize = 5;
//駒の移動方向
const MOVE_X: [i8; 8_usize] = [2_i8, 1, -1, -2, -2, -1, 1, 2];
const MOVE_Y: [i8; 8_usize] = [1_i8, 2, 2, 1, -1, -2, -2, -1];

// ゲーム盤
struct GameBoard {
    board: [[i8; N + 4]; N + 4],
    x: i8,
    y: i8,
    trace: Vec<usize>,
}
impl GameBoard {
    // 盤面の初期化
    pub fn init(&mut self) {
        // 壁
        for i in 0_usize..self.board.len() {
            // 上側
            self.board[0][i] = 1_i8;
            self.board[1][i] = 1_i8;
            // 下側
            self.board[N + 2][i] = 1_i8;
            self.board[N + 3][i] = 1_i8;
            // 左側
            self.board[i][0] = 1_i8;
            self.board[i][1] = 1_i8;
            // 下側
            self.board[i][N + 2] = 1_i8;
            self.board[i][N + 3] = 1_i8;
        }
    }

    // 盤面の描画
    pub fn print(&self) {
        for i in 2_usize..=(N + 1_usize) {
            for j in 2_usize..=(N + 1_usize) {
                print!("{:4}", self.board[i][j]);
            }
            println!("");
        }
    }

    // 駒を動かせるかの検証
    fn can_move(&self, start: usize) -> Result<usize, ()> {
        for i in start..MOVE_X.len() {
            if self.board[(self.x + MOVE_X[i]) as usize][(self.y + MOVE_Y[i]) as usize] == 0 {
                //println!("Ok");
                return Ok(i);
            }
        }

        Err(())
    }

    // 駒の移動
    fn move_knight(&mut self, start: usize) -> bool {
        match self.can_move(start) {
            Ok(i) => {
                self.trace.push(i);
                self.x += MOVE_X[i];
                self.y += MOVE_Y[i];
                self.board[self.x as usize][self.y as usize] = 1_i8;
                //println!("Move -> ({}, {})", self.x, self.y);
                true
            }
            Err(_) => {
                self.back_knight();
                false
            }
        }
    }

    // 移動先が無くなったときに駒を戻す
    fn back_knight(&mut self) -> bool {
        println!("Back");
        // 移動方向
        let i: usize = self.trace[self.trace.len() - 1_usize];
        self.trace.remove(self.trace.len() - 1_usize);

        // 現在の値を未通過に戻す
        self.board[self.x as usize][self.y as usize] = 0_i8;
        self.x -= MOVE_X[i];
        self.y -= MOVE_Y[i];

        if i == MOVE_X.len() - 1_usize {
            self.back_knight()
        } else {
            self.move_knight(i + 1_usize)
        }
    }
}

fn main() {
    // 盤面インスタンス
    let mut game_board: GameBoard = GameBoard {
        board: [[0_i8; N + 4]; N + 4],
        x: 2_i8,
        y: 2_i8,
        trace: Vec::<usize>::new(),
    };
    // 初期化
    game_board.init();
    // 最初の盤面
    game_board.print();

    println!("");
    while game_board.move_knight(0_usize) {
        game_board.print();
        println!("");
    }
    game_board.print();
}

// 以下は愚直に書き直したコード
// 配列への不正なアクセスが発生してエラーになった
// 原因不明
/*
const N: usize = 3;
static mut board: [[i8; N + 4]; N + 4] = [[0_i8; N + 4]; N + 4];
static dx: [i8; 8_usize] = [2_i8, 1, -1, -2, -2, -1, 1, 2];
static dy: [i8; 8_usize] = [1_i8, 2, 2, 1, -1, -2, -2, -1];
static mut count: usize = 0_usize;

/// 盤面を出力
fn print_board() {
    let mut i: usize;
    let mut j: usize;

    unsafe {
        for i in 2_usize..=(N + 1_usize) {
            for j in 2_usize..=(N + 1_usize) {
                print!("{:4}", board[i][j]);
            }
            println!("");
        }
    }
}

fn try_game(x: usize, y: usize) {
    let i: i8;

    unsafe {
        println!("{}手目 -> 座標[{}, {}] = {}", count, x, y, board[x][y]);

        if board[x][y] != 0_i8 {
            return;
        }
        count += 1_usize;
        board[x][y] += count as i8;
        if count >= N * N {
            println!("count up");
            print_board();
        } else {
            for i in 0_usize..dx.len() {
                try_game((x as i8 + dx[i]) as usize, (y as i8 + dy[i]) as usize);
            }
        }
        board[x][y] = 0_i8;
        count -= 1;
    }
}

/// き: 騎士巡歴の問題 - Knight's Tour
fn main() {
    let i: usize;
    let j: usize;

    unsafe {
        for i in 0_usize..=(N + 3_usize) {
            for j in 0_usize..(N + 3_usize) {
                board[i][j] = 1_i8;
            }
        }
        for i in 2_usize..=(N + 1_usize) {
            for j in 2_usize..=(N + 1_usize) {
                board[i][j] = 0_i8;
            }
        }
    }
    print_board();
    try_game(2_usize, 2_usize);
    print_board();
}
*/

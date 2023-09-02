const { invoke } = window.__TAURI__.tauri;

const boardSize = 8;
let board = [];
let turn = 0; 
const cells = document.getElementById('othello-board');

const black = 1;
const white = 2;



async function drawBoard() {
    board = await invoke("handle_board");   
  cells.innerHTML = '';
  for (let y = 0; y < boardSize; y++) {
      const tr = document.createElement('tr');
      for (let x = 0; x < boardSize; x++) {
          const td = document.createElement('td');
          if (board[y][x] === white) {
              td.classList.add('white');
          } else if (board[y][x] === black) {
              td.classList.add('black');
          }
          td.addEventListener('click', () => handleCellClick(x, y));
          tr.appendChild(td);
      }
      cells.appendChild(tr);
  }
}

async function handleCellClick(x, y) {
    await invoke("put_piece_handle", {y: y, x: x});   
    drawBoard();
}
// ゲームの初期化
async function initializeGame() {
    await invoke("init_board", {});   
}



async function main() {
    await initializeGame();
    drawBoard();
}

main();

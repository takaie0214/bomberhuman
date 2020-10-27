const canvas = document.getElementById("bomberhuman-canvas");
canvas.width=750
canvas.height=650
const ctx = canvas.getContext('2d');

const resources = () => {
  let res = {
    player: document.createElement('canvas'),
    bomb: document.createElement('canvas'),
    wall: document.createElement('canvas'),
  }

  // Player
  res.player.width = 50;
  res.player.height = 50;
  let pCtx = res.player.getContext('2d');
  pCtx.fillStyle = "yellow";
  pCtx.beginPath();
  pCtx.arc(25, 25, 25, 0, 2 * Math.PI);
  pCtx.fill();

  // Bomb
  res.bomb.width = 50;
  res.bomb.height = 50;
  let bCtx = res.bomb.getContext('2d');
  bCtx.fillStyle = "green";
  bCtx.beginPath();
  bCtx.arc(25, 25, 25, 0, 2 * Math.PI);
  bCtx.fill();

  res.wall.width = 50;
  res.wall.height = 50;
  let wCtx = res.wall.getContext('2d');
  wCtx.fillStyle = "red";
  wCtx.beginPath();
  wCtx.rect(0, 0, 50,50);
  wCtx.fill();
  return res;
};

const res = resources();

export function draw_player(x, y) {
    ctx.drawImage(res.player, x - 10, y - 10);
    ctx.fillStyle = "black";
}

export function draw_bomb(x, y) {
    ctx.drawImage(res.bomb, x - 10, y - 10);
    ctx.fillStyle = "black";
}

export function draw_wall(x, y) {
    ctx.drawImage(res.wall, x , y);
    ctx.fillStyle = "black";
}

export function clear_screen() {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}


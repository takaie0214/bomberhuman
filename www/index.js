import { GameState } from "bomberhuman";

const gamestate = GameState.new(200, 200);

const canvas = document.getElementById("bomberhuman-canvas");
const ctx = canvas.getContext('2d');

const resources = () => {
  let res = {
    player: document.createElement('canvas')
  }

  // Player
  res.player.width = 20;
  res.player.height = 20;
  let eCtx = res.player.getContext('2d');
  eCtx.fillStyle = "yellow";
  eCtx.beginPath();
  eCtx.arc(10, 10, 10, 0, 2 * Math.PI);
  eCtx.fill();

  return res;
};

const draw_player = (x, y) => {
      ctx.drawImage(res.player, x - 10, y - 10);

      ctx.fillStyle = "black";
};

const clear_screen = () => {
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
};


document.addEventListener('keydown', e => gamestate.processKey(e.key, true));
document.addEventListener('keyup', e => gamestate.processKey(e.key, false));

function resize() {
  // We make the canvas somewhat smaller to get some zooming
  canvas.width = window.innerWidth * 0.8;
  canvas.height = window.innerHeight * 0.8;
}
window.addEventListener('resize', () => {
  resize();
});

const draw = () => {
    clear_screen();

    let x = gamestate.get_objx();
    let y = gamestate.get_objy();
    draw_player(x, y);
};

var start = null;
var prevTimestamp = null;

const renderLoop = (timestamp) => {
    // Initialization
    if (!prevTimestamp) {
      start = timestamp;
      prevTimestamp = timestamp;
      requestAnimationFrame(renderLoop);
      return;
    }

    // Update and draw
    let progress = (timestamp - prevTimestamp) / 1000;
    gamestate.update(progress); 
    draw(); 

    // Some bookkeeping
    prevTimestamp = timestamp;
    requestAnimationFrame(renderLoop);
};

const res = resources();
resize(); 
renderLoop();


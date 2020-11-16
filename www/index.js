import { GameState } from "bomberhuman";

const gamestate = GameState.new(600, 300);

document.addEventListener('keydown', e => gamestate.processkey(e.key, true));
document.addEventListener('keyup', e => gamestate.processkey(e.key, false));


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
    gamestate.draw();

    // Some bookkeeping
    prevTimestamp = timestamp;
    requestAnimationFrame(renderLoop);
};

renderLoop();

import { GameState, KeyType} from "bomberhuman";

const gamestate = GameState.new(600, 300);

var controllers = {};

window.addEventListener("gamepadconnected", function(e) {
  console.log("Gamepad connected at index %d: %s. %d buttons, %d axes.",
    e.gamepad.index, e.gamepad.id,
    e.gamepad.buttons.length, e.gamepad.axes.length);
    controllers[e.gamepad.index] = e.gamepad;
});

window.addEventListener("gamepaddisconnected", function(e) {
  console.log("Gamepad disconnected from index %d: %s",
    e.gamepad.index, e.gamepad.id);
    delete controllers[e.gamepad.index];
});


function scangamepads() {
  var gamepads = navigator.getGamepads ? navigator.getGamepads() : (navigator.webkitGetGamepads ? navigator.webkitGetGamepads() : []);
  for (var i = 0; i < gamepads.length; i++) {
      if (gamepads[i] && (gamepads[i].index in controllers)) {
          controllers[gamepads[i].index] = gamepads[i];
      }
  }
  for (var j in controllers) {
    var controller = controllers[j];

    if(controller.buttons.length < 15) {
      console.log(controller.axes[1])
      gamestate.processcontroller(j, KeyType.Up      ,controller.axes[1] < -0.5);
      gamestate.processcontroller(j, KeyType.Down    ,controller.axes[1] > 0.5);
      gamestate.processcontroller(j, KeyType.Right   ,controller.axes[0] > 0.5);
      gamestate.processcontroller(j, KeyType.Left    ,controller.axes[0] < -0.5);
      gamestate.processcontroller(j, KeyType.Button1 ,controller.buttons[0].pressed ? true : false);
    } else {
      gamestate.processcontroller(j, KeyType.Up      ,controller.buttons[12].pressed ? true : false);
      gamestate.processcontroller(j, KeyType.Down    ,controller.buttons[13].pressed ? true : false);
      gamestate.processcontroller(j, KeyType.Left    ,controller.buttons[14].pressed ? true : false);
      gamestate.processcontroller(j, KeyType.Right   ,controller.buttons[15].pressed ? true : false);
      gamestate.processcontroller(j, KeyType.Button1 ,controller.buttons[1].pressed ? true : false);
    }
  }
}

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

    scangamepads();

    // Update and draw
    let progress = (timestamp - prevTimestamp) / 1000;
    gamestate.update(progress);
    gamestate.draw();

    // Some bookkeeping
    prevTimestamp = timestamp;
    requestAnimationFrame(renderLoop);
};

document.addEventListener('keydown', e => gamestate.processkey(e.key, true));
document.addEventListener('keyup', e => gamestate.processkey(e.key, false));

renderLoop();

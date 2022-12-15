import { Game } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const game = Game.new(164, 48);

const renderLoop = () => {
  pre.textContent = game.render();
  game.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

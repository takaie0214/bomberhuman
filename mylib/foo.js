export function name() {
    return 'Rust';
}

export function clear_screen() {
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
}

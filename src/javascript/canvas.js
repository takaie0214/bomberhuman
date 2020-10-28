const canvas = document.getElementById("bomberhuman-canvas");
canvas.width=750
canvas.height=650
const ctx = canvas.getContext('2d');


class Rectangle{
    constructor(x,y,width,height){
        this.x=x;
        this.y=y;
        this.width=width;
        this.height=height;
    }
}

let image_list = [
    {id:"player1", src: "image/akane.png", reg: {x: 48, y: 48}},
    {id:"player2", src: "image/aoi.png",   reg: {x: 48, y: 48}},
    {id:"player3", src: "image/yukari.png",reg: {x: 48, y: 48}},
    {id:"player4", src: "image/maki.png",  reg: {x: 48, y: 48}},
    {id:"bomb"   , src: "image/bomb.png",  reg: {x: 416, y: 416}},
    {id:"wall"   , src: "image/wall.png",reg: {x: 602, y: 602}},
    {id:"sblock" , src: "image/sblock.png",reg: {x: 317, y: 317}},

];


class Sprite{
    constructor(){
        this.images = new Array();
        var i =0;
        for (i=0; i<image_list.length; i++) {
            this.images[i] = {id: image_list[i].id, image: new Image(), reg:image_list[i].reg}
            this.images[i].image.src = image_list[i].src
        }

    }

    draw(id,recx, recy, x, y) {
        var index = 0;
        for (let [i, val]  of this.images.entries()){
            if (val.id == id){
                index = i;
                break;
            }
        }
        var img = this.images[index];
        ctx.drawImage(img.image, 
            img.reg.x * recx, img.reg.y * recy, 
            img.reg.x, img.reg.y,
            x, y, 
            50, 50);
    }
}

//let sprite = new Sprite();
//let  sprite = new Sprite('image/akane.png', new Rectangle(48, 0, 48, 48))
let  sprite = new Sprite();


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

  // Wall  
  res.wall.width = 50;
  res.wall.height = 50;
  let wCtx = res.wall.getContext('2d');
  wCtx.fillStyle = "blue";
  wCtx.beginPath();
  wCtx.rect(0, 0, 50,50);
  wCtx.fill();
  return res;
};

const res = resources();

export function draw_player(x, y) {
    ctx.drawImage(res.player, x - 25, y - 25);
    ctx.fillStyle = "black";
}

export function draw_bomb(recx, recy, x, y) {
    //ctx.drawImage(res.bomb, x - 25, y - 25);
    //ctx.fillStyle = "black";
    sprite.draw("bomb", recx, recy, x-25, y-25);
}

export function draw_wall(x, y) {
    //ctx.drawImage(res.wall, x-25 , y-25);
    //ctx.fillStyle = "black";
    sprite.draw("wall", 0, 0, x-25, y-25);
}

export function draw_sblock(x, y) {
    sprite.draw("sblock", 0, 0, x-25, y-25);
}

export function clear_screen() {
    ctx.fillStyle = "green";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

export function draw_player_animation(id, recx, recy, x, y){
    sprite.draw(id, recx, recy, x-25, y-25);
}

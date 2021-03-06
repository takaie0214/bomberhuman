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
    {id:"player1", src: "image/1012010501.png", reg: {x: 49, y: 49}},
    {id:"player2", src: "image/1087010501.png", reg: {x: 49, y: 49}},
    {id:"player3", src: "image/1104010501.png", reg: {x: 49, y: 49}},
    {id:"player4", src: "image/1114010501.png", reg: {x: 49, y: 49}},
    {id:"bomb"   , src: "image/bomb.png",       reg: {x: 416, y: 416}},
    {id:"wall"   , src: "image/wall.png",       reg: {x: 602, y: 602}},
    {id:"block"  , src: "image/sblock.png",     reg: {x: 317, y: 317}},
    {id:"fire"   , src: "image/fire.png",       reg: {x:177, y:177}},
    {id:"item_fire"   , src: "image/item_fire.png",       reg: {x:800,y:800}},
    {id:"item_bomb"   , src: "image/item_bomb.png",       reg: {x:800,y:800}},
    {id:"item_boots"   , src: "image/item_boots.png",       reg: {x:400,y:400}},

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

export function draw_block(x, y) {
    sprite.draw("block", 0, 0, x-25, y-25);
}

export function draw_fire(recx, recy, x, y) {
    sprite.draw("fire", recx, recy, x-25, y-25);
}

export function draw_item_fire(x,y) {
    sprite.draw("item_fire", 0, 0, x-25, y-25);
}

export function draw_item_bomb(x,y) {
    sprite.draw("item_bomb", 0, 0, x-25, y-25);
}

export function draw_item_boots(x,y) {
    sprite.draw("item_boots", 0, 0, x-25, y-25);
}

export function clear_screen() {
    ctx.fillStyle = "green";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

export function draw_player_animation(id, recx, recy, x, y){
    sprite.draw(id, recx, recy, x-25, y-25);
}

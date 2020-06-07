const canvas = document.getElementById('shigaiCanvas');
const ctx = canvas.getContext('2d');
const square = 5;

document.addEventListener("mousemove", mouseMoveHandler, false);

let pressed = false;
canvas.onmousedown = function () {
    pressed = true;
};

canvas.onmouseup = function () {
    pressed = false;
}

function mouseMoveHandler(e) {
    var relX = e.clientX - canvas.offsetLeft
    var relY = e.clientY - canvas.offsetTop

    if (pressed) {
        ctx.fillStyle = "#" + ((Math.pow(16, 6) + relX * 100 + relY * 100) % Math.pow(16, 6)).toString(16);
        ctx.fillRect(relX - square / 2, relY - square / 2, square, square);

    }

}
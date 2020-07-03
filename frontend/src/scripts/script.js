const canvas = document.getElementById('shigaiCanvas');
const ctx = canvas.getContext('2d');
let mouseX = 0;
let mouseY = 0;

document.addEventListener("mousemove", mouseMoveHandler, false);
let pressed = false;

canvas.onmousedown = function (e) {
    mouseX = e.clientX - canvas.offsetLeft
    mouseY = e.clientY - canvas.offsetTop
    pressed = true;
};

document.onmouseup = function () {
    pressed = false;
}

function mouseMoveHandler(e) {
    var relX = e.clientX - canvas.offsetLeft
    var relY = e.clientY - canvas.offsetTop

    if (pressed) {
        ctx.beginPath();
        ctx.moveTo(mouseX, mouseY);
        ctx.lineCap = 'round';
        ctx.lineWidth = 2;
        ctx.lineTo(relX , relY);
        ctx.stroke();
        mouseX = relX;
        mouseY = relY;
    }

}
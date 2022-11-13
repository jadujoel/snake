import { getElementById } from "./utils/dom";

interface Board {
    width: number
    height: number
    cellSize: number
}

interface DrawContext extends Board {
    ctx: CanvasRenderingContext2D
}

export function getCanvas() {
    const canvas = getElementById<HTMLCanvasElement>("snake-canvas");
    if (!canvas) {
        throw new Error("Could not get canvas");
    }
    return canvas;
}

export function getContext(canvas: HTMLCanvasElement) {
    const context = canvas.getContext("2d");
    if (!context) {
        throw new Error("Could not get context");
    }
    return context;
}

export function drawBoard({ctx, width, height, cellSize}: DrawContext) {
    ctx.beginPath()
    ctx.fillStyle = "white";
    ctx.strokeStyle = "white";
    ctx.lineWidth = 2;
    for (let x = 0; x <= width; x++) {
        ctx.moveTo(cellSize * x, 0)
        ctx.lineTo(cellSize * x, cellSize * width)
    }
    for (let y = 0; y <= height; y++) {
        ctx.moveTo(0, cellSize * y)
        ctx.lineTo(cellSize * height, cellSize * y)
    }
    ctx.stroke()
}

export function drawSnake({ctx, width, height, cellSize}: DrawContext, {head, body}: { head: number, body: number[] }) {
    const drawCell = (cell: number, index: number) => {
        ctx.fillStyle = index === 0 ? "#3b20d5" : "#a14393";
        // ctx.fillStyle = getColor(index)
        const col = cell % width;
        const row = Math.floor(cell / width);
        ctx.beginPath()
        ctx.fillRect(
            col * cellSize,
            row * cellSize,
            cellSize -1,
            cellSize -1
        )
    }
    body.forEach(drawCell)
    ctx.stroke()
}

export function drawReward({ctx, width, height, cellSize}: DrawContext, reward: number) {
    const col = reward % width;
    const row = Math.floor(reward / width);
    ctx.fillStyle = "red";
    ctx.beginPath()
    ctx.fillRect(
        col * cellSize,
        row * cellSize,
        cellSize,
        cellSize
    )
    ctx.stroke()
}

import init, { World } from "snake";
import { controller } from "./controller";
import { drawBoard, drawReward, drawSnake, getCanvas, getContext } from "./draw";
import { getRandomInt } from "./utils/random";


async function main() {
    await init();
    const fps = 2 as const
    const ms = 1000 / fps
    const width = 4 as const
    const cellSize = 800 / width
    const height = width

    const startIndex = getRandomInt(0, width * height - 1)
    const world = World.new(width, height, startIndex);

    const canvas = getCanvas();
    const ctx = getContext(canvas)
    canvas.width = width * cellSize;
    canvas.height = height * cellSize;

    const drawContext = {
        ctx,
        width,
        height,
        cellSize: cellSize
    } as const

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawBoard(drawContext)
        drawSnake(drawContext, {
            head: world.snake_head_index(),
            body: Array.from(world.snake_body())
        })
        const reward = world.reward_cell()
        if (reward !== undefined) {
            drawReward(drawContext, reward)
        }
    }

    function update() {
        setTimeout(() => {
            world.step();
            draw();
            requestAnimationFrame(update)
            controller.update(world)
        }, ms)
    }

    draw();
    update();

    controller.init(world)
}

main()

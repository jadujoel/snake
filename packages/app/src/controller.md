use game::{ Direction, GameStatus, World };
import { getElementById, unwrap } from "./utils/dom";

```ts
export const controller = {
    info: getElementById("snake-info"),
    pointsElement: getElementById("snake-score-output"),
    init: (world: World) => {
        document.addEventListener("keydown", (e) => {
            switch (e.key) {
                case "ArrowUp":
                    world.set_direction(Direction.Up);
                    break;
                case "ArrowRight":
                    world.set_direction(Direction.Right);
                    break;
                case "ArrowDown":
                    world.set_direction(Direction.Down);
                    break;
                case "ArrowLeft":
                    world.set_direction(Direction.Left);
                    break;
                case " ": {
                    const match = {
                        [GameStatus.Paused]: () => {
                            hideWin()
                            world.start_game()
                            controller.info.className = "hide"
                        },
                        [GameStatus.Played]: () => {
                            hideWin()
                            world.pause_game()
                            controller.info.className = "show"
                        },
                        [GameStatus.Lost]: () => {
                            hideWin()
                            controller.info.className = "hide"
                            world.restart()
                            world.start_game()
                        },
                        [GameStatus.Won]: () => {
                            controller.info.className = "show"
                            world.restart()
                        },
                    } as const
                    unwrap(match[world.game_status()])()
                }
            }
        })
    },
    update: (world: World) => {
        if (world.game_status() === GameStatus.Lost) {
            showInfo()
        }
        else if (world.game_status() === GameStatus.Won) {
            showInfo()
            showWin()
        }
        else {
            hideWin()
        }
        controller.pointsElement.innerText = (world.snake_body_len() - 3).toString()
    }
}

function showInfo() {
    getElementById("snake-info").className = "show"
}

function showWin() {
    getElementById("snake-win").className = "show"
}

function hideWin() {
    getElementById("snake-win").className = "hide"
}

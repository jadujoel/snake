use crate::board::SnakeCanvas;
use crate::game::{Direction, GameStatus};
use crate::{game::World, utils::random};
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_interval;

#[function_component(App)]
pub fn app() -> Html {
    let width = 16;
    let height = 16;
    let size = (width * height) - 1;
    let start_index = random(0, size);

    let fps = 12;

    let world = use_mut_ref(|| World::new(width, height, start_index));

    let millis = use_state(|| 1000 / fps);
    let reward = use_state(|| world.borrow().reward_cell());
    let status = use_state(|| world.borrow().game_status());
    let body = use_state(|| world.borrow().snake_body());

    // update the game each interval
    {
        let body = body.clone();
        let reward = reward.clone();
        let world = world.clone();
        let status = status.clone();

        use_interval(
            move || {
                let mut world = world.borrow_mut();
                status.set(world.game_status());
                if !(world.game_status() == GameStatus::Running) {
                    return;
                }
                world.step();
                body.set(world.snake_body());
                reward.set(world.reward_cell());
            },
            *millis,
        );
    }

    // controller
    {
        let world = world.clone();
        use_effect(move || {
            // Attach a keydown event listener to the document.
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "keydown", move |event| {
                let event = match event.dyn_ref::<web_sys::KeyboardEvent>() {
                    Some(event) => event,
                    None => return,
                };
                let mut world = world.borrow_mut();
                match event.key().as_str() {
                    "ArrowUp" => {
                        world.set_direction(Direction::Up);
                    }
                    "ArrowDown" => {
                        world.set_direction(Direction::Down);
                    }
                    "ArrowLeft" => {
                        world.set_direction(Direction::Left);
                    }
                    "ArrowRight" => {
                        world.set_direction(Direction::Right);
                    }
                    " " => match world.game_status() {
                        GameStatus::Paused => {
                            world.start_game();
                        }
                        GameStatus::Running => {
                            world.pause_game();
                        }
                        GameStatus::Won => {
                            world.restart();
                        }
                        GameStatus::Lost => {
                            world.restart();
                        }
                    },
                    _ => {}
                };
            });
            // Called when the component is unmounted.  The closure has to hold on to `listener`, because if it gets
            // dropped, `gloo` detaches it from the DOM. So it's important to do _something_, even if it's just dropping it.
            || drop(listener)
        });
    }

    html! (
        <div class="game">
        {
            match *status {
                GameStatus::Paused => {
                    html! {
                        <div class="info">
                            <h1>{ "Snake" }</h1>
                            <p>{ "Use the arrow keys to move the snake around." }</p>
                            <p>{ "Eat the food to grow longer." }</p>
                            <p>{ "Don't run into yourself or the walls." }</p>
                            <p>{ "Press space to start." }</p>
                        </div>
                    }
                }
                GameStatus::Won  => {
                    html! {
                        <div class="info">
                            <h1>{ "You won!" }</h1>
                            <p>{ "Press space to restart." }</p>
                        </div>
                    }
                }
                GameStatus::Lost => {
                    html! {
                        <div class="info">
                            <h1>{ "You lost!" }</h1>
                            <p>{ "Press space to restart." }</p>
                        </div>
                    }
                }
                _ => {
                    html!(
                        <SnakeCanvas
                        height={height as u32}
                        width={width as u32}
                        reward={*reward}
                        body={body.to_vec()}
                        />
                    )
                }
            }
        }
        </div>
    )
}

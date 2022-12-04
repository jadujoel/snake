use crate::board::SnakeCanvas;
use crate::game::{Direction, GameStatus, World};
use crate::utils::random;

use web_sys::KeyboardEvent;
use yew::{function_component, html, use_mut_ref, use_state};
use yew_hooks::{use_event_with_window, use_interval};

#[function_component(App)]
pub fn app() -> Html {
    let width = 8;
    let height = 8;
    let size = (width * height) - 1;
    let start_index = random(0, size);

    let bpm = 128;
    let division = 2;
    let seconds = 60.0 / bpm as f32 / division as f32;

    let world = use_mut_ref(|| World::new(width, height, start_index));
    let is_started = use_mut_ref(|| false);

    let millis = use_state(|| (seconds * 1000.0) as u32);
    let reward = use_state(|| world.borrow().reward_cell());
    let status = use_state(|| world.borrow().game_status());
    let body = use_state(|| world.borrow().snake_body());

    {
        let world = world.clone();
        let is_started = is_started.clone();
        use_event_with_window("keydown", move |event: KeyboardEvent| {
            let mut world = world.borrow_mut();
            let mut is_started = is_started.borrow_mut();
            if !*is_started {
                world.start_audio();
                *is_started = true;
                return;
            }
            match event.key().as_str() {
                "ArrowUp" => world.set_direction(Direction::Up),
                "ArrowDown" => world.set_direction(Direction::Down),
                "ArrowLeft" => world.set_direction(Direction::Left),
                "ArrowRight" => world.set_direction(Direction::Right),
                " " => match world.game_status() {
                    GameStatus::Paused => world.resume_game(),
                    GameStatus::Running => world.pause_game(),
                    GameStatus::Won => world.restart(),
                    GameStatus::Lost => world.restart(),
                },
                _ => {}
            }
        });
    }

    // update the game each interval
    {
        let body = body.clone();
        let reward = reward.clone();
        // let world = world.clone();
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

    html! (
        <div class="game">
        {
            match *is_started.borrow() {
                false => {
                    html! {
                        <div class="info">
                            <h1>{"Press any key to start"}</h1>
                        </div>
                    }
                },
                true => {
                    match *status {
                        GameStatus::Paused => {
                            html! {
                                <div class="info">
                                    <h1>{ "Snake" }</h1>
                                    <p>{ "Use the arrow keys to move the snake around." }</p>
                                    <p>{ "Eat the food to grow longer." }</p>
                                    <p>{ "Don't run into yourself." }</p>
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
            }

        }
        </div>
    )
}

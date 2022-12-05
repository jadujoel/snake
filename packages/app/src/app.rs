use crate::game::{Direction, Status, World};
use crate::render::SnakeCanvas;
use crate::utils::{random_usize, u32_from_usize, u32_from_f64};

use web_sys::KeyboardEvent;
use yew::{function_component, html, use_mut_ref, use_state};
use yew_hooks::{use_event_with_window, use_interval};

#[function_component(App)]
pub fn app() -> Html {
    let width = 8;
    let height = 8;
    let size = width * height - 1;
    let start_index = random_usize(0, size);

    let bpm = 128;
    let division = 2;
    let seconds = 60.0 / f64::from(bpm) / f64::from(division);

    let world = use_mut_ref(|| World::new(width, height, start_index));
    let is_started = use_mut_ref(|| false);

    let millis = use_state(|| u32_from_f64(seconds * 1000.0));
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
                    Status::Paused => world.resume_game(),
                    Status::Running => world.pause_game(),
                    Status::Won | Status::Lost => world.restart(),
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
                if !(world.game_status() == Status::Running) {
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
            if *is_started.borrow() {
                match *status {
                    Status::Paused => {
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
                    Status::Won  => {
                        html! {
                            <div class="info">
                                <h1>{ "You won!" }</h1>
                                <p>{ "Press space to restart." }</p>
                            </div>
                        }
                    }
                    Status::Lost => {
                        html! {
                            <div class="info">
                                <h1>{ "You lost!" }</h1>
                                <p>{ "Press space to restart." }</p>
                            </div>
                        }
                    }
                    Status::Running => {
                        html!(
                            <SnakeCanvas
                                height={u32_from_usize(height)}
                                width={u32_from_usize(width)}
                                reward={*reward}
                                body={body.to_vec()}
                            />
                        )
                    }
                }
            } else {
                html! {
                    <div class="info">
                        <h1>{"Press any key to start"}</h1>
                    </div>
                }
            }
        }
        </div>
    )
}

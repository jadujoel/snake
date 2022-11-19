use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_node_ref, use_state, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub width: u32,
    pub height: u32,
    pub reward: Option<usize>,
    pub body: Vec<usize>,
}

#[function_component(SnakeCanvas)]
pub fn gui(props: &Props) -> Html {
    // let Props { width, height, reward, body } = props.clone();
    let Props {
        width,
        height,
        reward,
        body,
    } = props.clone();

    let cell_size = use_state(|| 800 / width);
    let node_ref = use_node_ref();

    let canvas_width = width * *cell_size;
    let canvas_height = height * *cell_size;

    fn draw_board(
        ctx: &web_sys::CanvasRenderingContext2d,
        width: u32,
        height: u32,
        cell_size: u32,
    ) {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("white"));
        ctx.set_line_width(1.0);
        for x in 0..width + 1 {
            ctx.move_to((cell_size * x) as f64, 0.0);
            ctx.line_to((cell_size * x) as f64, (cell_size * width) as f64);
        }
        for y in 0..height + 1 {
            ctx.move_to(0.0, (cell_size * y) as f64);
            ctx.line_to((cell_size * height) as f64, (cell_size * y) as f64);
        }
        ctx.stroke()
    }

    fn draw_snake(
        ctx: &web_sys::CanvasRenderingContext2d,
        width: u32,
        cell_size: u32,
        body: &Vec<usize>,
    ) {
        let mut index = 0;
        for cell in body {
            let color = if index == 0 { "#3b20d5" } else { "#a14393" };
            ctx.set_fill_style(&JsValue::from_str(color));
            let col = cell % width as usize;
            let row = cell / width as usize;
            ctx.begin_path();
            ctx.fill_rect(
                (col * cell_size as usize) as f64,
                (row * cell_size as usize) as f64,
                (cell_size - 1) as f64,
                (cell_size - 1) as f64,
            );
            index += 1;
        }
        ctx.stroke();
    }

    fn draw_reward(
        ctx: &web_sys::CanvasRenderingContext2d,
        width: u32,
        cell_size: u32,
        reward: u32,
    ) {
        let col = reward % width;
        let row = reward / width;
        ctx.set_fill_style(&JsValue::from_str("red"));
        ctx.begin_path();
        ctx.fill_rect(
            (col * cell_size) as f64,
            (row * cell_size) as f64,
            cell_size as f64,
            cell_size as f64,
        );
        ctx.stroke();
    }

    let draw = move |canvas: &HtmlCanvasElement| {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        draw_board(&context, width.clone(), height.clone(), *cell_size);
        match reward {
            Some(reward) => {
                draw_reward(&context, width.clone(), *cell_size, reward as u32);
            }
            None => {}
        }
        draw_snake(&context, width.clone(), *cell_size, &body);
        // Perform the cleanup
        || {}
    };

    match node_ref.cast::<HtmlCanvasElement>() {
        Some(canvas) => {
            canvas.set_width(canvas_width);
            canvas.set_height(canvas_height);
            draw(&canvas)();
        }
        None => {}
    };

    html!(
        <canvas
            id="canvas"
            ref={node_ref}
            width={canvas_width.to_string()}
            height={canvas_height.to_string()}
        />
    )
}

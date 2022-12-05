use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_node_ref, Properties};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub width: u32,
    pub height: u32,
    pub reward: Option<usize>,
    pub body: Vec<usize>,
}

#[allow(clippy::cast_possible_truncation, clippy::as_conversions, clippy::cast_precision_loss)]
#[function_component(SnakeCanvas)]
pub fn snake_canvas(props: &Props) -> Html {
    // let Props { width, height, reward, body } = props.clone();
    let Props {
        width,
        height,
        reward,
        body,
    } = props.clone();

    let cell_size = 800 / width;
    let node_ref = use_node_ref();

    let canvas_width = width * cell_size;
    let canvas_height = height * cell_size;

    let draw = move |canvas: &HtmlCanvasElement| {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        draw_board(&context, width, height, cell_size);
        if let Some(reward) = reward {
            #[allow(clippy::cast_precision_loss)]
            draw_reward(&context, f64::from(width), f64::from(cell_size), reward as f64);
        }
        draw_snake(&context, f64::from(width), f64::from(cell_size), &body);
        // Perform the cleanup
        || {}
    };

    if let Some(ref canvas) = node_ref.cast::<HtmlCanvasElement>() {
        canvas.set_width(canvas_width);
        canvas.set_height(canvas_height);
        draw(canvas)();
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


fn draw_snake(
    ctx: &web_sys::CanvasRenderingContext2d,
    width: f64,
    cell_size: f64,
    body: &[usize],
) {
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions, clippy::cast_precision_loss, clippy::cast_sign_loss)]
    for (index, cell) in body.iter().enumerate() {
        let color = if index == 0 { "#3b20d5" } else { "#a14393" };
        ctx.set_fill_style(&JsValue::from_str(color));
        let col = cell % width as usize;
        let row = cell / width as usize;
        ctx.begin_path();
        let x = (col * cell_size as usize) as f64;
        let y = (row * cell_size as usize) as f64;
        let w = cell_size - 1.0;
        let h = cell_size - 1.0;
        ctx.fill_rect(x, y, w, h);
    }
    ctx.stroke();
}


fn draw_board(
    ctx: &web_sys::CanvasRenderingContext2d,
    width: u32,
    height: u32,
    cell_size: u32,
) {
    ctx.begin_path();
    ctx.set_fill_style(&JsValue::from_str("white"));
    ctx.set_line_width(1.0);
    for col in 0..=width {
        let y_start = 0.0;
        let y_end = f64::from(cell_size * width);
        let x = f64::from(cell_size * col);
        ctx.move_to(x, y_start);
        ctx.line_to(x, y_end);
    }
    for row in 0..=height {
        let x_start = 0.0;
        let x_end = f64::from(cell_size * height);
        let y = f64::from(cell_size * row);
        ctx.move_to(x_start, y);
        ctx.line_to(x_end, y);
    }
    ctx.stroke();
}

fn draw_reward(
    ctx: &web_sys::CanvasRenderingContext2d,
    width: f64,
    cell_size: f64,
    reward: f64,
) {
    let col = reward % width;
    let row = reward / width;
    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.begin_path();
    let x = (col * cell_size) as f64;
    let y = (row * cell_size) as f64;
    let w = cell_size as f64;
    let h = cell_size as f64;
    ctx.fill_rect(x, y, w, h);
    ctx.stroke();
}

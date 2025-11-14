use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod state;
use state::State;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mandelbrot Set Renderer")
        .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(State::new(window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, window_id }
            if window_id == state.window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(*new_inner_size);
                }

                WindowEvent::CursorMoved {device_id, position, ..} => {
                    state.cursor_position = Some(position);
                }

                WindowEvent::MouseInput {device_id, state:a, button, ..} => {
                    if button == MouseButton::Left && a == ElementState::Pressed {
                        if let Some(pos) = state.cursor_position {
                            let width = state.size.width as f64;
                            let height = state.size.height as f64;
                            let norm_x = (pos.x / width) - 0.5;
                            let norm_y = (pos.y / height) - 0.5;
                            let c_real = state.view_params.center[0] as f64 + norm_x * state.view_params.range[0] as f64;
                            let c_imag = state.view_params.center[1] as f64 + norm_y * state.view_params.range[1] as f64;
                            state.view_params.center = [c_real as f32, c_imag as f32];
                            state.view_params.range[0] *= 0.5;
                            state.view_params.range[1] *= 0.5;
                            state.trigger_render(false);
                        }
                    }
                }

                _ => {}
            },

            Event::RedrawRequested(window_id) if window_id == state.window.id() => {
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                state.window.request_redraw();
            }

            _ => {}
        }
    });
}
use crevice::std140::AsStd140;

use std::path;
use std::sync::Arc;

use ggez::event::{self};
use ggez::graphics::{self, ImageFormat, Sampler};
use ggez::{Context, ContextBuilder, GameResult};
use glam::Vec2;

use led_matrix_zmq::server::{MatrixMessage, ThreadedMatrixServerHandle};

const WGSL_SHADER: &str = include_str!("matrix.wgsl");

#[derive(AsStd140)]
struct Dim {
    width: f32,
    height: f32,
}

struct MainState {
    frame: Option<graphics::Image>,
    dim: Dim,
    shader: graphics::Shader,
    params: graphics::ShaderParams<Dim>,
    opts: ViewerOpts,
    zmq_handle: Arc<ThreadedMatrixServerHandle>,
}

impl MainState {
    fn new(
        opts: ViewerOpts,
        zmq_handle: Arc<ThreadedMatrixServerHandle>,
        ctx: &mut Context,
    ) -> GameResult<MainState> {
        let dim = Dim {
            width: zmq_handle.settings.width as f32,
            height: zmq_handle.settings.height as f32,
        };
        let shader = graphics::ShaderBuilder::from_code(WGSL_SHADER).build(&ctx.gfx)?;
        let params = graphics::ShaderParamsBuilder::new(&dim).build(ctx);

        let s = MainState {
            frame: None,
            opts,
            zmq_handle,
            shader,
            params,
            dim,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let zmq_msg = match self.zmq_handle.rx.try_recv() {
            Ok(m) => m,
            Err(_) => return Ok(()),
        };
        match zmq_msg {
            MatrixMessage::Frame(frame) => {
                let rgba = frame
                    .chunks(3)
                    .flat_map(|chunk| [chunk[0], chunk[1], chunk[2], 255])
                    .collect::<Vec<_>>();

                let img = graphics::Image::from_pixels(
                    _ctx,
                    &rgba,
                    ImageFormat::Rgba8UnormSrgb,
                    self.zmq_handle.settings.width as u32,
                    self.zmq_handle.settings.height as u32,
                );
                self.frame = Some(img);
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        if let Some(frame) = self.frame.as_ref() {
            let screen_coords = canvas.screen_coordinates().unwrap();
            let screen_center = Vec2::new(screen_coords.w / 2.0, screen_coords.h / 2.0);

            let largest_frame_dim = self
                .zmq_handle
                .settings
                .width
                .max(self.zmq_handle.settings.height) as f32;
            let largest_screen_dim = screen_coords.w.max(screen_coords.h) as f32;
            let scale = largest_screen_dim / largest_frame_dim;

            let draw_param = graphics::DrawParam::new()
                .dest(screen_center)
                .scale(Vec2::new(scale, scale))
                .offset(Vec2::new(0.5, 0.5));

            {
                canvas.set_sampler(Sampler::nearest_clamp());
                self.params.set_uniforms(ctx, &self.dim);
                canvas.set_shader(&self.shader);
                canvas.set_shader_params(&self.params);
                canvas.draw(frame, draw_param);
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub struct ViewerOpts {
    pub scale: f32,
}

pub fn run(opts: ViewerOpts, zmq_handle: Arc<ThreadedMatrixServerHandle>) {
    let (mut ctx, event_loop) = ContextBuilder::new("Matrix Viewer", "M.H.")
        .window_setup(ggez::conf::WindowSetup::default().title("Matrix Viewer").srgb(true).vsync(true))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            zmq_handle.settings.width as f32 * opts.scale,
            zmq_handle.settings.height as f32 * opts.scale,
        ))
        .build()
        .unwrap();

    let state = MainState::new(opts, zmq_handle, &mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}

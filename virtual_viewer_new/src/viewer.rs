use std::sync::Arc;

use gfx::{self, *};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, ImageFormat};
use ggez::{Context, ContextBuilder, GameResult};
use glam::Vec2;

use led_matrix_zmq::server::{MatrixMessage, ThreadedMatrixServerHandle};

struct MainState {
    frame: Option<graphics::Image>,
    // matrix_shader: graphics::Shader<MatrixPixelShader>,
    opts: ViewerOpts,
    zmq_handle: Arc<ThreadedMatrixServerHandle>,
    pos_x: f32,
}

impl MainState {
    fn new(
        opts: ViewerOpts,
        zmq_handle: Arc<ThreadedMatrixServerHandle>,
        ctx: &mut Context,
    ) -> GameResult<MainState> {
        // let mps: MatrixPixelShader = MatrixPixelShader {
        //     width: zmq_handle.settings.width as f32,
        //     height: zmq_handle.settings.height as f32,
        // };

        let s = MainState {
            pos_x: 0.0,
            frame: None,
            opts,
            zmq_handle,
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

                // // let mut img = graphics::Image::from_rgba8(
                //     ctx,
                //     self.zmq_handle.settings.width as u16,
                //     self.zmq_handle.settings.height as u16,
                //     &rgba,
                // )
                // .unwrap();
                // let mut img = graphics::Image::from_pixels(
                //     _ctx,
                //     &rgba,
                //     ImageFormat::Rgba8Uint,
                //     self.zmq_handle.settings.width as u32,
                //     self.zmq_handle.settings.height as u32,
                // );
                // img2.
                // img2.set_filter(graphics::FilterMode::Nearest);

                // self.frame = Some(img);
            }
            _ => (),
        }

        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        canvas.draw(&circle, Vec2::new(self.pos_x, 380.0));
        // if let Some(frame) = self.frame.as_ref() {
        //     let screen_coords = canvas.screen_coordinates().unwrap();
        //     let screen_center = Vec2::new(screen_coords.w / 2.0, screen_coords.h / 2.0);

        //     let largest_frame_dim = self
        //         .zmq_handle
        //         .settings
        //         .width
        //         .max(self.zmq_handle.settings.height) as f32;
        //     let largest_screen_dim = screen_coords.w.max(screen_coords.h) as f32;
        //     let scale = largest_screen_dim / largest_frame_dim;

        //     let draw_param = graphics::DrawParam::new()
        //         .dest(screen_center)
        //         .scale(Vec2::new(scale, scale))
        //         .offset(Vec2::new(0.5, 0.5));

        //     {
        //         // let _lock = graphics::use_shader(ctx, &self.matrix_shader);
        //         // graphics::draw(ctx, frame, draw_param).unwrap();
        //         // canvas.draw(frame, draw_param);
        //     }
        // }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub struct ViewerOpts {
    pub scale: f32,
}

pub fn run(opts: ViewerOpts, zmq_handle: Arc<ThreadedMatrixServerHandle>) {
    let (mut ctx, event_loop) = ContextBuilder::new("Matrix Viewer", "")
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            zmq_handle.settings.width as f32 * opts.scale,
            zmq_handle.settings.height as f32 * opts.scale,
        ))
        .build()
        .unwrap();

    let state = MainState::new(opts, zmq_handle, &mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}

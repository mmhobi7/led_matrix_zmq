use clap::{App, Arg};

use led_matrix_zmq::server::{MatrixMessage, MatrixServer, MatrixServerSettings};
use rpi_led_matrix::{LedColor, LedMatrix};

const COL_SIZE: u32 = 64;
const ROW_SIZE: u32 = 32;

fn main() {
    let app = App::new("led_matrix_zmq").arg(
        Arg::with_name("bind_address")
            .long("bind-address")
            .takes_value(true)
            .default_value("tcp://*:42024"),
    );

    let app = rpi_led_matrix::args::add_matrix_args(app);
    let matches = app.get_matches();
    let (mut matrix_opts, mut matrix_rt_opts) =
        rpi_led_matrix::args::matrix_options_from_args(&matches);

    matrix_opts.set_cols(COL_SIZE);
    matrix_opts.set_rows(ROW_SIZE);
    matrix_opts.set_refresh_rate(false);
    matrix_opts.set_parallel(3);
    // matrix_opts.set_pwm_dither_bits(0);
    matrix_opts.set_pwm_lsb_nanoseconds(200);
    matrix_opts.set_brightness(100);
    matrix_opts.set_limit_refresh(0);
   // matrix_opts.set_limit_refresh(120);

    matrix_rt_opts.set_daemon(true);
    matrix_rt_opts.set_drop_privileges(true);
    matrix_rt_opts.set_gpio_slowdown(1);

    let matrix = LedMatrix::new(Some(matrix_opts), Some(matrix_rt_opts)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    let matrix_server_settings = MatrixServerSettings {
        bind_address: matches.value_of("bind_address").unwrap().to_string(),
        width: COL_SIZE,
        height: ROW_SIZE,
    };

    let matrix_server = MatrixServer::new(&matrix_server_settings);

    loop {
        let msg = matrix_server.recv();
        match msg {
            MatrixMessage::Frame(frame) => {
                for y in 0..ROW_SIZE {
                    for x in 0..COL_SIZE {
                        let i = (y * COL_SIZE + x) * 3;
                        let r = frame[i];
                        let g = frame[i + 1];
                        let b = frame[i + 2];
                        canvas.set(
                            x as i32,
                            y as i32,
                            &LedColor {
                                red: r,
                                green: g,
                                blue: b,
                            },
                        );
                    }
                }

                canvas = matrix.swap(canvas);
            }
            _ => {}
        }
    }
}

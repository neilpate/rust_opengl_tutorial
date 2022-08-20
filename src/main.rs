extern crate sdl2;

fn main() {
    println!("Hello, world!");

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Hi", 900, 700)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
            for event in event_pump.poll_iter(){
               //Handle user input here
                match event {
                    sdl2::event::Event::Quit { timestamp } => {print!("Program ran for {} ms", timestamp); break 'main},
                    _ => {},
                }     

            }

            //Render window contents here
        }

}

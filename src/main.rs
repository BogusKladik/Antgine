extern crate antgine;

use std::{
    mem,
    thread::{sleep, self},
    time::{Duration, Instant}, sync::mpsc, net::TcpListener, io::Write,
};

use antgine::physics_engine::{
    map::Map, objects::rectangle::Rectangle, traits::object_interface::ObjectInterface,
    types::vec2d::Vec2D,
};
use serde_json::{Value, json};

enum Program {
    Start,
    _Working,
    _Exit,
}

fn main() {
    let (tx_ready, rx_ready) = mpsc::channel();
    let (tx_data, rx_data) = mpsc::channel();

    let mut handles = Vec::with_capacity(2);
    handles.push(thread::spawn(move || {
        let _program = Program::Start;
    let mut map = Map::default();
    map.init_map_border(Vec2D::new(0.0, 0.0), Vec2D::new(1920.0, 1080.0));
    // map.test_rectangle();
    let a = Rectangle::default();
    let a = Rectangle::new(
        Vec2D::new(1910.0, 0.0),
        Vec2D::new(1910.0, 20.0),
        20.0,
        10.0,
        1.0,
        Vec2D::new(-100.0, 0.0),
        0.01,
        0.0,
        0.01,
    );
    // let b = Rectangle::new(
    //     Vec2D::new(1900.0, 0.0),
    //     Vec2D::new(1900.0, 20.0),
    //     20.0,
    //     10.0,
    //     1.0,
    //     Vec2D::new(-100.0, 0.0),
    //     0.01,
    //     0.0,
    //     0.01,
    // );
    println!("{:?} {:?}", (&a).get_current_position(), (&a).get_size());
    println!("{}", mem::size_of_val(&a));
    map.dyn_objects.push(Box::new(a));
    // map.dyn_objects.push(Box::new(b));

    let mut ready = false;
    loop {
        let now = Instant::now();
        sleep(Duration::from_millis(10));
        map.run(Instant::now().duration_since(now).as_secs_f32());

        if let Ok(answer) = rx_ready.try_recv() {
            ready = answer;
        }

        if ready {
            let (pos, ang) = map.create_data();
            let pos: Vec<(f32, f32)> = pos.into_iter().map(|vec2d| (vec2d.x, vec2d.y)).collect();
            let ang: Vec<f32> = ang.into_iter().map(|angle| angle.get_radian()).collect();
            
            let json_obj: Value = json!({
                "Ant": {
                    "pos": pos,
                    "angle": ang,
                }
            });

            println!("{}", &json_obj);

            tx_data.send(json_obj).unwrap();
        } else {
            // println!("data not received");
            // thread::sleep(Duration::from_secs(1));
        }
    }
    }));

    handles.push(thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:6142").expect("Failed to bind address");
        println!("Server listening on port 6142");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("Connected to client");
                    tx_ready.send(true).unwrap();
                    
                    loop {
                        let data = rx_data.recv().unwrap();

                        // thread::sleep(Duration::from_millis(100));

                        if let Err(e) = stream.write_all(data.to_string().as_bytes()) {
                            tx_ready.send(false).unwrap();
                            eprintln!("failed to write to socket; err = {:?}", e);
                            break;
                        }
                    }

                    println!("Closed to client");
                }
                Err(_) => tx_ready.send(false).unwrap()
            }
        }
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}

use nannou::{prelude::*, noise::{Perlin, NoiseFn}};
use nannou_egui::{self, egui, Egui};
use std::fs;
use std::io::ErrorKind;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing{
    positions: Vec<Vec2>
}
impl Thing {
    pub fn new (p:Vec2) -> Self{
        let mut positions = Vec::new();
        positions.push(p);
        Thing{
            positions,
        }
    }
}
struct Model {
    ui: Egui,
    main_window: WindowId,
   things:Vec<Thing>,
   noise: Perlin,
   frames_dir: String,
cur_frame: u32,
recording: bool,
}

const N_THINGS: usize = 2000;

fn model(app: &App) -> Model {
    let main_window = app.new_window().size(1024, 1024).view(view).build().unwrap();
    let mut things = Vec:: new();

    for i in 0 .. N_THINGS{
    let thing = Thing::new(vec2(
        (random::<f32>()-0.5)*1024.0,
        (random::<f32>()-0.5)*1024.0
        ));
    things.push(thing);}

    let ui_window = app.new_window()
                .title(app.exe_name().unwrap() + " controls")
                .size(280, 130)
                .view(ui_view)
                .raw_event(raw_ui_event)
                .key_pressed(key_pressed)
                .build()
                .unwrap();

                let ui_window_ref = app.window(ui_window).unwrap();
                let ui = Egui::from_window(&ui_window_ref);


    let noise = Perlin::new();

    let frames_dir = app.exe_name().unwrap() + "_frames";
    let recording = false;
    let cur_frame = 0;
    

    Model { 
        ui,
        main_window,
        things,
        noise, 
        frames_dir,
        recording,
        cur_frame,
    }
    
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 120.0;
    let sn = 0.01 + time.cos() as f64 * 0.005;
    update_ui(model);
for thing in model.things.iter_mut(){ 

    thing.positions.clear();
    thing.positions.push(vec2(
        (random::<f32>()-0.5)*1024.0,
        (random::<f32>()-0.5)*1024.0));

    for k in 0 .. 50{
    let last = thing.positions[0];
    let new = last + vec2( 
        model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
        model.noise.get([sn*last.x as f64,sn*last.y as f64, 1.0]) as f32,
    );
thing.positions.insert(0, new);}}

if model.recording && app.elapsed_frames() % 2 == 0 {
    model.cur_frame += 1;
    if model.cur_frame > 1000 {
        model.recording = false;
    } else {
        let filename = format!("{}/universe{:>04}.png",
            model.frames_dir,
            model.cur_frame);
        match app.window(model.main_window) {
            Some(window) => {
                window.capture_frame(filename);
            }
            None => {}
        }
}
}
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

   
    if app.elapsed_frames() == 1{
    draw.background().color(BLACK);
    }
for thing in model.things.iter(){
    draw.polyline().points(thing.positions.iter().cloned()).color(WHITE);
   
}
draw.rect().w_h(1024.0, 1024.0).color(srgba(0.0,0.0,0.0,0.1));

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {

        Key::R => {
            if model.recording {
                model.recording = false;
            } else {
                fs::create_dir(&model.frames_dir).unwrap_or_else(|error| {
                    if error.kind() != ErrorKind::AlreadyExists {
                        panic!{"Problem creating directory {:?}", model.frames_dir};
                    }
                });
                model.recording = true;
                model.cur_frame = 0;
            }
        }
      
        Key::S => {
            match app.window(model.main_window) {
                Some(window) => {
                    window.capture_frame(app.exe_name().unwrap() + ".png");
                }
                None => {}
            }
        }
        
        
        _other_key => {}
    }

}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Universe Control Panel").collapsible(false).show(&ctx, |ui| {
     
    });
}
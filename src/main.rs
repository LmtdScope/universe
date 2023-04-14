use nannou::{prelude::*};

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing{
    position: Vec2
}
impl Thing {
    pub fn new (p:Vec2) -> Self{
        Thing{
            position:p,
        }
    }
}
struct Model {
   things:Vec<Thing>,
}

const N_THINGS: usize = 2000;

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024, 1024).view(view).build().unwrap();
    let mut things = Vec:: new();

    for i in 0 .. N_THINGS{
    let thing = Thing::new(vec2(
        (random::<f32>()-0.5)*1024.0,
        (random::<f32>()-0.5)*1024.0,
        ));
    things.push(thing);}

    Model { things, }
    
}

fn update(_app: &App, model: &mut Model, _update: Update) {

for thing in model.things.iter_mut(){ 
    thing.position += vec2(1.0,0.0);
     }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let time = app.elapsed_frames() as f32 / 60.0;

    draw.background().color(BLACK);
 
for thing in model.things.iter(){
    draw.ellipse().xy(thing.position).radius(5.0).color(WHITE);
}

    draw.to_frame(app, &frame).unwrap();
}
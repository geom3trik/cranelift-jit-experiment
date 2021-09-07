

use tuix::*;

use super::node_widget::*;
use super::socket_widget::*;

pub struct NodeView {
    translate_x: f32,
    translate_y: f32,
    scale: f64,

    prev_translate_x: f32,
    prev_translate_y: f32,
    panning: bool,

    canvas: Entity,
}

impl NodeView {
    pub fn new() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale: 1.0,

            prev_translate_x: 0.0,
            prev_translate_y: 0.0,
            panning: false,

            canvas: Entity::null(),
        }
    }
}

impl Widget for NodeView {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.canvas = Element::new().build(state, entity, |builder| 
            builder
                .set_clip_widget(entity)
                .set_hoverable(false)
            //.set_background_color(Color::rgb(50,50,200))
        );



        state.set_focus(entity);


        self.canvas
    }
    
    
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(window_event) = event.message.downcast() {
            match window_event {

                
                WindowEvent::MouseDown(button) => {
                    //if event.target == entity {
                        if *button == MouseButton::Middle {
                            self.panning = true;
                            state.capture(entity);
                            self.prev_translate_x = self.translate_x;
                            self.prev_translate_y = self.translate_y;
                        }
                    //}
                }

                WindowEvent::MouseUp(button) => {
                    if event.target == entity {
                        if *button == MouseButton::Middle {
                            self.panning = false;
                            state.release(entity);
                        }
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    // When middle mouse button is pressed, pan the canvas when mouse is moved
                    if self.panning {
                        let dx = *x - state.mouse.middle.pos_down.0;
                        let dy = *y - state.mouse.middle.pos_down.1;

                        self.translate_x = self.prev_translate_x + dx;
                        self.translate_y = self.prev_translate_y + dy;
                        //println!("x: {}, y: {}", self.translate_x, self.translate_y);
                        self.canvas.set_translate(state, (self.translate_x, self.translate_y));
                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                    }

                }

                WindowEvent::MouseScroll(x,y) => {
                    self.scale += 0.1 * *y as f64;
                    if self.scale >= 2.0 {
                        self.scale = 2.0;
                    }

                    if self.scale <= 0.5 {
                        self.scale = 0.5;
                    }



                    self.canvas.set_scale(state, self.scale as f32);
                    //println!("scale: {}", self.scale);
                }

                WindowEvent::KeyDown(code, key) => {
                    println!("Key: {:?} {:?}", code, key);
                    match *code {


                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
    
}
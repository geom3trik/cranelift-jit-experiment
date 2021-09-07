
pub mod node_view;
use std::{collections::HashMap, ops::Index};

pub use node_view::*;

pub mod node_widget;
pub use node_widget::*;

pub mod socket_widget;
pub use socket_widget::*;

use tuix::*;

pub mod graph;
pub use graph::*;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeEvent {
    TrySnap(Entity, Entity),
    ConnectSockets(Entity),
    ConnectInput,
    ConnectOutput,
    //Disconnect(Entity),
    Snap(Entity, Entity),
    Connecting,
    Disconnect,
}

#[derive(PartialEq, Clone)]
pub struct NodeDesc {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(PartialEq)]
pub enum AppEvent {
    AddNode(NodeDesc),
    InsertNode(String),
}

// Where everything lives
// TODO - Rename me
pub struct NodeApp {
    graph: IndexGraph,
    node_view: Entity,
    menu: Entity,
    node_descriptions: HashMap<String, NodeDesc>,
}

impl NodeApp {
    pub fn new() -> Self {
        Self {
            graph: IndexGraph::new(),
            node_view: Entity::null(),
            menu: Entity::null(),
            node_descriptions: HashMap::new(),
        }
    }
}

impl Widget for NodeApp {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let popup = Popup::new()
            .build(state, entity, |builder| {
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Auto)
                    .set_z_order(10)
            });

        self.menu = List::new()
            .build(state, popup, |builder| {
                builder
                    .set_height(Auto)
            });

        self.node_view = NodeView::new().build(state, entity, |builder| {
            builder
        });

        self.node_view
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::AddNode(node) => {

                    let node_name = node.name.clone();

                    self.node_descriptions.insert(node.name.clone(), node.clone());

                    // Add a button to the menu from the node description

                    Button::with_label(&node.name)
                        .on_release(move |_, state, button| {
                            button.emit(state, AppEvent::InsertNode(node_name.clone()));
                            button.emit(state, PopupEvent::Close);
                        })
                        .build(state, self.menu, |builder| 
                            builder
                    );
                }

                AppEvent::InsertNode(name) => {
                    if let Some(node_desc) = self.node_descriptions.get(name) {

                        // Create the node from the description

                        let node = NodeWidget::new(&name).build(state, self.node_view, |builder| 
                            builder
                        );
            
                        for param in node_desc.inputs.iter() {
            
                            let row = Row::new().build(state, node, |builder| 
                                builder
                                    .set_height(Pixels(30.0))
                                    .set_child_space(Stretch(1.0))
                            );
                        
                            InputSocket::new().build(state, row, |builder| 
                                builder
                                    .set_left(Pixels(-10.0))
                                    .set_right(Stretch(0.0))
                            );
                        
                            Label::new(&param).build(state, row, |builder| 
                                builder
                                    .set_child_space(Stretch(1.0))
                                    .set_child_left(Pixels(5.0))
                                    .set_space(Pixels(0.0))
                                    .set_hoverable(false)
                            );
                        }
            
                        for ret in node_desc.outputs.iter() {
                            let row = Row::new().build(state, node, |builder| 
                                    builder
                                        .set_height(Pixels(30.0))
                                        .set_child_space(Stretch(1.0))
                                );
                    
                            Label::new(&ret).build(state, row, |builder| 
                                builder
                                    .set_child_space(Stretch(1.0))
                                    .set_child_right(Pixels(5.0))
                                    .set_space(Pixels(0.0))
                                    .set_hoverable(false)
                            );
                    
                            OutputSocket::new().build(state, row, |builder| 
                                builder
                                    .set_left(Stretch(0.0))
                                    .set_right(Pixels(-10.0))
                            );                
                        }
                    }
                }

                _=> {}
            }
        }

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseUp(button) if *button == MouseButton::Right => {
                    entity.emit_to(state, self.menu, PopupEvent::OpenAtCursor);
                }

                _=> {}
            }
        }
    }
}
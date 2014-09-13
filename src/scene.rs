
use std::collections::hashmap::HashMap;
use uuid::Uuid;

use piston::Event;

use graphics::*;

use event::{
    Behavior,
    State,
    Running,
};

use Sprite;

use action::{
    Action,
    ActionState,
    EmptyState,
};

pub struct Scene<I: ImageSize> {
    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, uint>,
    running: HashMap<Uuid, Vec<(State<Action>, ActionState)>>,
}

impl<I: ImageSize> Scene<I> {
    pub fn new() -> Scene<I> {
        Scene {
            children: Vec::new(),
            children_index: HashMap::new(),
            running: HashMap::new(),
        }
    }

    pub fn update(&mut self, e: &Event) {
        // regenerate the actions and their states
        let running = self.running.clone();
        self.running = HashMap::new();

        for (id, actions) in running.move_iter() {
            let mut new_actions = Vec::new();

            for (mut a, mut s) in actions.move_iter() {
                let sprite = self.child_mut(id).unwrap();
                let (status, _) = a.update(e, |dt, action| {
                    let state = match s {
                        EmptyState => action.to_state(sprite),
                        _ => s,
                    };
                    let (state, status, remain) = state.update(sprite, dt);
                    s = state;
                    (status, remain)
                });

                match status {
                    // the behavior is still running, add it for next update
                    Running => {
                        new_actions.push((a.clone(), s));
                    },
                    _ => {},
                }
            }

            if new_actions.len() > 0 {
                self.running.insert(id, new_actions);
            }
        }
    }

    pub fn draw<B: BackEnd<I>>(&self, c: &Context, b: &mut B) {
        for child in self.children.iter() {
            child.draw(c, b);
        }
    }

    pub fn run_action(&mut self, sprite_id: Uuid, action: Behavior<Action>) {
        let actions = self.running.find_or_insert_with(sprite_id, |_| Vec::new());
        actions.push((State::new(action), EmptyState));
    }

    pub fn add_child(&mut self, sprite: Sprite<I>) -> Uuid {
        let id = sprite.id();
        self.children.push(sprite);
        self.children_index.insert(id, self.children.len() - 1);
        id
    }

    pub fn child(&self, id: Uuid) -> Option<&Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(&self.children[*i]) },
            None => {
                for child in self.children.iter() {
                    match child.child(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }

    pub fn child_mut(&mut self, id: Uuid) -> Option<&mut Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(self.children.get_mut(*i)) },
            None => {
                for child in self.children.mut_iter() {
                    match child.child_mut(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }
}


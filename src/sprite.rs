
use std::rc::Rc;
use std::collections::hashmap::HashMap;
use uuid::Uuid;

use graphics::*;
use graphics::internal::{
    Vec2d,
    Rectangle,
};
use graphics::vecmath::Scalar;

pub struct Sprite<I: ImageSize> {
    id: Uuid,

    /// Normalized
    anchor: Vec2d,

    position: Vec2d,
    /// In degree
    rotation: Scalar,
    scale: Vec2d,

    flip_x: bool,
    flip_y: bool,

    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, uint>,

    texture: Rc<I>,
}

impl<I: ImageSize> Sprite<I> {
    pub fn from_texture(texture: Rc<I>) -> Sprite<I> {
        Sprite {
            id: Uuid::new_v4(),

            anchor: [0.5, 0.5],

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],

            flip_x: false,
            flip_y: false,

            texture: texture,

            children: Vec::new(),
            children_index: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn id(&self) -> Uuid {
        self.id
    }

    #[inline(always)]
    pub fn anchor(&self) -> (Scalar, Scalar) {
        (self.anchor[0], self.anchor[1])
    }

    #[inline(always)]
    pub fn set_anchor(&mut self, x: Scalar, y: Scalar) {
        self.anchor = [x, y];
    }

    #[inline(always)]
    pub fn position(&self) -> (Scalar, Scalar) {
        (self.position[0], self.position[1])
    }

    #[inline(always)]
    pub fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.position = [x, y];
    }

    #[inline(always)]
    pub fn rotation(&self) -> Scalar {
        self.rotation
    }

    #[inline(always)]
    pub fn set_rotation(&mut self, deg: Scalar) {
        self.rotation = deg;
    }

    #[inline(always)]
    pub fn flip_x(&self) -> bool {
        self.flip_x
    }

    #[inline(always)]
    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }

    #[inline(always)]
    pub fn flip_y(&self) -> bool {
        self.flip_y
    }

    #[inline(always)]
    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }

    #[inline(always)]
    pub fn texture(&self) -> &Rc<I> {
        &self.texture
    }

    #[inline(always)]
    pub fn set_texture(&mut self, texture: Rc<I>) {
        self.texture = texture;
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

    pub fn draw<B: BackEnd<I>>(&self, c: &Context, b: &mut B) {
        let (w, h) = self.texture.get_size();
        let w = w as f64;
        let h = h as f64;
        let anchor = [self.anchor[0] * w, self.anchor[1] * h];

        let transformed = c.trans(self.position[0], self.position[1])
                           .rot_deg(self.rotation)
                           .scale(self.scale[0], self.scale[1]);

        let mut model = transformed.rect(-anchor[0],
                                         -anchor[1],
                                          w,
                                          h);

        if self.flip_x {
            model = model.trans(w - 2.0 * anchor[0], 0.0).flip_h();
        }

        if self.flip_y {
            model = model.trans(0.0, h - 2.0 * anchor[1]).flip_v();
        }

        // for debug: bounding_box
        //model.rgb(1.0, 0.0, 0.0).draw(b);

        model.image(&*self.texture).draw(b);

        // for debug: anchor point
        //c.trans(self.position[0], self.position[1]).rect(-5.0, -5.0, 10.0, 10.0).rgb(0.0, 0.0, 1.0).draw(b);

        for child in self.children.iter() {
            child.draw(&transformed, b);
        }
    }

    pub fn bounding_box(&self) -> Rectangle {
        let (w, h) = self.texture.get_size();
        let w = w as f64 * self.scale[0];
        let h = h as f64 * self.scale[1];

        [
            self.position[0] - self.anchor[0] * w,
            self.position[1] - self.anchor[1] * h,
            w,
            h
        ]
    }
}


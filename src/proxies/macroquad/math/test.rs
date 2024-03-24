pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}



impl Into<Rect> for Rect {
    fn into(self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}

impl<'a> Into<&'a Rect> for &Rect {
    fn into(self) -> &'a Rect {
        &Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}

impl<'a> Into<&'a mut Rect> for &mut Rect {
    fn into(self) -> &'a mut Rect {
        &mut Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}

impl Rect {
    pub fn point(&self) -> Vec2{
        let parent: &Rect = Rect::from(self);
    }
}
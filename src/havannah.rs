#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Havannah{
    pub x:isize,
    pub y:isize
}

impl Havannah{
    pub(crate) fn add(&self, b:Havannah )->Havannah{
        Havannah{x:self.x+b.x, y:self.y+b.y}
    }

    pub(crate) fn fused_multiply_add(&self, direction:Havannah, steps:isize) ->Havannah{
        Havannah{x:self.x+ direction.x*steps, y:self.y+direction.y*steps}
    }

    pub(crate) fn is_bounded(&self, between:isize)->bool{
        self.x<between && self.y<between && self.y>=0 && self.x>=0
    }
}

pub const NORTHEAST:Havannah    =Havannah{x:-1,y:0};
pub const NORTHWEST:Havannah    =Havannah{x:-1,y:-1};
pub const SOUTHEAST:Havannah    =Havannah{x:1,y:1};
pub const SOUTHWEST:Havannah    =Havannah{x:1,y:0};
pub const EAST:Havannah         =Havannah{x:0,y:1};
pub const WEST:Havannah         =Havannah{x:0,y:-1};
pub const NEIGHBOURS:[&Havannah;6] =[&NORTHEAST, &NORTHWEST, &SOUTHEAST, &SOUTHWEST, &EAST, &WEST];
pub const NEIGHBOUR_OPP:[[&Havannah;2];3]=[[&EAST, &WEST], [&SOUTHWEST, &NORTHEAST], [&SOUTHEAST, &NORTHWEST]];
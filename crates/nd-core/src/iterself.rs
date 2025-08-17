use crate::direction::Direction;

pub trait CircularIterSelf {
    fn all() -> Vec<Self>
    where
        Self: Sized + PartialEq + Clone;

    fn movecursor(&mut self, direction: Direction)
    where
        Self: Sized + PartialEq + Clone,
    {
        let all = Self::all();
        let mut index = all.iter().position(|i| i == self).unwrap_or(0);

        direction.apply_usize(&mut index, all.len());
        *self = all[index].clone();
    }
}

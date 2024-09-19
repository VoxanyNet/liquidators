pub struct VecRemoveIter<'a, T> {
    index: usize,
    pub vec: &'a mut Vec<T>
}

impl<'a, T> VecRemoveIter<'a, T> {

    pub fn next(&mut self) -> Option<VecRemoveIterItem<T>> {


        if self.vec.len() == 0 {
            return None
        }

        if self.index >= self.vec.len() {
            return None
        };

        
        let element = self.vec.remove(self.index);

        let item = VecRemoveIterItem {
            restore_index: self.index.clone(), // use this to put the item in the correct spot if restored
            vec: &mut self.vec,
            element,
            iter_index: &mut self.index
        }; 

        

        Some(item)
    }
}

pub struct VecRemoveIterItem<'a, T> {
    restore_index: usize,
    pub vec: &'a mut Vec<T>,
    pub element: T,
    iter_index: &'a mut usize
}

impl<'a, T> VecRemoveIterItem<'a, T> {
    pub fn restore(self) {

        if self.restore_index == self.vec.len() {
            self.vec.push(self.element)
        }
        
        else {
            self.vec.insert(self.restore_index, self.element);
        }

        *self.iter_index += 1;
        
    }
}

pub trait IntoVecRemoveIter<'a> {
    type Item;
    fn into_vec_remove_iter(&'a mut self) -> VecRemoveIter<'a, Self::Item>;
}

impl<'a, T> IntoVecRemoveIter<'a> for Vec<T> {
    type Item = T;
    fn into_vec_remove_iter(&'a mut self) -> VecRemoveIter<'a, T> {
        VecRemoveIter {
            index: 0,
            vec: self,
        }
    }
}
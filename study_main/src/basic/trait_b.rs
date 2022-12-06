struct SliceIterator {
    val:i32,
}

impl Iterator for SliceIterator{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
       if self.val < 10{
        self.val += 1;
        return Some(self.val);
       }
       None
    }
}

#[test]
fn test_trait_ob(){
   let slice_i = SliceIterator{val:2};
   for item in slice_i.into_iter(){
        println!("{}",item)
   }
}
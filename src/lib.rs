#![feature(step_trait)]

pub mod ref_list;
pub mod to_index;
pub mod test_data;
pub mod range_vec;

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::ref_list::RefList;
    use crate::test_data::input::Input;


    #[test]
    fn test_input() {
        let button = Input::MouseButton(0);

        {
            let mut rl = RefList::<Input>::new();
            rl.push(button.clone());
            let instant = Instant::now();
            let contains = rl.contains(&button);
            let _ = format_args!("{:?}", rl);
            println!("Found {:?} in RefList[{}]={} in {}secs.", button, rl.len(), contains, instant.elapsed().as_secs_f32())
        }
        {
            let l = Input::range();
            let instant = Instant::now();
            let contains = l.contains(&button);
            let _ = format_args!("{:?}", l);
            println!("Found {:?} in Vec[{}]={} in {}secs.", button, l.len(), contains, instant.elapsed().as_secs_f32())
        }
    }
}

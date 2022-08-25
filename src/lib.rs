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

        let rl_duration = {
            let mut rl = RefList::<Input>::new();
            rl.push(button.clone());
            let instant = Instant::now();
            let contains = rl.contains(&button);
            let _ = format_args!("{:?}", rl);
            let duration = instant.elapsed().as_secs_f32();
            println!("Found {:?} in RefList[{}]={} in {}secs.", button, rl.len(), contains, duration);
            duration
        };
        let list_duration = {
            let l = Input::range();
            let instant = Instant::now();
            let contains = l.contains(&button);
            let _ = format_args!("{:?}", l);
            let duration = instant.elapsed().as_secs_f32();
            println!("Found {:?} in Vec[{}]={} in {}secs.", button, l.len(), contains, duration);
            duration
        };
        assert!(rl_duration <= list_duration, "RefList Duration {} <= List Duration {} = {}", rl_duration, list_duration, rl_duration <= list_duration);
    }

    #[test]
    fn test_input_2() {
        let button = Input::MouseButton(0);

        let rl_duration = {
            let mut rl = RefList::<Input>::new();
            rl.push(button.clone());
            let instant = Instant::now();
            let contains = rl.contains(&button);
            let duration = instant.elapsed().as_secs_f32();
            println!("Found {:?} in RefList[{}]={} in {}secs.", button, rl.len(), contains, duration);
            duration
        };
        let list_duration = {
            let l = Input::range();
            let instant = Instant::now();
            let contains = l.contains(&button);
            let duration = instant.elapsed().as_secs_f32();
            println!("Found {:?} in Vec[{}]={} in {}secs.", button, l.len(), contains, duration);
            duration
        };
        assert!(rl_duration <= list_duration, "RefList Duration {} <= List Duration {} = {}", rl_duration, list_duration, rl_duration <= list_duration);
    }
}

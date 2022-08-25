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

    #[test]
    fn test_input3() {
        let list = {
            let mouse_ids = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let mut keys = ('a'..'t').into_iter().map(|value| Input::Key(value)).collect::<Vec<Input>>();
            let mut input = mouse_ids.iter().map(|value| Input::MouseButton(*value)).collect::<Vec<Input>>();
            input.append(&mut keys);
            input
        };
        let ref_list = {
            let mut ref_list = RefList::new();
            for value in &list {
                ref_list.push(value.clone());
            }
            ref_list
        };

        let iterations = 100;

        let list_time = {
            let instant = Instant::now();
            for _ in 0..iterations {
                for value in &list {
                    let _ = list.contains(value);
                }
            }
            instant.elapsed().as_secs_f32() / iterations as f32
        };
        let ref_list_time = {
            let instant = Instant::now();
            for _ in 0..iterations {
                for value in &list {
                    let _ = ref_list.contains(value);
                }
            }
            instant.elapsed().as_secs_f32() / iterations as f32
        };

        println!("Average List Contains: {}secs vs Average RefList Contains: {}secs", list_time, ref_list_time);
    }
}

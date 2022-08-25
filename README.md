# RefList
A data structure used to quickly determine if an element exists inside of the structure.

This data structure is primarily used to determine if an element is contained within the structure. The proof of concept is proven to be very efficient, up to 2x faster than the contains() function contained within the Vec struct.

RefList got its name from the usage of the structure; to determine if the structure contains a reference to a value.

# Use Cases
A RefList is not the best data structure to use if you are concerned about memory usage. The data structure can be very large in side, as it contains every possible value within the range of the type provided to the RefList.
For example, a RefList<u8> would have a size of `(0+255) x 4` in bytes. If both the RefList and a standard Vec have no elements, the standard Vec would be `0`.

However, if we are able to put the size of the RefList to the side, it is great at everything else. The contains and remove functions are ample faster than the standard Vec alternatives. This is because, using the `ToIndex` trait, you are able to predetermine the index of type T, which is used to change the value within the RefList from `None` to `Some(T)`.

# Examples
Suppose you were determining if mouse buttons/key strokes needed to be checked within your application. Using some functionality like... `InputSystem::contains_key('w')` and this would return true if the `W` key was pressed.
You could use a standard Vec, and keep track of every single keystroke/mouse button pressed and simply clear the vec at the end of the frame. However, according to tests I had ran, the contains function seems to be a little slow. Because of this, it makes it hard for this sort of application to use. (test results will be below).
Let's create a Input enum:
```rust
///This enumeration is the example of a mouse/keyboard input system.
pub enum Input {
  ///This represents a mouse input. Examples: left click, right click, center click. This supports up to 255 mouse inputs.
  MouseButton(u8),
  
  ///This represents a key stroke. Example, 'w', 'a', 's', 'd'. This supports a large array of inputs.
  Keyboard(char)
}
```

Now, implement `ToIndex` trait for `Input`:
```rust
impl ToIndex for Input {
    ///The max index is the sum of the max mouse value and the max char value.
    const MAX_INDEX: usize = Input::MAX_MOUSE + Input::MAX_CHAR;
    const MIN_INDEX: usize = 0;

    fn to_index(&self) -> usize {
        match self {
            Input::MouseButton(mouse) => {
                //If the input value is a mouse button, the index is between [MIN_INDEX..255]
                (*mouse) as usize
            }
            Input::Key(char) => {
                //If the input value is a key stroke, the index is between [256..MAX_INDEX]
                Input::MAX_MOUSE + 1 + (*char as usize)
            }
        }
    }
}
```

Now that `ToIndex` has been implemented, we can now create a `RefList` from type `Input`.
```rust
let mut rl = RefList::<Input>::new();
```
This `rl` RefList can now be used like any other data structure; making use of the `push`, `contains`, `remove`, `clear`, `len`, and `is_empty` functions.
Check out the test within the `lib.rs` file for an example.

# Test Results

<h1>With Formatting</h1>
This test was used to determine the efficiency of the RefList, when used in conjunction with the default formatter.

```rust
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
        assert!(rl_duration <= list_duration, "RefList Duration {} <= List Duration {} = {}", rl_duration, list_duration, rl_duration <= list_duration)
```

<h2>Results</h2> 

```text
Found MouseButton(0) in RefList[1114366]=true in 0.000000536secs.
Found MouseButton(0) in Vec[1114366]    =true in 0.000000755secs.
```

<h1>Simple Contains</h1>
This test was used to determine the efficiency of simply checking if a value is contained within the RefList.

```rust
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
```

<h2>Results</h2> 

```text
Found MouseButton(0) in RefList[1114366]=true in 0.000000314secs.
Found MouseButton(0) in Vec[1114366]    =true in 0.000001404secs.
```

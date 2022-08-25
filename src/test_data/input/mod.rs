use crate::to_index::ToIndex;

#[derive(Debug, Clone, PartialEq)]
///This enumeration is the example of a mouse/keyboard input system.
pub enum Input {
    ///This represents a mouse input. Examples: left click, right click, center click. This supports up to 255 mouse inputs.
    MouseButton(u8),

    ///This represents a key stroke. Example, 'w', 'a', 's', 'd'. This supports a large array of inputs.
    Key(char),
}

impl Input {
    ///This constant value represents a left click.
    pub const LEFT_CLICK: Input = Input::MouseButton(0);

    ///This constant value represents a right click.
    pub const RIGHT_CLICK: Input = Input::MouseButton(1);

    ///This constant value represents a center click.
    pub const CENTER_CLICK: Input = Input::MouseButton(2);

    ///This constant represents the maximum range of mouse inputs.
    const MAX_MOUSE: usize = u8::MAX as usize;

    ///This constant represents the maximum range of keyboard inputs.
    const MAX_CHAR: usize = char::MAX as usize;

    ///Create a new mouse input value, providing the moue button as an argument.
    pub fn mouse_button(mouse: u8) -> Input {
        Self::MouseButton(mouse)
    }

    ///Create a new keyboard input value, providing the key as an argument.
    pub fn key(key: char) -> Input {
        Self::Key(key)
    }

    ///Creates a list of all of the possible values of Input.
    pub fn range() -> Vec<Input> {
        let mut mouse = {
            let mut output = vec![];
            for ind in 0..Self::MAX_MOUSE {
                output.push(Input::MouseButton(ind as u8));
            }
            output
        };
        let mut keys = {
            let mut output = vec![];
            for i in 0..Self::MAX_CHAR {
                output.push(Input::Key(i as u8 as char));
            }
            output
        };

        mouse.append(&mut keys);
        mouse
    }
}

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
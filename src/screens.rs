use std::fmt::{Display, Formatter};

use crate::machines::VirtualMachine;

/// A single pixel on the screen. It can only be lit or dark.
#[derive(Debug, Clone, Copy)]
enum Pixel {
    Lit,
    Dark,
}

impl Into<char> for Pixel {
    /// Transform the pixel into the character it should display on the screen
    fn into(self) -> char {
        match self {
            Pixel::Lit => '#',
            Pixel::Dark => '.',
        }
    }
}

/// Defined by the specification
const SCREEN_WIDTH: usize = 40;

/// Defined by the specification
const SCREEN_HEIGHT: usize = 6;

/// A screen is a visual output controlled by an underlying machine. In this
/// case, the underlying machine executes a program which instructs the screen
/// when and where to light pixels.
pub struct Screen {
    machine: VirtualMachine,
    sprite_middle: isize,
    pixels: [Pixel; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Screen {
    /// Creates a new screen controlled by the given VM and its program
    pub fn new(machine: VirtualMachine) -> Self {
        // Initially, every pixel is dark
        let pixels = [Pixel::Dark; SCREEN_WIDTH * SCREEN_HEIGHT];

        // A sprite is three pixels wide. Its middle is the easiest way to track
        // its position. Initially, the middle is at index `1` so one pixel to
        // the left (index `0`) and one to the right (index `2`) are also
        // displayed.
        let middle = 1;

        Screen {
            machine,
            pixels,
            sprite_middle: middle,
        }
    }

    /// Refresh the screen so it is ready to be displayed. Underneath, this
    /// cycles the VM to determine if a pixel should be lit or not.
    pub fn refresh(&mut self) {
        while self.machine.is_executing() {
            self.light();
            self.machine.cycle();
            self.sprite_middle = self.machine.read_register();
        }
    }

    /// Lights a pixel if the VM signals for it
    fn light(&mut self) {
        // The screen updates pixels according to the program executing in the
        // underlying VM. It cycles the VM 240 times -- once for each pixel on
        // the screen. At each cycle, the index for the pixel is the machine's
        // tick (or cycle count). Since it's an index, subtract one.
        let screen_index = self.machine.get_ticks() - 1;
        let middle = self.sprite_middle;

        if screen_index >= SCREEN_HEIGHT * SCREEN_WIDTH {
            // TODO this is required for the sample but not the puzzle input
            return;
        }

        // The screen index maps into a flat array, but the screen is vertical.
        // The row index is found using the screen width.
        let row_index = screen_index % SCREEN_WIDTH;
        let row_index = row_index as isize;

        // The sprite is three pixels wide and tracked by its center position.
        // Light the pixel when the underlying program's register value (given
        // `screen_index` and `row_index`) aligns with the sprite's position on
        // the row.
        let should_light =
            row_index == middle || row_index == middle - 1 || row_index == middle + 1;

        let index = screen_index as usize;

        if should_light {
            self.pixels[index] = Pixel::Lit;
        }
    }
}

impl Display for Screen {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut column = 0;

        for pixel in self.pixels {
            if column == 40 {
                writeln!(formatter).unwrap();
                column = 0;
            }

            let pixel: char = pixel.into();
            write!(formatter, "{}", pixel).unwrap();

            column += 1;
        }

        writeln!(formatter)
    }
}
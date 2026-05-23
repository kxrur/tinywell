use serde::{Deserialize, Serialize};
use specta::Type;

/// Represents an individual LED in the grid
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Led {
    /// id
    pub id: usize,
    /// LED state
    pub enabled: bool,
    /// Overall brightness intensity (0.0 - 1.0)
    pub brightness: f32,
    /// Wavelength in nm
    pub wavelength: u16,
}

impl Default for Led {
    fn default() -> Self {
        Self {
            id: 0,
            enabled: false,
            brightness: 1.0,
            wavelength: 0,
        }
    }
}

/// Represents the physical LED grid
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LedGrid {
    pub rows: usize,
    pub cols: usize,
    pub leds: Vec<Led>,
}

impl LedGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let count = rows * cols;
        let mut leds = Vec::with_capacity(count);

        for id in 0..count {
            leds.push(Led {
                id,
                ..Default::default()
            });
        }

        Self { rows, cols, leds }
    }

    /// Retrieve a mutable reference to an LED by its index
    pub fn get_led_mut(&mut self, index: usize) -> Option<&mut Led> {
        self.leds.get_mut(index)
    }

    /// Retrieve an LED by its 2D coordinates
    pub fn get_led_mut_by_coord(&mut self, row: usize, col: usize) -> Option<&mut Led> {
        if row < self.rows && col < self.cols {
            let index = row * self.cols + col;
            self.leds.get_mut(index)
        } else {
            None
        }
    }
}

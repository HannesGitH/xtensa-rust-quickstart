//! # Use ws2812 leds with timers
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! The `new` method needs a periodic timer running at 3 MHz
//!
//! If it's too slow (e.g.  e.g. all/some leds are white or display the wrong color)
//! you may want to try the `slow` feature.

//#![no_std]

use esp32_hal::hal as hal;

use hal::digital::v2::OutputPin;
use xtensa_lx::timer::delay;
use smart_leds_trait::{SmartLedsWrite, RGB8};


pub struct Ws2812<PIN> {
    pin: PIN,
}

const DELAY_CYCLES:u32=4;

impl<PIN> Ws2812<PIN>
where
    PIN: OutputPin,
{
    /// The timer has to already run at with a frequency of 3 MHz
    pub fn new(mut pin: PIN) -> Ws2812<PIN> {
        pin.set_low().ok();
        Self {pin }
    }


    /// Write a single color for ws2812 devices
    #[cfg(not(feature = "slow"))]
    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0xF0) != 0 {
                self.pin.set_high().ok();
                delay(DELAY_CYCLES*14);
                self.pin.set_low().ok();
                delay(DELAY_CYCLES*12);
            } else {
                self.pin.set_high().ok();
                delay(DELAY_CYCLES*7);
                //self.pin.set_low().ok();
                delay(DELAY_CYCLES*16);
            }
            data <<= 1;
        }
    }

    /*
    /// Write a single color for ws2812 devices
    #[cfg(feature = "slow")]
    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
                delay(DELAY_CYCLES);
                self.pin.set_high().ok();
                delay(DELAY_CYCLES*2);
                self.pin.set_low().ok();
            } else {
                delay(DELAY_CYCLES);
                self.pin.set_high().ok();
                self.pin.set_low().ok();
                delay(DELAY_CYCLES*2);
            }
            data <<= 1;
        }
    }*/
}

impl<PIN> SmartLedsWrite for Ws2812<PIN>
where
    PIN: OutputPin,
{
    type Error = ();
    type Color = RGB8;
    /// Write all the items of an iterator to a ws2812 strip
    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>,
    {
        for (index, item) in iterator.enumerate() {
            let item = item.into();
            self.write_byte(item.g);
            self.write_byte(item.r);
            self.write_byte(item.b);
            //if index%10==0 {delay(DELAY_CYCLES*1100);}
        }
        // Get a timeout period of more than 50 micro s
        delay(DELAY_CYCLES*1100);
        Ok(())
    }
}
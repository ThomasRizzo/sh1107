//! sh1107 Communication Interface (I2C/SPI)
//!
//! These are the two supported interfaces for communicating with the display. They're used by the
//! [builder](../builder/index.html) methods
//! [connect_i2c](../builder/struct.Builder.html#method.connect_i2c) and
//! [connect_spi](../builder/struct.Builder.html#method.connect_spi).
//!
//! The types that these interfaces define are quite lengthy, so it is recommended that you create
//! a type alias. Here's an example for the I2C1 on an STM32F103xx:
//!
//! ```rust
//! # extern crate sh1107;
//! # extern crate stm32f103xx_hal as hal;
//! # use hal::gpio::gpiob::{PB8, PB9};
//! # use hal::gpio::{Alternate, OpenDrain};
//! # use hal::i2c::I2c;
//! # use hal::prelude::*;
//! # use hal::stm32f103xx::I2C1;
//! # use sh1107::interface::I2cInterface;
//! type OledDisplay = GraphicsMode<
//!     I2cInterface<I2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>>,
//! >;
//! ```
//!
//! Here's one for SPI1 on an STM32F103xx:
//!
//! ```rust
//! # extern crate sh1107;
//! # extern crate stm32f103xx_hal as hal;
//! # use hal::gpio::gpioa::{PA5, PA6, PA7};
//! # use hal::gpio::gpiob::PB1;
//! # use hal::gpio::{Alternate, Floating, Input, Output, PushPull};
//! # use hal::spi::Spi;
//! # use hal::stm32f103xx::SPI1;
//! # use sh1107::interface::SpiInterface;
//! pub type OledDisplay = GraphicsMode<
//!     SpiInterface<
//!         Spi<
//!             SPI1,
//!             (
//!                 PA5<Alternate<PushPull>>,
//!                 PA6<Input<Floating>>,
//!                 PA7<Alternate<PushPull>>,
//!             ),
//!         >,
//!         PB1<Output<PushPull>>,
//!     >,
//! >;
//! ```

pub mod i2c;
pub mod spi;
//use core::future::Future;

//TODO: proper way to implement async trait?
/*
use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
`#[warn(async_fn_in_trait)]` on by defaultrustcClick for full compiler diagnostic
mod.rs(65, 54): you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change: `impl std::future::Future<Output = `, `> + Send`
*/

/// A method of communicating with sh1107
#[allow(async_fn_in_trait)]
pub trait DisplayInterface {
    /// Interface error type
    type Error;

    /// Initialize device.
    fn init(&mut self) -> Result<(), Self::Error>;
    /// Send a batch of up to 8 commands to display.
    async fn send_commands(&mut self, cmd: &[u8]) -> Result<(), Self::Error>;
    //fn send_commands(&mut self, cmd: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// Send data to display.
    async fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
    //fn send_data(&mut self, buf: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + Send;
}


pub use self::{i2c::I2cInterface, spi::SpiInterface};

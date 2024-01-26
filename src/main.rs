#![no_std]
#![no_main]
use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::board;
use bsp::hal::gpt::ClockSource;
use bsp::hal::timer::Blocking;

const GPT1_FREQUENCY: u32 = 1_000;
const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;

#[bsp::rt::entry]
fn main() -> ! {
    let instances = board::instances();

    let board::Resources {
        pins,
        mut gpt1,
        mut gpio2,
        srtc,
        mut snvs_lp_core,
        ..
    } = board::t40(instances);

    let led = board::led(&mut gpio2, pins.p13);

    gpt1.disable();
    gpt1.set_divider(GPT1_DIVIDER);
    gpt1.set_clock_source(GPT1_CLOCK_SOURCE);

    let mut delay = Blocking::<_, { GPT1_FREQUENCY }>::from_gpt(gpt1);

    delay.block_ms(2000);

    led.toggle();

    let _srtc = srtc.enable(&mut snvs_lp_core);

    led.toggle();

    loop {
        delay.block_ms(1000);
        led.toggle();
    }
}

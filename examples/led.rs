#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::select::select;
use embassy_rp::{
    block::ImageDef,
    gpio::{Input, Level, Output, Pull},
    pwm::{self, Pwm},
};
use embassy_time::Timer;
use panic_probe as _;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blink"),
    embassy_rp::binary_info::rp_program_description!(
        c"The RP Pico Hello, World application blinking the led connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

/// Make the light fade in and fade out instead of blinking
async fn _fade_in_out(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // let mut led = Output::new(p.PIN_25, Level::Low);
    let mut config = pwm::Config::default();
    let mut led = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, config.clone());
    config.compare_b = 1;

    let mut incr = true;

    loop {
        if incr {
            config.compare_b = config.compare_b.saturating_add(256);
        } else {
            config.compare_b = config.compare_b.saturating_sub(256);
        }
        if config.compare_b >= config.top {
            incr = false;
        }
        if config.compare_b <= 1 {
            incr = true;
        }
        Timer::after_millis(5).await;
        led.set_config(&config);
    }
}

/// Turn the LED off when the wire is connecting gnd and gpio 2
async fn _on_off_with_wire(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);
    let button = Input::new(p.PIN_2, Pull::Up);
    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
        Timer::after_millis(10).await;
    }
}

/// When the pin voltage is changed the light is toggled.
async fn _touch_on_off(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);
    let mut button = Input::new(p.PIN_2, Pull::Up);
    loop {
        loop {
            button.wait_for_falling_edge().await;
            Timer::after_millis(50).await;
            if button.is_low() {
                break;
            }
        }
        if led.is_set_low() {
            led.set_high();
        } else {
            led.set_low();
        }

        loop {
            button.wait_for_rising_edge().await;
            Timer::after_millis(50).await;
            if button.is_high() {
                break;
            }
        }
    }
}

/// State machine controlled by touching a pin
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    #[derive(PartialEq)]
    pub enum State {
        Off,
        Blink,
        Fade,
    }

    impl State {
        fn next(&self) -> Self {
            if *self == Self::Off {
                return Self::Blink;
            } else if *self == Self::Blink {
                return Self::Fade;
            } else {
                return Self::Off;
            }
        }
    }
    let mut state = State::Off;

    let p = embassy_rp::init(Default::default());
    let mut button = Input::new(p.PIN_2, Pull::Up);

    let mut config = pwm::Config::default();
    let mut led = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, config.clone());
    config.compare_b = 1;

    let mut incr = true;

    async fn animate(led: &mut Pwm<'_>, config: &mut pwm::Config, state: &State, incr: &mut bool) {
        loop {
            match state {
                State::Off => {
                    config.compare_b = 0;
                    led.set_config(&config);
                    Timer::after_millis(1000).await;
                }
                State::Blink => {
                    if config.compare_b >= config.top {
                        config.compare_b = 0;
                    } else {
                        config.compare_b = config.top;
                    }
                    led.set_config(&config);
                    Timer::after_millis(500).await;
                }
                State::Fade => {
                    if *incr {
                        config.compare_b = config.compare_b.saturating_add(256);
                    } else {
                        config.compare_b = config.compare_b.saturating_sub(256);
                    }
                    if config.compare_b >= config.top {
                        *incr = false;
                    } else if config.compare_b <= 1 {
                        *incr = true;
                    }
                    led.set_config(&config);
                    Timer::after_millis(5).await;
                }
            }
        }
    }

    async fn wait_for_change(button: &mut Input<'_>) {
        loop {
            button.wait_for_falling_edge().await;
            Timer::after_millis(50).await;
            if button.is_low() {
                return;
            }
        }
    }

    loop {
        match select(
            animate(&mut led, &mut config, &state, &mut incr),
            wait_for_change(&mut button),
        )
        .await
        {
            embassy_futures::select::Either::First(_) => {}
            embassy_futures::select::Either::Second(_) => state = state.next(),
        }
    }
}

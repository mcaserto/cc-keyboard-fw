// embassy includes
use embassy_rp::{
    bind_interrupts,
    Peri,
    peripherals::{self, DMA_CH0, PIO0},
    pio::{InterruptHandler, Pio},
    pio_programs::ws2812::{PioWs2812, PioWs2812Program},
};
use embassy_time::Timer;

// crate includes
use crate::cc_engine::tasks::resources;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
pub async fn status_light_handler(status_led: Peri<'static, peripherals::PIN_17>, pio: Peri<'static, PIO0>, dma: Peri<'static, DMA_CH0>) -> ! {
    // setup pio for driving addressable led
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio, Irqs);

    const NUM_LEDS: usize = 1;
    let program = PioWs2812Program::new(&mut common);
    let mut led_output: PioWs2812<'_, PIO0, 0, NUM_LEDS> =
        PioWs2812::new(&mut common, sm0, dma, status_led, &program);

    // initialize state
    let mut counter: u32 = 0;
    let mut status = resources::Status::Heartbeat;
    let mut color = smart_leds::RGB8::new(0, 0, 0);
    led_output.write(&[color]).await;

    loop {
        if resources::STATUS_SIGNAL.signaled() {
            status = resources::STATUS_SIGNAL.wait().await;
        }

        color = match status {
            resources::Status::Idle => smart_leds::RGB8::new(0, 5, 0),
            resources::Status::Error => smart_leds::RGB8::new(50, 0, 0),
            resources::Status::Color(color) => color,
            resources::Status::Heartbeat => {
                if counter % 2 == 0 {
                    smart_leds::RGB8::new(5, 5, 5)
                } else {
                    smart_leds::RGB8::new(0, 0, 0)
                }
            }
        };

        led_output.write(&[color]).await;
        counter += 1;
        Timer::after_millis(250).await;
    }
}

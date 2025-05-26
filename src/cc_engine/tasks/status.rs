// embassy includes
use embassy_rp::{
    bind_interrupts,
    peripherals::{self, DMA_CH0, PIO0},
    pio::{InterruptHandler, Pio},
    pio_programs::ws2812::{PioWs2812, PioWs2812Program},
};
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
pub async fn status_light_handler(status_led: peripherals::PIN_17, pio: PIO0, dma: DMA_CH0) -> ! {
    // do setup
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio, Irqs);

    const NUM_LEDS: usize = 1;
    let program = PioWs2812Program::new(&mut common);
    let mut ws2812: PioWs2812<'_, PIO0, 0, NUM_LEDS> =
        PioWs2812::new(&mut common, sm0, dma, status_led, &program);

    // loop
    let mut count: u32 = 0;
    let mut color = smart_leds::RGB8::new(0, 0, 0);
    loop {
        // // blink led
        if (count % 2) != 0 {
            color.r = 30;
            color.g = 0;
            color.b = 0;
        } else {
            color.r = 0;
            color.g = 30;
            color.b = 0;
        }

        ws2812.write(&[color]).await;

        count += 1;
        Timer::after_secs(1).await;
    }
}

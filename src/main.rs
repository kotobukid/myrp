#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use rp2040_hal as hal;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC;

#[entry]
fn main() -> ! {
    // ウォッチドッグ＆クロック初期化（デフォルトのXOSC 12MHz前提）
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    // SIO/GPIO
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // PicoのオンボードLEDは GP25
    let mut led = pins.gpio25.into_push_pull_output();

    // 適当なソフトウェアディレイ
    loop {
        led.set_high().unwrap();
        busy_delay(300_000);
        led.set_low().unwrap();
        busy_delay(300_000);
    }
}

// 超簡易ディレイ（タイマ未使用の手抜き版）
#[inline(never)]
fn busy_delay(mut n: u32) {
    while n > 0 {
        cortex_m::asm::nop();
        n -= 1;
    }
}

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_halt as _;

/// micro:bit v2 で LED を点滅させる "Hello, World!" プログラム
///
/// micro:bit v2 の 5x5 LED マトリクスは行と列で制御される:
/// - ROW を HIGH にして電流のソースにする
/// - COL を LOW にして電流のシンクにする
/// - ROW=HIGH かつ COL=LOW のとき LED が点灯
#[entry]
fn main() -> ! {
    // ボードの初期化
    let board = Board::take().unwrap();

    // タイマーの初期化
    let mut timer = Timer::new(board.TIMER0);

    // LED マトリクスの行1と列1を取得
    // 行(ROW)は HIGH で電流を供給、列(COL)は LOW で電流を吸い込む
    let mut row1 = board
        .display_pins
        .row1
        .into_push_pull_output(microbit::hal::gpio::Level::High); // ROW を HIGH に
    let mut col1 = board
        .display_pins
        .col1
        .into_push_pull_output(microbit::hal::gpio::Level::Low); // COL を LOW に（LED ON）

    // LED を点滅（無限ループ）
    loop {
        // LED ON: ROW=HIGH, COL=LOW
        row1.set_high().unwrap();
        col1.set_low().unwrap();
        timer.delay_ms(500u32);

        // LED OFF: COL=HIGH にして電流を止める
        col1.set_high().unwrap();
        timer.delay_ms(500u32);
    }
}

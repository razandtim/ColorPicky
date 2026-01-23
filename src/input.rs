use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::InputPin;

pub enum ButtonEvent {
    SingleClick,
    DoubleClick,
    LongPress,
}

pub struct ButtonInput<P> {
    pin: P,
    is_pressed: bool,
    full_press_start: Option<Instant>,
    last_release_time: Option<Instant>,
    click_count: u8,
}

impl<P: InputPin> ButtonInput<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            is_pressed: false,
            full_press_start: None,
            last_release_time: None,
            click_count: 0,
        }
    }

    /// Poll the button state. Should be called frequently.
    /// Returns an event if one occurred.
    pub async fn poll(&mut self) -> Option<ButtonEvent> {
        let current_pressed = self.pin.is_low().ok().unwrap_or(false);
        let now = Instant::now();

        // Edge detection
        if current_pressed && !self.is_pressed {
            // Press started
            self.is_pressed = true;
            self.full_press_start = Some(now);
        } else if !current_pressed && self.is_pressed {
            // Released
            self.is_pressed = false;
            let duration = now - self.full_press_start.unwrap_or(now);

            if duration > Duration::from_millis(1000) {
                // Reset click logic after long press
                self.click_count = 0;
                return Some(ButtonEvent::LongPress);
            } else if duration > Duration::from_millis(50) {
                // Short press (debounce 50ms)
                self.click_count += 1;
                self.last_release_time = Some(now);
            }
        }

        // Logic to detect single vs double click
        if self.click_count > 0 && !self.is_pressed {
            let time_since_release = now - self.last_release_time.unwrap_or(now);

            if self.click_count == 2 {
                self.click_count = 0;
                return Some(ButtonEvent::DoubleClick);
            }

            if time_since_release > Duration::from_millis(350) {
                // Time window for double click expired, so it's a single click
                self.click_count = 0;
                return Some(ButtonEvent::SingleClick);
            }
        }

        Timer::after_millis(10).await; // Polling delay
        None
    }
}

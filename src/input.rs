use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::InputPin;

pub enum ButtonEvent {
    SingleClick,
    DoubleClick,
    LongPress,
    Held,     // Button is currently being held down
    Released, // Button was just released (after being held)
}

pub struct ButtonInput<P> {
    pin: P,
    is_pressed: bool,
    full_press_start: Option<Instant>,
    last_release_time: Option<Instant>,
    click_count: u8,
    was_held_long: bool, // Track if we already fired Held events
}

impl<P: InputPin> ButtonInput<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            is_pressed: false,
            full_press_start: None,
            last_release_time: None,
            click_count: 0,
            was_held_long: false,
        }
    }

    /// Check if button is currently pressed
    pub fn is_held(&self) -> bool {
        self.is_pressed
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
            self.was_held_long = false;
        } else if !current_pressed && self.is_pressed {
            // Released
            self.is_pressed = false;
            let duration = now - self.full_press_start.unwrap_or(now);

            if self.was_held_long {
                // Was sampling, now released - save the color
                self.was_held_long = false;
                self.click_count = 0;
                return Some(ButtonEvent::Released);
            } else if duration > Duration::from_millis(1000) {
                // Long press without held mode
                self.click_count = 0;
                return Some(ButtonEvent::LongPress);
            } else if duration > Duration::from_millis(50) {
                // Short press (debounce 50ms)
                self.click_count += 1;
                self.last_release_time = Some(now);
            }
        }

        // While held, after 200ms threshold, start "sampling mode"
        if self.is_pressed && !self.was_held_long {
            let duration = now - self.full_press_start.unwrap_or(now);
            if duration > Duration::from_millis(200) {
                self.was_held_long = true;
                return Some(ButtonEvent::Held);
            }
        }

        // If still held and in sampling mode, return Held continuously
        if self.is_pressed && self.was_held_long {
            Timer::after_millis(10).await;
            return Some(ButtonEvent::Held);
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

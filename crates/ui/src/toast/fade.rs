use std::time::Duration;

pub const LIFETIME: Duration = Duration::from_millis(1100);
const FADE_IN: Duration = Duration::from_millis(80);
const FADE_OUT: Duration = Duration::from_millis(180);

pub fn opacity(elapsed: Duration) -> f32 {
    let rising = elapsed.as_secs_f32() / FADE_IN.as_secs_f32();
    let falling = LIFETIME.saturating_sub(elapsed).as_secs_f32() / FADE_OUT.as_secs_f32();
    rising.min(falling).clamp(0.0, 1.0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEvent {
    Down { x: i32, y: i32, at_ms: u64 },
    Up { x: i32, y: i32, at_ms: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gesture {
    Drag,
    MultiClick,
}

const DRAG_DISTANCE_PX: i64 = 4;
const MULTI_CLICK_WINDOW_MS: u64 = 500;

#[derive(Debug, Clone, Copy)]
struct Press {
    x: i32,
    y: i32,
    at_ms: u64,
}

#[derive(Debug, Default)]
pub struct GestureTracker {
    pending: Option<Press>,
    last_click: Option<Press>,
}

impl GestureTracker {
    pub fn feed(&mut self, event: MouseEvent) -> Option<Gesture> {
        match event {
            MouseEvent::Down { x, y, at_ms } => {
                self.pending = Some(Press { x, y, at_ms });
                None
            }
            MouseEvent::Up { x, y, at_ms } => {
                let down = self.pending.take()?;
                let up = Press { x, y, at_ms };
                if apart_sq(down, up) >= DRAG_DISTANCE_PX * DRAG_DISTANCE_PX {
                    self.last_click = None;
                    return Some(Gesture::Drag);
                }
                let chained = self.last_click.is_some_and(|prev| continues(prev, down));
                self.last_click = Some(down);
                chained.then_some(Gesture::MultiClick)
            }
        }
    }
}

fn continues(prev: Press, next: Press) -> bool {
    next.at_ms.saturating_sub(prev.at_ms) <= MULTI_CLICK_WINDOW_MS
        && apart_sq(prev, next) <= DRAG_DISTANCE_PX * DRAG_DISTANCE_PX
}

fn apart_sq(a: Press, b: Press) -> i64 {
    let dx = i64::from(a.x) - i64::from(b.x);
    let dy = i64::from(a.y) - i64::from(b.y);
    dx * dx + dy * dy
}

#[cfg(test)]
mod tests;

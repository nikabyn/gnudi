use std::time::{Duration, Instant};

use color::{AlphaColor, Srgb};

use crate::Margin;

pub struct Animation<T: Blend = f32> {
    pub(crate) from: T,
    pub(crate) target: T,

    pub(crate) started: Instant,
    pub(crate) duration: Duration,
    pub(crate) easing: fn(f32) -> f32,
}

// anything an animation can move between, `amount` goes from 0 at `from` to 1 at `to`
pub trait Blend: Copy + PartialEq + Send + Sync + 'static {
    fn blend(from: Self, to: Self, amount: f32) -> Self;
}

impl<T: Blend> Animation<T> {
    // starts at rest on `value`, nothing moves until `to` is called
    pub fn new(value: T) -> Self {
        Self {
            from: value,
            target: value,

            started: Instant::now(),
            duration: Duration::from_millis(200),
            easing: easing_linear,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;

        self
    }

    pub fn easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;

        self
    }

    // a change halfway through starts from where it is now, so it never jumps
    pub fn to(&mut self, target: T) {
        if target == self.target {
            return;
        }

        self.from = self.current();
        self.target = target;
        self.started = Instant::now();
    }

    // read in the view, and while it is still moving the window keeps drawing frames
    pub fn value(&self) -> T {
        if self.progress() < 1.0 {
            moving::set();
        }

        self.current()
    }

    fn current(&self) -> T {
        let eased = (self.easing)(self.progress());

        T::blend(self.from, self.target, eased)
    }

    // 0 when it has just started, 1 once it has arrived
    fn progress(&self) -> f32 {
        if self.duration.is_zero() {
            return 1.0;
        }

        let elapsed = self.started.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();

        f32::min(elapsed / total, 1.0)
    }
}

impl Blend for f32 {
    fn blend(from: Self, to: Self, amount: f32) -> Self {
        from + (to - from) * amount
    }
}

impl Blend for f64 {
    fn blend(from: Self, to: Self, amount: f32) -> Self {
        from + (to - from) * (amount as f64)
    }
}

impl Blend for u8 {
    fn blend(from: Self, to: Self, amount: f32) -> Self {
        let from = f32::from(from);
        let to = f32::from(to);

        let blended = f32::blend(from, to, amount);

        blended.round() as u8
    }
}

impl Blend for i32 {
    fn blend(from: Self, to: Self, amount: f32) -> Self {
        let from = f64::from(from);
        let to = f64::from(to);

        let blended = f64::blend(from, to, amount);

        blended.round() as i32
    }
}

impl Blend for AlphaColor<Srgb> {
    fn blend(from: Self, to: Self, amount: f32) -> Self {
        AlphaColor::lerp_rect(from, to, amount)
    }
}

impl Blend for Margin {
    fn blend(from: Self, to: Self, amount: f32) -> Self {
        Margin {
            top: i32::blend(from.top, to.top, amount),
            right: i32::blend(from.right, to.right, amount),
            bottom: i32::blend(from.bottom, to.bottom, amount),
            left: i32::blend(from.left, to.left, amount),
        }
    }
}

/// the same speed the whole way
pub fn easing_linear(progress: f32) -> f32 {
    progress
}

/// starts fast and slows down as it arrives
pub fn easing_out(progress: f32) -> f32 {
    let remaining = 1.0 - progress;

    1.0 - remaining * remaining * remaining
}

/// starts slow, speeds up in the middle and slows down again
pub fn easing_in_out(progress: f32) -> f32 {
    if progress < 0.5 {
        return 4.0 * progress * progress * progress;
    }

    let remaining = 2.0 - 2.0 * progress;

    1.0 - remaining * remaining * remaining / 2.0
}

/*
 * asks for one more frame after this one; call it from the view while
 * something you move yourself has not arrived, and stop calling it once
 * it has, so the window can rest
 */
pub fn request_frame() {
    moving::set();
}

pub mod moving {
    use std::sync::atomic::{AtomicBool, Ordering};

    // set by any animation the view read that has not arrived yet
    static MOVING: AtomicBool = AtomicBool::new(false);

    pub fn set() {
        MOVING.store(true, Ordering::Relaxed);
    }

    // asked once after each view, so the flag only ever describes that view
    pub fn take() -> bool {
        MOVING.swap(false, Ordering::Relaxed)
    }
}

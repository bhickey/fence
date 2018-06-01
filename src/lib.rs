use std::{thread, time};
use std::marker::{Send, Sync};

pub struct Fence {
  duration: time::Duration,
  block_until: time::Instant,
}

unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

/// Fence provides a timed rate limiter. It's useful for imposing a framerate
/// cap on a UI thread, or enforcing periodicity on a polling thread.
/// 
/// In a typical usage, place a fence at the end of a loop to control the
/// invocation rate.
impl Fence {

    /// Construct a fence from the specified seconds.
    pub fn from_secs(s: u64) -> Fence {
        Fence::from_duration(time::Duration::from_secs(s))
    }

    /// Construct a fence from the specified milliseconds.
    pub fn from_millis(m: u64) -> Fence {
        Fence::from_duration(time::Duration::from_millis(m))
    }

    /// Construct a fence from the given duration.
    pub fn from_duration(dur: time::Duration) -> Fence {
        Fence {
            duration: dur,
            block_until: time::Instant::now() + dur,
        }
    }

    /// Sleep the current thread until at least the specified passage of time.
    pub fn sleep(&mut self) {
        let now = time::Instant::now();
        if now < self.block_until {
          thread::sleep(self.block_until.duration_since(now))
        }
        self.block_until = time::Instant::now() + self.duration;
    }

    pub fn allow(&mut self) -> bool {
        let now = time::Instant::now();
        if now < self.block_until {
            return false;
        }
        self.block_until = now + self.duration;
        true
    }
}

#[cfg(test)]
mod tests {
    use std::time;
    use Fence;

    #[test]
    fn fence_blocks() {
        let fence_dur = time::Duration::from_millis(10);
        let mut f = Fence::from_duration(fence_dur);
        let before = time::Instant::now();
        
        f.sleep();
        let after = time::Instant::now();
        assert!(after >= before + fence_dur);
    }

    #[test]
    fn fence_rate_limits() {
        let fence_dur = time::Duration::from_millis(5);
        let mut f = Fence::from_duration(fence_dur);
        let before = time::Instant::now();

        let lim = 10;
        for _ in 0..lim {
          f.sleep()
        }
        let after = time::Instant::now();
        assert!(after >= before + fence_dur * lim);
    }
}

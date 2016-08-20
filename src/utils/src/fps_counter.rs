pub struct FpsCounter {
    current_delta: ::Delta,
    frames: u32,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            current_delta: 0.0,
            frames: 0,
        }
    }

    pub fn frame(&mut self, delta: ::Delta) {
        self.frames += 1;

        self.current_delta += delta;

        while self.current_delta > 1.0 {
            self.current_delta -= 1.0;
            if self.frames < 50 {
                warn!("FPS Low: {}", self.frames);
            } else {
                info!("FPS: {}", self.frames);
            }
            self.frames = 0;
        }
    }
}


#[derive(Debug, Clone)]
struct BowlingFrame {
    rolls: Vec<u8>,
    last_frame: bool,
    current_pins: u8,
}

enum FrameResult {
    New,
    Continue,
}

#[derive(Debug, Clone, Copy)]
enum Special {
    Spare,
    Strike,
    Meh,
}

#[derive(Debug, Clone)]
pub struct BowlingGame {
    current_frame: usize,
    frames: Vec<BowlingFrame>,
    over: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 0,
            frames: Vec::new(),
            over: false,
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), String> {
        print!("Rolling {}.", pins);
        if self.over {
            return Err(format!("Game is over"));
        }
        let mut this_frame = self.frames.pop().unwrap_or(BowlingFrame::new(false));

        let res = this_frame.roll(pins);
        match res {
            Ok(x) => {
                match x {
                    FrameResult::New if this_frame.last_frame => {
                        self.frames.push(this_frame);
                        self.over = true;
                    }
                    FrameResult::New => {
                        self.frames.push(this_frame);
                        if self.frames.len() >= 9 {
                            self.frames.push(BowlingFrame::new(true));
                        } else {
                            self.frames.push(BowlingFrame::new(false));
                        }
                    }
                    FrameResult::Continue => {
                        self.frames.push(this_frame);
                    }
                }
                Ok(())
            }
            Err(n) => Err(n),
        }
    }

    fn all_rolls(&self) -> Vec<(Special, u8)> {
        let mut all_rolls: Vec<(Special, u8)> = Vec::new();

        for frame in self.frames.iter() {
            for roll in frame.create_rolls() {
                all_rolls.push(roll)
            }
        }
        all_rolls
    }

    pub fn score(&self) -> Result<u32, String> {
        println!("{:?}", self);
        match self.over {
            true => {
                let mut total: u32 = 0;
                let all_rolls = self.all_rolls();

                println!("{:?}", all_rolls);
                for (i, roll) in all_rolls.clone().into_iter().enumerate() {
                    println!("Current total: {} Adding {}", total, roll.1);
                    total += roll.1 as u32;
                    total += match roll.0 {
                        Special::Strike if i < all_rolls.len() - 3 => {
                            let col: Vec<u32> = all_rolls.clone()
                                .into_iter()
                                .skip(i + 1)
                                .take(2)
                                .map(|x| x.1 as u32)
                                .collect();
                            println!("strike! {:?}", col);
                            col.iter().sum()
                        }
                        Special::Spare if i < all_rolls.len() - 3 => {
                            let col: Vec<u32> = all_rolls.clone()
                                .into_iter()
                                .skip(i + 1)
                                .take(1)
                                .map(|x| x.1 as u32)
                                .collect();
                            println!("Spare! {:?}", col);
                            col.iter().sum()
                        }
                        _ => 0,
                    }
                }
                Ok(total)
            }
            n => Err(format!("Current frame is {}", n)),
        }
    }
}

impl BowlingFrame {
    fn new(last_frame: bool) -> Self {
        BowlingFrame {
            rolls: Vec::new(),
            last_frame: last_frame,
            current_pins: 10,
        }
    }

    fn roll(&mut self, pins: u8) -> Result<FrameResult, String> {
        if pins > 10 {
            return Err(format!("Number of pins is incorrect: {}", pins));
        }
        if pins > self.current_pins {
            return Err(format!("Trying to remove {} pins. Only have {}",
                               pins,
                               self.current_pins));
        }
        self.rolls.push(pins);
        self.current_pins -= pins;
        if self.last_frame {
            match self.rolls.len() {
                1 => {
                    if self.current_pins == 0 {
                        self.current_pins = 10
                    }
                    Ok(FrameResult::Continue)
                }
                2 => {
                    if self.current_pins == 0 {
                        self.current_pins = 10;
                    } 
                    if self.rolls.iter().sum::<u8>() >= 10 {
                        Ok(FrameResult::Continue)
                    } else {
                        Ok(FrameResult::New)
                    }
                }
                3 => Ok(FrameResult::New),
                n => Err(format!("Should not get {} rolls on the last frame", n)),
            }
        } else {
            if self.rolls.len() == 2 || self.rolls.iter().sum::<u8>() == 10 {
                return Ok(FrameResult::New);
            }
            Ok(FrameResult::Continue)
        }
    }

    fn create_rolls(&self) -> Vec<(Special, u8)> {
        let mut ret_vec: Vec<(Special, u8)> = Vec::new();
        let mut first = true;
        let mut current_total = 0;
        for &roll in self.rolls.iter() {
            current_total += roll;

            let sp = match current_total {
                10 if first => Special::Strike,
                10 if !first => Special::Spare,
                _ => Special::Meh,
            };
            ret_vec.push((sp, roll as u8));

            first = false;
        }
        ret_vec
    }
}
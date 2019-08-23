use crate::scripting::{Command, ParseError, ScriptVariable, Token, Variable};
use crate::Vector2f;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct PenState {
    pub position: Vector2f,
    pub color: [u8; 3],
    pub rotation: f32,

    pub step: f32,
    pub turning_angle: f32,
}

impl PenState {
    pub fn new() -> PenState {
        PenState {
            position: Vector2f::new(0.0, 0.0),
            color: [0, 0, 0],
            rotation: 0.0,
            step: 1.0,
            turning_angle: 90.0,
        }
    }

    pub fn load_config(&mut self, config: &HashMap<String, f64>) -> Result<(), ParseError> {
        use std::str::FromStr;
        for (var, value) in config {
            let var = Variable::from_str(var)?;
            self.get_variable(var).from_num(*value);
        }
        Ok(())
    }

    pub fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = (new_rotation + 360.0) % 360.0;
    }

    pub fn get_direction(&self) -> Vector2f {
        let x = self.rotation.to_radians().cos();
        let y = self.rotation.to_radians().sin();
        Vector2f::new(x * self.step, y * self.step)
    }

    fn get_variable(&mut self, var: Variable) -> &mut dyn ScriptVariable {
        match var {
            Variable::Rotation => &mut self.rotation,
            Variable::ColorR => &mut self.color[0],
            Variable::ColorG => &mut self.color[1],
            Variable::ColorB => &mut self.color[2],
            Variable::TurningAngle => &mut self.turning_angle,
            Variable::Step => &mut self.step,
        }
    }
}

pub struct Drawer {
    pub pen: PenState,
    pub stack: Vec<PenState>,
    pub commands: HashMap<u8, Vec<Command>>,
}

impl Drawer {
    pub fn new(pen: PenState) -> Drawer {
        Drawer {
            pen,
            stack: Vec::new(),
            commands: HashMap::new(),
        }
    }

    pub fn update(&mut self, symbol: u8) -> Vec<(Vector2f, Vector2f, [u8; 3])> {
        let mut strokes = Vec::new();
        if !self.commands.contains_key(&symbol) {
            return strokes;
        }

        let commands = &self.commands[&symbol];
        for command in commands {
            match command {
                Command::Forward => {
                    let dir = self.pen.get_direction();
                    let n_pos = self.pen.position + dir;
                    strokes.push((self.pen.position, n_pos, self.pen.color));
                    self.pen.position = n_pos;
                }
                Command::ClockWise => {
                    self.pen
                        .set_rotation(self.pen.rotation + self.pen.turning_angle);
                }
                Command::CounterClockWise => {
                    self.pen
                        .set_rotation(self.pen.rotation - self.pen.turning_angle);
                }
                Command::PushStack => {
                    self.stack.push(self.pen.clone());
                }
                Command::PopStack => {
                    self.pen = self
                        .stack
                        .pop()
                        .expect("A stack pop was invoked when there are no states on the stack");
                }
                Command::Add(x, y) => {
                    let b = match y {
                        Token::Number(n) => *n,
                        Token::Variable(var) => self.pen.get_variable(*var).to_num(),
                    };

                    let a = self.pen.get_variable(*x);
                    a.from_num(a.to_num() + b);
                }
                Command::Multiply(x, y) => {
                    let b = match y {
                        Token::Number(n) => *n,
                        Token::Variable(var) => self.pen.get_variable(*var).to_num(),
                    };
                    let a = self.pen.get_variable(*x);
                    a.from_num(a.to_num() * b);
                }
                Command::Set(x, y) => {
                    let b = match y {
                        Token::Number(n) => *n,
                        Token::Variable(var) => self.pen.get_variable(*var).to_num(),
                    };
                    let a = self.pen.get_variable(*x);
                    a.from_num(b);
                }
            }
        }

        strokes
    }

    pub fn load_config(&mut self, config: &HashMap<char, Vec<String>>) -> Result<(), ParseError> {
        for (symbol, commands) in config {
            let mut list = Vec::new();
            for command in commands {
                let cmd = command.parse()?;
                list.push(cmd);
            }
            self.commands.insert(*symbol as u8, list);
        }

        Ok(())
    }
}

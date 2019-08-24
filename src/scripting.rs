use std::str::FromStr;

// This trait is implemented for types that have to interact with the config files
// Scripts for now only use floating point numbers
// from_num() cannot return Self because then
// we wouldn't be able to make ScriptVariable into a trait object
pub trait ScriptVariable {
    fn from_num(&mut self, value: f64);
    fn to_num(&self) -> f64;
}

impl ScriptVariable for u8 {
    fn from_num(&mut self, value: f64) {
        *self = ((value + 256.0) % 256.0) as u8;
    }
    fn to_num(&self) -> f64 {
        f64::from(*self)
    }
}

impl ScriptVariable for f32 {
    fn from_num(&mut self, value: f64) {
        *self = value as f32;
    }
    fn to_num(&self) -> f64 {
        f64::from(*self)
    }
}

// TODO: maybe have a separate error for Variable/Token/Command
use std::{error, fmt};

#[derive(Debug)]
pub struct ParseError {
    name: String,
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error parsing {}, token/variable/command not recognised",
            self.name
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Variable {
    Rotation,
    ColorR,
    ColorG,
    ColorB,
    TurningAngle,
    Step,
}

// Parsing is made with Rust std FromStr trait
impl FromStr for Variable {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "rotation" => Ok(Variable::Rotation),
            "color_r" => Ok(Variable::ColorR),
            "color_g" => Ok(Variable::ColorG),
            "color_b" => Ok(Variable::ColorB),
            "turning_angle" => Ok(Variable::TurningAngle),
            "step" => Ok(Variable::Step),
            _ => Err(ParseError {
                name: text.to_string(),
            }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Variable(Variable),
    Number(f64),
}

// Parsing is made with Rust std FromStr trait
impl FromStr for Token {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // Try to parse the number as a f64
        // And if it fails try to parse it as a Variable
        Ok(match text.parse::<f64>() {
            Ok(n) => Token::Number(n),
            Err(_) => Token::Variable(text.parse()?),
        })
    }
}

#[derive(Debug)]
pub enum Command {
    Forward,
    ClockWise,
    CounterClockWise,
    PushStack,
    PopStack,

    Add(Variable, Token),
    Multiply(Variable, Token),
    Set(Variable, Token),
}

// Parsing is made with Rust std FromStr trait
impl FromStr for Command {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<&str> = text.split_whitespace().collect();
        let command = parsed[0];
        match command {
            "forward" => Ok(Command::Forward),
            "clockwise" => Ok(Command::ClockWise),
            "counterclockwise" => Ok(Command::CounterClockWise),
            "push_stack" => Ok(Command::PushStack),
            "pop_stack" => Ok(Command::PopStack),
            "add" => {
                let left = parsed[1].parse()?;
                let right = parsed[2].parse()?;
                Ok(Command::Add(left, right))
            }
            "multiply" => {
                let left = parsed[1].parse()?;
                let right = parsed[2].parse()?;
                Ok(Command::Multiply(left, right))
            }
            "set" => {
                let left = parsed[1].parse()?;
                let right = parsed[2].parse()?;
                Ok(Command::Set(left, right))
            }
            _ => Err(ParseError {
                name: command.to_string(),
            }),
        }
    }
}

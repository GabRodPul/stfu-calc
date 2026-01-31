#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum CalcKey {
    // Characters
    NumZero,
    NumOne,
    NumTwo,
    NumThree,
    NumFour,
    NumFive,
    NumSix,
    NumSeven,
    NumEight,
    NumNine,
    Add,
    Sub,
    Mul,
    Div,
    Dec,

    // Commands
    Eq,
    CAll,
    CEnt,
}

impl CalcKey {
    pub const ROWS: [&'static [Self]; 4] = [
            &[
                CalcKey::NumSeven,
                CalcKey::NumEight,
                CalcKey::NumNine,
                CalcKey::CEnt,
                CalcKey::CAll,
            ],
            &[
                CalcKey::NumFour,
                CalcKey::NumFive,
                CalcKey::NumSix,
                CalcKey::Mul,
                CalcKey::Div,
            ],
            &[
                CalcKey::NumOne,
                CalcKey::NumTwo,
                CalcKey::NumThree,
                CalcKey::Add,
                CalcKey::Sub,
            ],
            &[
                CalcKey::NumZero,
                CalcKey::Dec,
                CalcKey::Eq
            ],
        ];
}

impl From<CalcKey> for char {
    fn from(key: CalcKey) -> Self {
        match key {
            CalcKey::NumZero => '0',
            CalcKey::NumOne => '1',
            CalcKey::NumTwo => '2',
            CalcKey::NumThree => '3',
            CalcKey::NumFour => '4',
            CalcKey::NumFive => '5',
            CalcKey::NumSix => '6',
            CalcKey::NumSeven => '7',
            CalcKey::NumEight => '8',
            CalcKey::NumNine => '9',
            CalcKey::Add => '+',
            CalcKey::Sub => '-',
            CalcKey::Mul => '*',
            CalcKey::Div => '/',
            CalcKey::Dec => '.',
            CalcKey::Eq => '=',

            _ => unreachable!(), //    CalcKey::CAll     => 'a',
                                 //    CalcKey::CEnt     => 'e',
        }
    }
}

impl TryFrom<char> for CalcKey {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0'      => Ok(CalcKey::NumZero),
            '1'      => Ok(CalcKey::NumOne),
            '2'      => Ok(CalcKey::NumTwo),
            '3'      => Ok(CalcKey::NumThree),
            '4'      => Ok(CalcKey::NumFour),
            '5'      => Ok(CalcKey::NumFive),
            '6'      => Ok(CalcKey::NumSix),
            '7'      => Ok(CalcKey::NumSeven),
            '8'      => Ok(CalcKey::NumEight),
            '9'      => Ok(CalcKey::NumNine),
            '+'      => Ok(CalcKey::Add),
            '-'      => Ok(CalcKey::Sub),
            '*'      => Ok(CalcKey::Mul),
            '/'      => Ok(CalcKey::Div),
            '.'      => Ok(CalcKey::Dec),
            '='      => Ok(CalcKey::Eq),
            '\u{08}' => Ok(CalcKey::CEnt), // Backspace
            '\u{27}' => Ok(CalcKey::CAll), // Escape

            _       => Err(CalcBuf::SYNTAX_ERR)
        }
    }
}

impl ToString for CalcKey {
    fn to_string(&self) -> String {
        match *self {
            Self::CAll => "CA".to_string(),
            Self::CEnt => "CE".to_string(),
            _ => char::from(*self).to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CalcBuf(pub String);

impl CalcBuf {
    const SYNTAX_ERR: &str = "Syntax ERROR";
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn push(&mut self, k: CalcKey) -> &mut Self {
        match k {
            CalcKey::Eq => {
                println!("{:?}", eval::eval(&self.0));
                self.0.replace_range(0.., &eval::eval(&self.0).map_or(Self::SYNTAX_ERR.to_string(), |r| r.to_string()));
            }
            CalcKey::CAll => {
                self.0.clear();
                self.0.push('0');
            },
            CalcKey::CEnt => if self.0.len() == 1 { 
                    self.0.replace_range(0..1, "0");
                } else {
                    self.0.pop();
                }

            _ => if self.0.len() == 1 
                 && self.0.get(0..1) == Some("0") { 
                    self.0.replace_range(0..1, &k.to_string());
                } else {
                    if self.0 == Self::SYNTAX_ERR {
                        self.0.clear();
                    }

                    self.0.push(k.into());
                }
        };

        self
    }
}


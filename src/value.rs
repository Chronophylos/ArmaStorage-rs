use serde::{Deserialize, Serialize};

// https://community.bistudio.com/wiki/Side
#[derive(Debug, Deserialize, Serialize)]
pub enum Side {
    Blufor = 1,
    Opfor = 0,
    Independent = 2,
    Civilian = 3,
    AmbientLife = 9,
    Empty = 8,
    Friendly = 6,
    Enemy = 5,
    Unknown = 4,
    Logic = 7,
}

impl Side {
    pub fn as_sqf(&self) -> String {
        match *self {
            Side::Blufor => String::from("blufor"),
            Side::Opfor => String::from("opfor"),
            Side::Independent => String::from("independent"),
            Side::Civilian => String::from("civilian"),
            Side::AmbientLife => String::from("sideAmbientLife"),
            Side::Empty => String::from("sideEmpty"),
            Side::Friendly => String::from("sideFriendly"),
            Side::Enemy => String::from("sideEnemy"),
            Side::Unknown => String::from("sideUnknown"),
            Side::Logic => String::from("sideLogic"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Value {
    Array(Vec<Value>),
    Boolean(bool),
    Group(String), // TODO
    Number(f32),
    Object(String), // TODO
    Side(Side),
    String(String),
    Code(String),           // TODO
    Config(String),         // TODO
    Control(String),        // TODO
    Display(String),        // TODO
    Location(String),       // TODO
    ScriptHandle(String),   // TODO
    StructuredText(String), // TODO
    DiaryRecord(String),    // TODO
    Task(String),           // TODO
    TeamMember(String),     // TODO
    Namespace(String),      // TODO
    Void,
}

impl Value {
    pub fn as_sqf(&self) -> String {
        match self {
            Value::Array(array) => {
                let array: Vec<String> = array.iter().map(|value| value.as_sqf()).collect();

                format!("[{}]", array.join(", "))
            }
            Value::Boolean(boolean) => format!("{}", boolean),
            Value::Group(group) => format!("{}", group),
            Value::Number(number) => format!("{}", number),
            Value::Object(object) => format!("{}", object),
            Value::Side(side) => side.as_sqf(),
            Value::String(string) => format!("\"{}\"", string),
            Value::Code(code) => format!("{}", code),
            Value::Config(config) => format!("{}", config),
            Value::Control(control) => format!("{}", control),
            Value::Display(display) => format!("{}", display),
            Value::Location(location) => format!("{}", location),
            Value::ScriptHandle(script_handle) => format!("{}", script_handle),
            Value::StructuredText(strucured_text) => format!("{}", strucured_text),
            Value::DiaryRecord(diary_record) => format!("{}", diary_record),
            Value::Task(task) => format!("{}", task),
            Value::TeamMember(team_member) => format!("{}", team_member),
            Value::Namespace(namespace) => format!("{}", namespace),
            Value::Void => String::new(),
        }
    }
}

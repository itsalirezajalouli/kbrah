use std::{self, time, collections::HashMap};

// I guess we're using enums as states 
pub enum CurrentScreen {
    Main,
    Editing,
    Stats,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub cursor: char,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    // if no editing is happening => None
    pub currently_editing: Option<CurrentlyEditing>, 
    pub current_text: String,
    pub original_text: String,  
    pub wrong: bool,
    pub mistakes: u16,
    pub accuracy: u16,
    pub right_finger_map: HashMap<char, char>,
    pub left_finger_map: HashMap<char, char>,
    pub right_nums: Vec<char>,
    pub left_nums: Vec<char>,
    pub rights: Vec<char>,
    pub lefts: Vec<char>,
    pub start_time: Option<time::Instant>,
    pub time: Option<time::Duration>,
    pub wpm: Option<u16>
}

impl App {
    /// Creates a new `App` that holds states and temp inputs.
    pub fn new() -> App {
        App { key_input: String::new(),
              value_input: String::new(),
              cursor: ' ',
              pairs: HashMap::new(),
              current_screen: CurrentScreen::Main,
              currently_editing: None,
              original_text: "one time leave part out who take all same ask should".to_string(),
              current_text: "one time leave part out who take all same ask should".to_string(),
              wrong: false,
              mistakes: 0,
              accuracy: 100,
              right_finger_map: HashMap::new(),
              left_finger_map: HashMap::new(),
              right_nums: vec![],
              left_nums: vec![],
              rights: vec![],
              lefts: vec![],
              start_time: None,
              time: None,
              wpm: None,
        }
    }

    pub fn edit_text(&mut self, text: &str) {
        self.current_text = text.to_string();
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Value)}
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Key)}
            }
        } else {self.currently_editing = Some(CurrentlyEditing::Key)};
    }

    pub fn go_stats(&mut self) {
        self.current_screen = CurrentScreen::Stats;
    }

    pub fn update_accuracy(&mut self) {
        let ori_t: Vec<char> = self.original_text.chars().collect();
        let ori_len = ori_t.len() as f32;
        let correct = (ori_len - (self.mistakes as f32)) as f32;
        self.accuracy = ((correct / ori_len) * 100.0) as u16;
    }

    pub fn update_wpm(&mut self) {
        self.time = Some(self.start_time.unwrap().elapsed());
        let time = self.time.unwrap().as_secs_f64();
        let cnums: f64 = self.key_input.len() as f64;
        let numinator = (cnums - (self.mistakes as f64)) * 60.0;
        self.wpm = Some((numinator / (5.0 * time)) as u16);
    }

    pub fn reset(&mut self) {
        self.wrong = false;
        self.mistakes = 0;
        self.accuracy= 100;
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
        self.edit_text(&self.original_text.clone());
        self.lefts[self.key_input.len()] = ' ';
        self.rights[self.key_input.len()] = ' ';
        self.key_input.clear();
        self.wpm = Some(0);
    }

    pub fn add_map(&mut self) {
        self.left_finger_map.insert('a', '5');
        self.left_finger_map.insert('q', '5');
        self.left_finger_map.insert('z', '5');
        self.left_finger_map.insert('w', '4');
        self.left_finger_map.insert('s', '4');
        self.left_finger_map.insert('x', '4');
        self.left_finger_map.insert('e', '3');
        self.left_finger_map.insert('d', '3');
        self.left_finger_map.insert('c', '3');
        self.left_finger_map.insert('r', '2');
        self.left_finger_map.insert('f', '2');
        self.left_finger_map.insert('v', '2');
        self.left_finger_map.insert('t', '2');
        self.left_finger_map.insert('g', '2');
        self.left_finger_map.insert('b', '2');
        self.left_finger_map.insert(' ', ' ');
        self.right_finger_map.insert('y', '2');
        self.right_finger_map.insert('h', '2');
        self.right_finger_map.insert('n', '2');
        self.right_finger_map.insert('u', '2');
        self.right_finger_map.insert('j', '2');
        self.right_finger_map.insert('m', '2');
        self.right_finger_map.insert('i', '3');
        self.right_finger_map.insert('k', '3');
        self.right_finger_map.insert(',', '3');
        self.right_finger_map.insert('o', '4');
        self.right_finger_map.insert('l', '4');
        self.right_finger_map.insert('.', '4');
        self.right_finger_map.insert('p', '5');
        self.right_finger_map.insert(';', '5');
        self.right_finger_map.insert('/', '5');
        let chars: Vec<char> = self.original_text.chars().collect();
        let mut right_nums = vec![];
        let mut left_nums = vec![];
        for c in chars {
            if self.right_finger_map.contains_key(&c) {
                right_nums.push(self.right_finger_map[&c]);
                left_nums.push(' ');
                self.rights.push(' ');
                self.lefts.push(' ');
            }
            if self.left_finger_map.contains_key(&c) {
                left_nums.push(self.left_finger_map[&c]);
                right_nums.push(' ');
                self.rights.push(' ');
                self.lefts.push(' ');
            } 
        }
        self.right_nums= right_nums;
        self.left_nums= left_nums;
    }

}

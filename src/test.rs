// use crossterm::event::{self, Event, KeyCode, KeyEvent};
// use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

// use std::time::Duration;

// enum MovementKeys {
//     W,
//     S,
//     A,
//     D,
// }

// struct Player {
//     name: String,
//     health: isize,
//     walkspeed: isize,
//     x: isize,
//     y: isize,
// }

// impl Player {
//     fn move_player(&mut self, key: MovementKeys) {
//         match key {
//             MovementKeys::W => self.y -= 1,
//             MovementKeys::S => self.y += 1,
//             MovementKeys::A => self.x -= 1,
//             MovementKeys::D => self.x += 1,
//         }
//         println!("{} moved! newPosition: (X: {}, Y: {})", self.name, self.x, self.y)
//     }
// }

// fn main() {
//     let mut player: Player = Player {
//         name: String::from("Eclipse"),
//         health: 100,
//         walkspeed: 100,
//         x: 0,
//         y: 0,
//     };

//     enable_raw_mode().unwrap();

//     loop {
//         if event::poll(Duration::from_millis(100)).unwrap() {
//             if let Ok(Event::Key(KeyEvent {code, ..})) = event::read() {
//                 let movement = match code {
//                     KeyCode::Char('w') | KeyCode::Char('W') => Some(MovementKeys::W),
//                     KeyCode::Char('s') | KeyCode::Char('S') => Some(MovementKeys::S),
//                     KeyCode::Char('a') | KeyCode::Char('A') => Some(MovementKeys::A),
//                     KeyCode::Char('d') | KeyCode::Char('D') => Some(MovementKeys::D),
//                     KeyCode::Esc => {
//                         println!("Exiting game...\r");
//                         break;
//                     }
//                     _ => None,
//                 };

//                 if let Some(direction) = movement {
//                     player.move_player(direction)
//                 }
//             }
//         }
//     }

//     disable_raw_mode().unwrap()
// }
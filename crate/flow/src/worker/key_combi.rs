use std::time::Duration;

use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse};
use flow_model::KeyCombiAtrr;
use tower::enigo::TowerEnigo;
use tracing::debug;

use crate::{into_key, sync_sleep, FlowExecutor};

pub(crate) struct KeyCombiWorker;

impl KeyCombiWorker {
    pub fn do_work(executor: &FlowExecutor, attr: &KeyCombiAtrr) -> bool {
        executor.do_with_enigo(|mut enigo| {
            if executor.is_backend() {
                let control_1 = into_key(&attr.control_1);
                let control_2 = (attr.control_2.len() > 0).then(|| into_key(&attr.control_2));
                let key = (attr.key.len() > 0).then(|| into_key(&attr.key));
                return enigo
                    .backend()
                    .unwrap()
                    .key_combi(control_1, control_2, key)
                    .is_ok();
            }
            let (x, y) = executor.transfer_xy_with_offset((attr.x, attr.y), Coordinate::Abs);
            debug!("key_combi:do_work {:?}", (x, y));

            let do_key = |enigo: &mut TowerEnigo, key: &str, direct: Direction| {
                if key.len() > 0 {
                    enigo.key(into_key(key), direct).unwrap();
                    sync_sleep(Duration::from_millis(10));
                }
            };

            let do_char = |enigo: &mut TowerEnigo, char: &str, direct: Direction| {
                if char.len() > 0 {
                    let char = char.chars().next().unwrap();
                    enigo.key(Key::Unicode(char), direct).unwrap();
                    sync_sleep(Duration::from_millis(10));
                }
            };
            enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Direction::Click).unwrap();
            sync_sleep(Duration::from_millis(10));

            do_key(&mut enigo, &attr.control_1, Direction::Press);
            do_key(&mut enigo, &attr.control_2, Direction::Press);
            do_char(&mut enigo, &attr.key, Direction::Click);
            do_key(&mut enigo, &attr.control_1, Direction::Release);
            do_key(&mut enigo, &attr.control_2, Direction::Release);
            true
        })
    }
}

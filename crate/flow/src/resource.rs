use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use enigo::{Enigo, Settings};
pub struct FLowResource {
    enigo_lock: Mutex<bool>,
    enigo: Mutex<Enigo>,
    //pid, is_running
    pids: Mutex<HashMap<u32, bool>>,
}

impl FLowResource {
    pub fn new() -> Self {
        let enigo_lock = Mutex::new(false);
        let enigo = Mutex::new(Enigo::new(&Settings::default()).unwrap());
        Self {
            enigo,
            enigo_lock,
            pids: Mutex::new(HashMap::new()),
        }
    }

    ///判断停止运行标记,并清除标记,返回是否停止
    pub fn is_stop(&self, pid: u32) -> bool {
        let mut pids = self.pids.lock().unwrap();
        if pids.contains_key(&pid) {
            let is_running = pids.get(&pid).cloned().unwrap();
            if !is_running {
                pids.remove(&pid);
            }
            !is_running
        } else {
            true
        }
    }
    ///设置停止运行标记,外部停止执行
    pub fn set_stop(&self, pid: u32) {
        let mut pids = self.pids.lock().unwrap();
        if let Some(is_running) = pids.get_mut(&pid) {
            if *is_running {
                *is_running = false;
            }
        }
    }
    ///标记pid为运行中
    pub(crate) fn set_running(&self, pid: u32) {
        let mut pids = self.pids.lock().unwrap();
        pids.insert(pid, true);
    }
    ///运行完成后,清除标记
    pub(crate) fn set_stopped(&self, pid: u32) {
        let mut pids = self.pids.lock().unwrap();
        pids.remove(&pid);
    }
}

impl FLowResource {
    pub(crate) fn lock_enigo(&self) -> MutexGuard<bool> {
        self.enigo_lock.lock().unwrap()
    }

    pub(crate) fn do_with_enigo<F, T>(&self, f: F, with_lock: bool) -> T
    where
        F: Fn() -> T,
    {
        if with_lock {
            let _enigo_lock = self.lock_enigo();
            f()
        } else {
            f()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env, thread,
        time::{Duration, SystemTime},
    };

    use super::*;

    #[test]
    fn it_works() {
        let now = SystemTime::now();
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        let wait_time = Duration::from_secs(2);

        // thread::sleep(wait_time);

        // enigo.scroll(-2, Axis::Vertical).unwrap();
        // thread::sleep(wait_time);
        println!("{:?}", now.elapsed());
    }
}

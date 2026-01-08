pub enum FunctionId {
    Test,
    Record,
    Login,
    Script,
    CheckPoint,
}

impl FunctionId {
    fn method(&self) -> String {
        match self {
            Self::Test => "test".to_string(),
            Self::Record => "record".to_string(),
            Self::Login => "login".to_string(),
            Self::Script => "script".to_string(),
            Self::CheckPoint => "check_point".into(),
        }
    }
    fn js(&self, arg: &str) -> String {
        format!("window.__{}(\"{}\");", self.method(), arg)
    }
    fn js_2(&self, arg1: &str, arg2: &str) -> String {
        format!("window.__{}(\"{}\",\"{}\");", self.method(), arg1, arg2)
    }
    fn js_with_pid(&self, pid: u32, arg: &str) -> String {
        format!("window.__{}_{}(\"{}\");", self.method(), pid, arg)
    }
    fn js_2_with_pid(&self, pid: u32, arg1: &str, arg2: &str) -> String {
        format!(
            "window.__{}_{}(\"{}\",\"{}\");",
            self.method(),
            pid,
            arg1,
            arg2
        )
    }
}

impl FunctionId {
    pub fn call_func(&self, webview: &tauri::WebviewWindow, arg: &str) {
        webview.eval(self.js(arg).as_str()).unwrap();
    }

    pub fn call_func_2(&self, webview: &tauri::WebviewWindow, arg1: &str, arg2: &str) {
        webview.eval(self.js_2(arg1, arg2).as_str()).unwrap();
    }

    pub fn call_func_with_pid(&self, webview: &tauri::WebviewWindow, pid: u32, arg: &str) {
        webview.eval(self.js_with_pid(pid, arg).as_str()).unwrap();
    }

    pub fn call_func_2_with_pid(
        &self,
        webview: &tauri::WebviewWindow,
        pid: u32,
        arg1: &str,
        arg2: &str,
    ) {
        webview
            .eval(self.js_2_with_pid(pid, arg1, arg2).as_str())
            .unwrap();
    }
}

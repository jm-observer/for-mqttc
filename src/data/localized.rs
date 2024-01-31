pub fn get_locale() -> Locale {
    todo!()
    // let current_locale = Application::get_locale();
    // debug!("{}", current_locale);
    // if current_locale.as_str() == "zh-CN" {
    //     Locale::default_zh_cn()
    // } else {
    //     Locale::default()
    // }
}

#[derive(Clone, Debug)]
pub struct Locale {
    pub save: &'static str,
    pub disconnect: &'static str,
    pub reconnect: &'static str,
    pub connect: &'static str,
    pub close: &'static str,
    pub subscribe: &'static str,
    pub publish: &'static str,
    pub copy_github: &'static str,
    pub open: &'static str,
}

impl Locale {
    fn default_zh_cn() -> Self {
        Self {
            save: "保存",
            disconnect: "断开",
            reconnect: "重连",
            connect: "连接",
            close: "关闭",
            subscribe: "订阅",
            publish: "发布",
            copy_github: "复制github",
            open: "打开",
        }
    }
    fn default() -> Self {
        Self {
            save: "save",
            disconnect: "disconnect",
            reconnect: "reconnect",
            connect: "connect",
            close: "close",
            subscribe: "subscribe",
            publish: "publish",
            copy_github: "copy github addr",
            open: "open",
        }
    }
}

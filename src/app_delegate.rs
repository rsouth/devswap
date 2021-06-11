use crate::data::AppData;
use crate::{command_processor, ESC_HOT_KEY, EXEC_CMD, GLOBAL_HOT_KEY};
use druid::commands::CONFIGURE_WINDOW;
use druid::{
    AppDelegate, Command, DelegateCtx, Env, Event, Handled, HotKey, KbKey, Point, Target,
    WindowConfig, WindowId,
};

pub(crate) struct Delegate {
    window_id: WindowId,
    hot_key_esc: HotKey,
}
impl Delegate {
    pub fn new(winid: WindowId) -> Self {
        Delegate {
            window_id: winid,
            hot_key_esc: HotKey::new(None, KbKey::Escape),
        }
    }
}
impl AppDelegate<AppData> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        _data: &mut AppData,
        _env: &Env,
    ) -> Option<Event> {
        // println!("Event: {:?}", event);
        match &event {
            Event::KeyDown(key) if self.hot_key_esc.matches(key) => {
                _ctx.submit_command(Command::from(ESC_HOT_KEY));
            }
            _ => (),
        };

        Some(event)
    }

    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if let Some(number) = cmd.get(GLOBAL_HOT_KEY) {
            println!("Event sink got toggle visible event");
            if data.toggle_window() {
                println!("Showing window {:?}", number);
                let wc = WindowConfig::default().set_position(Point { x: -0.0, y: 0.0 });
                _ctx.submit_command(CONFIGURE_WINDOW.with(wc).to(*number));
            } else {
                println!("Hiding window {:?}", number);
                let wc = WindowConfig::default().set_position(Point {
                    x: -10000.0,
                    y: 100.0,
                });
                _ctx.submit_command(CONFIGURE_WINDOW.with(wc).to(*number));
            }

            Handled::Yes
        } else if let Some(payload) = cmd.get(EXEC_CMD) {
            let command = match payload {
                Some(p) => {
                    if data.command_text.len() == 0 {
                        Some(command_processor::Command::SingleChar(p.to_string()))
                    } else {
                        None
                    }
                }
                None => Some(command_processor::Command::ColonPrefixed(
                    data.command_text.to_string(),
                )),
            };

            command
                .map(|com| {
                    println!("Execute Command [{:?}]", com);
                    data.command_text.clear();
                    command_processor::process(_ctx, com, self.window_id);
                    Handled::Yes
                })
                .unwrap_or(Handled::No)
        } else {
            Handled::No
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _data: &mut AppData,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window added, {:?}", id);
    }
}

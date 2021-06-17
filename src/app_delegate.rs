use druid::commands::CONFIGURE_WINDOW;
use druid::{
    AppDelegate, Command, DelegateCtx, Env, Event, Handled, HotKey, KbKey, Point, Selector, Target,
    WindowConfig, WindowId,
};

use crate::command;
use crate::config::{ProjectSettings, Settings};
use crate::data::AppState;

pub const GLOBAL_HOT_KEY: Selector<WindowId> = Selector::new("dev.untitled1.toggle-window-hotkey");
pub const ESC_HOT_KEY: Selector = Selector::new("dev.untitled1.esc-hotkey");
pub const EXEC_CMD: Selector<Option<String>> = Selector::new("dev.untitled1.execute-command");

pub(crate) struct Delegate {
    _window_id: WindowId,
    hot_key_esc: HotKey,
}
impl Delegate {
    pub fn new(winid: WindowId) -> Self {
        Delegate {
            _window_id: winid,
            hot_key_esc: HotKey::new(None, KbKey::Escape),
        }
    }
}
impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        _data: &mut AppState,
        _env: &Env,
    ) -> Option<Event> {
        // println!("Event: {:?}", event);
        match &event {
            Event::KeyDown(key) if self.hot_key_esc.matches(key) => {
                ctx.submit_command(Command::from(ESC_HOT_KEY));
            }
            _ => (),
        };

        Some(event)
    }

    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(number) = cmd.get(GLOBAL_HOT_KEY) {
            println!("Event sink got toggle visible event");
            if data.toggle_window() {
                println!("Showing window {:?}", number);
                let wc = WindowConfig::default().set_position(Point { x: -0.0, y: 0.0 });
                ctx.submit_command(CONFIGURE_WINDOW.with(wc).to(*number));
            } else {
                println!("Hiding window {:?}", number);
                let wc = WindowConfig::default().set_position(Point {
                    x: -10000.0,
                    y: 100.0,
                });
                ctx.submit_command(CONFIGURE_WINDOW.with(wc).to(*number));
            }
            Handled::Yes
        } else if let Some(payload) = cmd.get(EXEC_CMD) {
            data.command_text.clear();
            match payload {
                Some(p) => {
                    data.add_to_command_history(p);
                    return match command::resolve(p.to_string(), ctx, data) {
                        Ok(mut ec) => {
                            println!("Executing EC: {}", p);
                            ec.execute().map_or(Handled::No, |d| {
                                println!("Command executed in {}\u{3bc}s", d);
                                Handled::Yes
                            })
                        }
                        Err(e) => {
                            println!("Error parsing command {} with error {}", p, e);
                            Handled::No
                        }
                    };
                }
                None => Handled::No,
            }
        } else {
            Handled::No
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window added, {:?}", id);
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        let data = data.get_settings_mut();
        data.update("devswap".to_string(), ProjectSettings::default());
        println!("{:?}", data);
        // let ll = *data.borrow();
        Settings::save(data);
    }
}

use crate::app_delegate::{EXEC_CMD, TOGGLE_MODE_CMD};
use crate::data::Mode;
use druid::widget::{Controller, TextBox};
use druid::{Env, Event, EventCtx, KbKey, KeyEvent, Modifiers, Target, Widget};

pub(crate) struct TextAreaController;
impl Default for TextAreaController {
    fn default() -> Self {
        TextAreaController {}
    }
}

impl Controller<String, TextBox<String>> for TextAreaController {
    fn event(
        &mut self,
        child: &mut TextBox<String>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut String,
        env: &Env,
    ) {
        match &event {
            Event::KeyDown(key_event) if is_ctrl_s(&key_event) => {
                println!("command for ctrl-s");
                ctx.submit_command(EXEC_CMD.with(Some(":w".to_string())).to(Target::Auto));
                ctx.set_handled();
            }
            Event::Command(c) => {
                println!("...");
                if let Some(cc) = c.get(TOGGLE_MODE_CMD) {
                    match cc {
                        Mode::Insert => {
                            println!("switching to insert mode.");
                            ctx.request_focus();
                        }
                        _ => {}
                    }
                }
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}

fn is_ctrl_s(key_event: &KeyEvent) -> bool {
    key_event.key == KbKey::Character("s".to_string())
        && key_event.mods.contains(Modifiers::CONTROL)
}

use crate::{ESC_HOT_KEY, EXEC_CMD};
use druid::widget::{Controller, TextBox};
use druid::{Env, Event, EventCtx, KbKey, Target, Widget};

pub(crate) struct CommandBoxController;
impl Default for CommandBoxController {
    fn default() -> Self {
        CommandBoxController {}
    }
}

impl Controller<String, TextBox<String>> for CommandBoxController {
    fn event(
        &mut self,
        child: &mut TextBox<String>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut String,
        env: &Env,
    ) {
        match &event {
            // Esc to switch to Command mode
            Event::Command(x) if x.is(ESC_HOT_KEY) => {
                println!("Entering Command mode");
                ctx.request_focus();
            }
            // Enter to execute Command
            Event::KeyDown(key_event) if key_event.key == KbKey::Enter => {
                println!("command_box event(Enter) -> {:?}", key_event);
                ctx.submit_command(EXEC_CMD.with(Some((*data).to_string())).to(Target::Auto));
                ctx.set_handled();
            }

            // Insert command
            Event::KeyDown(key_event) if key_event.key == KbKey::Character("i".to_string()) => {
                if data.is_empty() {
                    println!("command box event(i) -> {:?}", key_event);
                    ctx.submit_command(EXEC_CMD.with(Some("i".to_string())).to(Target::Auto));
                    ctx.set_handled();
                }
            }

            _ => child.event(ctx, event, data, env),
        }
    }
}

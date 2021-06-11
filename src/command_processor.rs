use druid::commands::SHOW_OPEN_PANEL;
use druid::{DelegateCtx, FileDialogOptions, WindowId};

pub(crate) fn process(ctx: &mut DelegateCtx, command: String, window_id: WindowId) {
    println!("Processing command [{}]", command);

    if !command.starts_with(":") {
        return;
    }

    // let parts = command.split_once(char::is_whitespace);
    let mut cmd_iter = command.split_whitespace();
    match cmd_iter.next() {
        // new project
        Some(":n") | Some(":new") => {
            let mut args = cmd_iter.fold(String::new(), |acc, s| acc + s + "_");
            args.pop();
            args.push_str(".devswap");

            println!("New project [{}]", args);
        }
        // open project ...
        Some(":o") | Some(":open") => {
            let opts = FileDialogOptions::default();
            ctx.submit_command(SHOW_OPEN_PANEL.with(opts).to(window_id));
        }
        None => {}
        _ => {}
    }
}

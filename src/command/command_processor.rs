// use crate::command::ccmmmdd::CommandSelector;
// use crate::config::Settings;
// use crate::data::AppData;
// use druid::commands::SHOW_OPEN_PANEL;
// use druid::{DelegateCtx, FileDialogOptions, WindowId};
//
// pub fn process(
//     ctx: &mut DelegateCtx,
//     command: &CommandSelector,
//     window_id: WindowId,
//     data: &AppData,
// ) {
//     println!("Processing command [{:?}]", command);
//     match command {
//         CommandSelector::ColonPrefixed(_) => {
//             process_colon_prefix_command(ctx, command, window_id, data);
//         }
//         CommandSelector::SingleChar(_) => process_single_char_command(ctx, command, window_id),
//     }
// }
//
// /// Process single-character commands
// /// e.g. `i` to switch to `insert mode`
// ///
// fn process_single_char_command(
//     _ctx: &mut DelegateCtx,
//     command: &CommandSelector,
//     _window_id: WindowId,
// ) {
//     println!("p_s_c_c {:?}", command);
// }
//
// /// Process colon-prefixed (vim-style) commands
// /// e.g. `:n` or `:new` to create a new project
// ///
// fn process_colon_prefix_command(
//     ctx: &mut DelegateCtx,
//     command: &CommandSelector,
//     window_id: WindowId,
//     data: &AppData,
// ) {
//     if let CommandSelector::ColonPrefixed(command) = command {
//         // let parts = command.split_once(char::is_whitespace);
//         let mut cmd_iter = command.split_whitespace();
//         match cmd_iter.next() {
//             //
//             // new project
//             Some(":n" | ":new") => {
//                 let mut args = cmd_iter.fold(String::new(), |acc, s| acc + s + "_");
//                 args.pop();
//                 args.push_str(".devswap");
//
//                 println!("New project [{}]", args);
//             }
//             //
//             // open project ...
//             Some(":o" | ":open") => {
//                 let opts = FileDialogOptions::default();
//                 ctx.submit_command(SHOW_OPEN_PANEL.with(opts).to(window_id));
//             }
//             //
//             // save settings ...
//             Some(":ws") => {
//                 Settings::save(data.get_settings());
//             }
//             //
//             // ...
//             _ => {}
//         }
//     }
// }

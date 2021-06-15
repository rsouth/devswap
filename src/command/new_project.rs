use druid::DelegateCtx;

use crate::command::{Executable, ExecutionError, HasArguments};
use crate::data;
use crate::data::AppState;

pub(crate) struct NewProject<'r> {
    file_name: String,
    args: String,
    _app_data: &'r AppState,
    _ctx: &'r DelegateCtx<'r>,
}

impl<'r> NewProject<'r> {
    pub fn new(
        file_name: &str,
        args: String,
        app_data: &'r mut data::AppState,
        ctx: &'r druid::DelegateCtx<'r>,
    ) -> Self {
        NewProject {
            file_name: file_name.into(),
            args,
            _app_data: app_data,
            _ctx: ctx,
        }
    }
}

impl HasArguments for NewProject<'_> {
    fn args(&self) -> String {
        self.args.clone()
    }

    fn process_args(&mut self) {
        let mut my_args = self
            .args()
            .split_whitespace()
            .skip(1)
            .fold(String::new(), |acc, s| acc + s + "_");

        // todo don't allow empty filenames..
        my_args.pop();
        my_args.push_str(".devswap");

        self.file_name = my_args;
    }
}

impl Executable for NewProject<'_> {
    fn execute(&mut self) -> Result<u128, ExecutionError> {
        self.process_args();
        println!("New project [{:#?}]", self.file_name);

        Err(ExecutionError)
    }
}

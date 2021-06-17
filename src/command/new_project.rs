use druid::DelegateCtx;

use crate::command::{Executable, ExecutionError, HasArguments};
use crate::data;
use crate::data::AppState;
use crate::document::{DocType, Header};
use std::fs::try_exists;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct NewProject<'r> {
    file_name: String,
    args: String,
    app_data: &'r mut AppState,
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
            app_data: app_data,
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

        match try_exists(Path::new(&self.file_name)) {
            Ok(exists) => {
                let f_name = if exists {
                    // update filename with suffix to make it unique
                    // todo rather than ts, get next non-existant incrementing name (fn_1, fn_2...)
                    if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                        let mut x: String = self.file_name.clone();
                        x.insert(x.len() - ".devswap".len(), '_');
                        x.insert_str(
                            x.len() - ".devswap".len(),
                            duration.as_millis().to_string().as_str(),
                        );
                        Some(x)
                    } else {
                        None
                    }
                } else {
                    Some(self.file_name.clone())
                };

                match f_name {
                    None => Err(ExecutionError),
                    Some(n) => {
                        // create here
                        // self.file_name.push_str(".devswap");
                        let pth = Path::new(&n);
                        if let Ok(ss) = std::fs::File::create(pth) {
                            println!("Created file {:?}", ss);

                            // todo  ...  :vomit:
                            let os_string = pth.as_os_str().to_os_string();
                            let path_string = os_string.to_str().unwrap_or("").to_string();
                            let hdr = Header::new(DocType::FileBased(path_string));

                            // let x = self._app_data;

                            self.app_data.push_doc(&hdr);

                            Ok(123)
                        } else {
                            Err(ExecutionError)
                        }
                    }
                }
            }
            Err(_) => Err(ExecutionError),
        }

        // std::fs::File::create();

        // std::fs::File::create()
    }
}

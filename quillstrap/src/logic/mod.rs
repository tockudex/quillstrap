use crate::{logic::{auto_mode::auto_main, manual_mode::manual_main}, prelude::*};

pub mod auto_mode;
pub mod manual_mode;

pub fn choose_mode(options: Options) {
    if options.args.manual_mode {
        manual_main(options);
    } else {
        auto_main(options);
    }
}
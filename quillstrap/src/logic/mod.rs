use crate::{logic::{auto_mode::auto_main, manual_mode::manual_main}, prelude::*};

pub mod auto_mode;
pub mod manual_mode;

pub fn choose_mode(args: Args) {
    if args.manual_mode {
        manual_main(args);
    } else {
        auto_main(args);
    }
}
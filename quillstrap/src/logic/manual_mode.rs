use crate::{prelude::*, things::get_thing_by_name};

pub fn manual_main(options: Options) {
    debug!("Manual mode selected...");

    things_setup();
    // First, get
    for name in options.clone().args.get {
        let impl_name = get_thing_by_name(&name, &options.things);
        info!("Managing repo for {}", name);
        git_get_manage(&impl_name, &options);

        // TODO move git get manage to get trait, pass options there

        info!("Get for {}", name);
        impl_name
            .get()
            .expect(&format!("Failed to get for {}", name));
    }
}

use crate::{common::commands::run_command, prelude::*};

// We use commands instead of library because for support for ssh key managers, I don't feel like reinventing the wheel for no benefit

// Here we assume we are in the directory where the git repo should be
pub fn git_get_manage(name: &str, _options: &Options) {
    if dir_exists(name) {
        dir_change(name);
        pull(name, _options);
        dir_change("../");
    } else {
        clone(name, _options);
    }
}

// Here we assume we are in the directory already
pub fn pull(name: &str, options: &Options) {
    // Note: no submodules, we don't use submodules, ever!
    let url = assemble_git_link(name, options);
    info!("Pulling existing repo: {}", url);
    run_command(&format!("git pull {}", url), options.config.command_output)
        .expect(&format!("Failed to pull repo: {}", url));
}

pub fn clone(name: &str, options: &Options) {
    let url = assemble_git_link(name, options);
    info!("Clonning repo: {}", url);
    run_command(&format!("git clone {}", url), options.config.command_output)
        .expect(&format!("Failed to clone repo: {}", url));
}

/*
https://github.com/PorQ-Pine/u-boot-pinenote.git
git@github.com:PorQ-Pine/u-boot-pinenote.git
*/
pub fn assemble_git_link(name: &str, options: &Options) -> String {
    let mut link = String::new();
    match options.config.git_link_type {
        crate::config::GitLinkType::Ssh => link.push_str("git@"),
        crate::config::GitLinkType::Https => link.push_str("https://"),
    }

    match options.config.git_platform {
        crate::config::GitPlatform::Gitlab => link.push_str("gitlab.com"),
        crate::config::GitPlatform::Github => link.push_str("github.com"),
    }

    match options.config.git_link_type {
        crate::config::GitLinkType::Ssh => link.push_str(":@"),
        crate::config::GitLinkType::Https => link.push_str("/"),
    }

    link.push_str(&options.config.git_username);
    link.push_str("/");
    link.push_str(name);
    link.push_str(".git");

    link
}

pub trait SetupThing: Copy + Clone + Default {
    fn name() -> &'static str; // By this name, other setup things will specify their dependencies
    // Relative path to this folder's path, created one folder up (../), ends with / (will still be in the repository)
    fn path() -> &'static str;
    // List of deps (depending on arguments, to be built before a target)
    fn deps() -> Vec< &'static str>;
    fn git() -> &'static str; // Git repo name, the link and type (ssh, https) depends on the configuration
    fn get() -> Result<(), String>; // How to get it, also updates it (git pull or else)
    fn clean() -> Result<(), String>; // Cleans all build-related files
    fn build() -> Result<(), String>; // Builds everything
    fn deploy() -> Result<(), String>; // The deployment will be based on a tree of deps, so the deepest things first
    fn run() -> Result<(), String>;
}

#[derive(Clone, Copy, Default)]
struct _ExampleImpl;

impl SetupThing for _ExampleImpl {
    fn name() -> &'static str {
        "example"
    }

    fn path() -> &'static str {
        "example/"
    }
    
    fn deps() -> Vec< &'static str> {
        vec!["example1", "example2"]
    }
    
    fn git() -> &'static str {
        "example_repo_name"
    }
    
    fn get() -> Result<(), String> {
        todo!()
    }

    fn clean() -> Result<(), String> {
        todo!()
    }
    
    fn build() -> Result<(), String> {
        todo!()
    }
    
    fn deploy() -> Result<(), String> {
        todo!()
    }
    
    fn run() -> Result<(), String> {
        todo!()
    }
}
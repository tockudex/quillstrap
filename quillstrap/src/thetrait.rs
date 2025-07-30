pub trait SetupThing {
    fn name() -> &'static str; // By this name, other setup things will specify their dependencies
    // Relative path to this folder's path, created one folder up (../), ends with / (will still be in the repository)
    fn path() -> &'static str;
    // List of deps (depending on arguments, to be built before a target)
    fn deps() -> Vec< &'static str>;
    fn git() -> &'static str; // Git repo name, the link and type (ssh, https) depends on the configuration
    fn clean() -> Result<(), String>; // Cleans all build-related files
    fn build() -> Result<(), String>; // Builds everything
    fn deploy() -> Result<(), String>;
    fn run() -> Result<(), String>;
}

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
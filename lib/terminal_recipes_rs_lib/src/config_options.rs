pub trait ConfigOptions: Clone {
    fn get_opts() -> Vec<String>;
}
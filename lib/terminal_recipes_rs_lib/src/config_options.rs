use std::path::Path;

pub trait ConfigOptions: Clone {
    fn update_cfg(self, opt: &str, val: &str) -> anyhow::Result<Self>;
    fn write_cfg<P: AsRef<Path>>(self, file: P) -> anyhow::Result<()>;
}

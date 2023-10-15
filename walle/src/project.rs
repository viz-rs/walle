use crate::module::Module;

pub struct Project<'a> {
    name: &'a str,
    modules: Vec<Module<'a>>,
}

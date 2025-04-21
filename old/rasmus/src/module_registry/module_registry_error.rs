pub enum ModuleRegistryError {
    UnableToReadModule { path: String },
    ModuleNotRegistered { name: String },
}

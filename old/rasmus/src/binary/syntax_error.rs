pub type ParseResult<T> = Result<T, SyntaxError>;

#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedSectionIdValue,
    ModuleMagicNotFound,
    ModuleVersionNotFound,
    InvalidModuleSection,
    InvalidTypesModuleSection,
    InvalidCodeModuleSection,
    InvalidFuncsModuleSection,
    InvalidImportsModuleSection,
    InvalidTablesModuleSection,
    InvalidMemsModuleSection,
    InvalidGlobalsModuleSection,
    InvalidStartModuleSection,
    InvalidElementSegmentModuleSection,
    InvalidDatasModuleSection,
    InvalidDataCountModuleSection,
    DataCountDoesntMatchDataLen,
}

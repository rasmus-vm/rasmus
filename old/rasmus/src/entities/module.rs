use crate::{instances::module::ExternalDependency, validation::module::validate};

pub use super::instructions::*;
use super::types::*;

#[derive(Debug, Default, Clone)]
pub struct Module {
    pub types: Vec<FuncType>,
    pub imports: Vec<ImportType>,
    pub funcs: Vec<TypeIdx>,
    pub tables: Vec<TableType>,
    pub mems: Vec<MemType>,
    pub globals: Vec<Global>,
    pub exports: Vec<ExportType>,
    pub start: Option<StartType>,
    pub elems: Vec<ElementSegmentType>,
    pub code: Vec<CodeType>,
    pub datas: Vec<DataType>,
}

impl Module {
    pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
    pub const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

    pub fn is_valid(&self, externals: &Vec<ExternalDependency>) -> bool {
        match validate(&self, externals) {
            Err(validation_error) => {
                println!("Invalid module. Validation error {validation_error:?}");
                false
            }
            Ok(_) => true,
        }
    }

    pub fn get_funcs(&self) -> Option<Vec<Func>> {
        let num = self.funcs.len();
        let mut funcs = Vec::with_capacity(num);
        for i in 0..num {
            let type_idx = self.funcs.get(i)?;
            let code = self.code.get(i)?;
            let locals =
                code.code
                    .locals
                    .clone()
                    .iter()
                    .fold(vec![], |mut locals_acc, current_locals| {
                        locals_acc.append(&mut vec![
                            current_locals.val_type.clone();
                            current_locals.n.0 as usize
                        ]);
                        locals_acc
                    });
            let body = ExpressionType {
                instructions: code.code.expression.instructions.clone(),
            };

            funcs.push(Func {
                func_type: type_idx.clone(),
                locals,
                body,
            });
        }

        Some(funcs)
    }
}

pub type SectionIdValue = Byte;

#[derive(Debug, PartialEq)]
pub enum SectionId {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    DataCount,
}

#[derive(Debug, PartialEq)]
pub struct CustomSection {
    pub name: String,
    pub bytes: Vec<Byte>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportType {
    pub module: NameType,
    pub name: NameType,
    pub desc: ImportDescription,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportDescription {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

impl ImportDescription {}

#[derive(Debug, PartialEq, Clone)]
pub struct ExportType {
    pub name: NameType,
    pub desc: ExportDescription,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportDescription {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StartType {
    pub func: FuncIdx,
}

#[derive(Debug, Clone)]
pub struct Global {
    pub global_type: GlobalType,
    pub init: ExpressionType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElementSegmentType {
    Active0Functions(Active0FunctionsElementSegmentType),
    ElemKindPassiveFunctions(ElemKindPassiveFunctionsElementSegmentType),
    ElemKindActiveFunctions(ElemKindActiveFunctionsElementSegmentType),
    ElemKindDeclarativeFunctions(ElemKindDeclarativeFunctionsElementSegmentType),
    Active0Expr(Active0ExprElementSegmentType),
    PassiveRef(PassiveRefElementSegmentType),
    ActiveRef(ActiveRefElementSegmentType),
    DeclarativeRef(DeclarativeRefElementSegmentType),
}

impl ElementSegmentType {
    pub fn get_type(&self) -> RefType {
        match self {
            Self::Active0Functions(_) => RefType::FuncRef,
            Self::ElemKindPassiveFunctions(t) => (&t.elem_kind).into(),
            Self::ElemKindActiveFunctions(t) => (&t.elem_kind).into(),
            Self::ElemKindDeclarativeFunctions(t) => (&t.elem_kind).into(),
            Self::Active0Expr(_) => RefType::FuncRef,
            Self::PassiveRef(t) => t.ref_type.clone(),
            Self::ActiveRef(t) => t.ref_type.clone(),
            Self::DeclarativeRef(t) => t.ref_type.clone(),
        }
    }

    pub fn get_init(&self) -> &Vec<ExpressionType> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Active0FunctionsElementSegmentType {
    pub mode: ElemModeActive0,
    // RefType::FuncRef only
    pub init: Vec<FuncIdx>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElemKindPassiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModePassive,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElemKindActiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModeActive,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElemKindDeclarativeFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModeDeclarative,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Active0ExprElementSegmentType {
    pub init: Vec<ExpressionType>,
    pub mode: ElemModeActive0,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PassiveRefElementSegmentType {
    pub ref_type: RefType,
    pub init: Vec<ExpressionType>,
    pub mode: ElemModePassive,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ActiveRefElementSegmentType {
    pub ref_type: RefType,
    pub init: Vec<ExpressionType>,
    pub mode: ElemModeActive,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarativeRefElementSegmentType {
    pub ref_type: RefType,
    pub init: Vec<ExpressionType>,
    pub mode: ElemModeDeclarative,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElemModeActive {
    pub table_idx: TableIdx,
    pub offset: ExpressionType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElemModeActive0 {
    pub offset: ExpressionType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElemModePassive;

#[derive(Debug, PartialEq, Clone)]
pub struct ElemModeDeclarative;

#[derive(Debug, PartialEq, Clone)]
pub enum ElemKind {
    FuncRef,
}

impl Into<RefType> for &ElemKind {
    fn into(self) -> RefType {
        RefType::FuncRef
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CodeType {
    pub size: U32Type,
    pub code: FuncCodeType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncCodeType {
    pub locals: Vec<LocalsType>,
    pub expression: ExpressionType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalsType {
    pub n: U32Type,
    pub val_type: ValType,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Active0(Active0DataType),
    Active(ActiveDataType),
    Passive(PassiveDataType),
}

impl DataType {
    pub fn clone_data(&self) -> Vec<Byte> {
        match self {
            Self::Active0(t) => t.init.clone(),
            Self::Active(t) => t.init.clone(),
            Self::Passive(t) => t.init.clone(),
        }
    }
}

pub type Active0DataType = GenericDataType<DataModeActive0>;
pub type ActiveDataType = GenericDataType<DataModeActive>;
pub type PassiveDataType = GenericDataType<DataModePassive>;

#[derive(Debug, Clone)]
pub struct DataModeActive0 {
    pub offset: ExpressionType,
}

#[derive(Debug, Clone)]
pub struct DataModeActive {
    pub memory: MemIdx,
    pub offset: ExpressionType,
}

#[derive(Debug, Clone)]
pub struct DataModePassive;

#[derive(Clone, Debug)]
pub struct GenericDataType<Mode: ::std::fmt::Debug + Clone> {
    pub mode: Mode,
    pub init: Vec<Byte>,
}

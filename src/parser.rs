#[derive(Debug)]
pub struct Identifier(String);

#[derive(Debug)]
pub struct Int(i128);

#[derive(Debug)]
pub struct PyString(String);

#[derive(Debug)]
pub struct Bytes(Vec<u8>);

#[derive(Debug)]
pub struct Object(());

#[derive(Debug)]
pub enum Singleton {
    None,
    True,
    False,
}

#[derive(Debug)]
pub struct Module {
    body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Keyword {
    arg: Option<Identifier>,
    value: Expression,
}

#[derive(Debug)]
pub struct Class {
    name: Identifier,
    keywords: Vec<Keyword>,
    body: Vec<Statement>,
    decorator_list: Vec<Expression>,
}

#[derive(Debug)]
pub struct FunctionArgument {
    arg: Identifier,
    annotation: Option<Expression>,
    default: Option<Expression>,
}

#[derive(Debug)]
pub struct FunctionArguments {
    args: Vec<FunctionArgument>,
    vararg: Option<FunctionArgument>,
    kwonlyargs: Vec<FunctionArgument>,
    kwarg: Option<FunctionArgument>,
}

#[derive(Debug)]
pub struct Function {
    name: Identifier,
    args: FunctionArguments,
    body: Vec<Statement>,
    decorator_list: Vec<Expression>,
    returns: Option<Expression>,
}

#[derive(Debug)]
pub struct Assign {
    targets: Vec<Expression>,
    value: Expression,
}

#[derive(Debug)]
pub struct AugAssign {
    target: Expression,
    op: Operators,
    value: Expression,
}

#[derive(Debug)]
pub struct AnnAssign {
    target: Expression,
    annotation: Expression,
    value: Option<Expression>,
}

#[derive(Debug)]
pub struct For {
    target: Expression,
    iter: Expression,
    body: Vec<Statement>,
    orelse: Vec<Statement>,
}

#[derive(Debug)]
pub struct While {
    test: Expression,
    body: Vec<Statement>,
    orelse: Vec<Statement>,
}

#[derive(Debug)]
pub enum BoolOperators {
    And,
    Or,
}

#[derive(Debug)]
pub enum UnaryOperators {
    Invert,
    Not,
    UAdd,
    USub,
}

#[derive(Debug)]
pub enum Operators {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

#[derive(Debug)]
pub enum CompareOperators {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

#[derive(Debug)]
pub struct If {
    test: Expression,
    body: Vec<Statement>,
    orelse: Vec<Statement>,
}

#[derive(Debug)]
pub struct WithItem {
    context: Expression,
    vars: Option<Expression>,
}

#[derive(Debug)]
pub struct With {
    items: Vec<WithItem>,
    body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Raise {
    exc: Option<Expression>,
    cause: Option<Expression>,
}

#[derive(Debug)]
pub struct ExceptHandler {
    exception: Expression,
    name: Option<Identifier>,
    body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Try {
    body: Vec<Statement>,
    handles: Vec<ExceptHandler>,
    orelse: Vec<Statement>,
    finalbody: Vec<Statement>,
}

#[derive(Debug)]
pub struct Assert {
    test: Expression,
    msg: Option<Expression>,
}

#[derive(Debug)]
pub struct Import {
    name: Identifier,
    asname: Option<Identifier>,
}

#[derive(Debug)]
pub struct ImportFrom {
    module: Option<Identifier>,
    names: Vec<Import>,
    level: Option<u8>,
}

#[derive(Debug)]
pub enum Statement {
    Function(Function),
    AsyncFunction(Function),
    Class(Class),
    Return(Option<Expression>),
    Delete(Vec<Expression>),
    Assign(Assign),
    AugAssign(AugAssign),
    AnnAssign(AnnAssign),
    For(For),
    AsyncFor(For),
    While(While),
    If(If),
    With(With),
    AsyncWith(With),
    Raise(Raise),
    Try(Try),
    Assert(Assert),
    Import(Vec<Import>),
    ImportFrom(ImportFrom),
    Global(Vec<Identifier>),
    Nonlocal(Vec<Identifier>),
    Pass,
    Break,
    Continue,
}

#[derive(Debug)]
pub struct BoolOp {
    op: BoolOperators,
    values: Vec<Expression>,
}

#[derive(Debug)]
pub struct BinOp {
    left: Expression,
    op: Operators,
    right: Expression,
}

#[derive(Debug)]
pub struct UnaryOp {
    op: UnaryOperators,
    operand: Expression,
}

#[derive(Debug)]
pub struct Lambda {
    args: FunctionArguments,
    body: Expression,
}

#[derive(Debug)]
pub struct IfExpr {
    test: Expression,
    body: Expression,
    orelse: Expression,
}

#[derive(Debug)]
pub struct Dict {
    keys: Vec<Expression>,
    values: Vec<Expression>,
}

#[derive(Debug)]
pub struct Set {
    elts: Vec<Expression>,
}

#[derive(Debug)]
pub struct Comprehension {
    target: Expression,
    iter: Expression,
    ifs: Vec<Expression>,
    is_async: bool,
}

#[derive(Debug)]
pub struct ListComp {
    elt: Expression,
    generators: Vec<Comprehension>,
}

#[derive(Debug)]
pub struct SetComp {
    elt: Expression,
    generators: Vec<Comprehension>,
}

#[derive(Debug)]
pub struct DictComp {
    key: Expression,
    value: Expression,
    generators: Vec<Comprehension>,
}

#[derive(Debug)]
pub struct GeneratorExpr {
    elt: Expression,
    generators: Vec<Comprehension>,
}

#[derive(Debug)]
pub struct Await(Expression);

#[derive(Debug)]
pub struct Yield(Option<Expression>);

#[derive(Debug)]
pub struct YieldFrom(Expression);

#[derive(Debug)]
pub struct Compare {
    left: Expression,
    ops: CompareOperators,
    comparators: ExceptHandler,
}

#[derive(Debug)]
pub struct Call {
    func: Expression,
    args: Vec<Expression>,
    keywords: Vec<Keyword>,
}

#[derive(Debug)]
pub struct FormattedString {
    value: Expression,
    conversion: Option<u8>,
    format_spec: Expression,
}

#[derive(Debug)]
pub struct Attribute {
    value: Expression,
    attr: Identifier,
}

#[derive(Debug)]
pub struct BasicSlice {
    lower: Option<Expression>,
    upper: Option<Expression>,
    step: Option<Expression>,
}

#[derive(Debug)]
pub struct ExtSlice {
    dims: Vec<Slice>,
}

#[derive(Debug)]
pub struct Index(Expression);

#[derive(Debug)]
pub enum Slice {
    Slice(BasicSlice),
    ExtSlice(ExtSlice),
    Index(Index),
}

#[derive(Debug)]
pub struct Subscript {
    value: Expression,
    slice: Slice,
}

#[derive(Debug)]
pub enum _Expression {
    BoolOp(BoolOp),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
    Lambda(Lambda),
    IfExpr(IfExpr),
    Dict(Dict),
    Set(Set),
    ListComp(ListComp),
    DictComp(DictComp),
    GeneratorExpr(GeneratorExpr),
    Await(Expression),
    Yield(Option<Expression>),
    YieldFrom(Expression),
    Compare(Compare),
    Call(Call),
    Num(Int),
    Str(PyString),
    FormattedString(FormattedString),
    JoinedStr(Vec<Expression>),
    Bytes(Bytes),
    NameConstrant(Singleton), // ??
    Ellipsis,
    Constant(()), // ??
    Attribute(Attribute),
    Subscript(Subscript),
    Starred(Expression),
    Name(Identifier),
    List(Vec<Expression>),
    Tuple(Vec<Expression>),
}

pub type Expression = Box<_Expression>;

// temp code, will do actual parsing another time

pub fn parse(_: String) -> Result<Module, ()> {
    Ok(Module {
        body: vec![
            Statement::Import(vec![Import {
                name: Identifier(String::from("typing")),
                asname: None,
            }]),
            Statement::Class(Class {
                name: Identifier(String::from("A")),
                keywords: vec![],
                body: vec![Statement::Function(Function {
                    name: Identifier(String::from("__init__")),
                    args: FunctionArguments {
                        args: vec![
                            FunctionArgument {
                                arg: Identifier(String::from("self")),
                                annotation: None,
                                default: None,
                            },
                            FunctionArgument {
                                arg: Identifier(String::from("arg")),
                                annotation: Some(Box::new(_Expression::Name(Identifier(
                                    String::from("int"),
                                )))),
                                default: None,
                            },
                        ],
                        kwarg: None,
                        kwonlyargs: vec![],
                        vararg: None,
                    },
                    body: vec![Statement::Assign(Assign {
                        targets: vec![Box::new(_Expression::Attribute(Attribute {
                            attr: Identifier(String::from("attr")),
                            value: Box::new(_Expression::Name(Identifier(String::from("self")))),
                        }))],
                        value: Box::new(_Expression::Name(Identifier(String::from("arg")))),
                    })],
                    decorator_list: vec![],
                    returns: Some(Box::new(_Expression::NameConstrant(Singleton::None))),
                })],
                decorator_list: vec![],
            }),
            Statement::Function(Function {
                name: Identifier(String::from("make_a")),
                args: FunctionArguments {
                    args: vec![FunctionArgument {
                        arg: Identifier(String::from("self")),
                        annotation: None,
                        default: None,
                    }],
                    kwarg: None,
                    kwonlyargs: vec![],
                    vararg: None,
                },
                body: vec![Statement::Return(Some(Box::new(_Expression::Call(Call {
                    func: Box::new(_Expression::Name(Identifier(String::from("A")))),
                    args: vec![Box::new(_Expression::Name(Identifier(String::from("arg"))))],
                    keywords: vec![],
                }))))],
                decorator_list: vec![],
                returns: Some(Box::new(_Expression::Name(Identifier(String::from("A"))))),
            }),
        ],
    })
}

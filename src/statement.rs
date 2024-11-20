use crate::pretty::impl_display_via_pretty;
use crate::{Block, Expression, Type, Value};
use pretty::Pretty;

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Statements
#[derive(Clone)]
pub enum CStatement {
    Label {
        identifier: String,
        statement: Box<CStatement>,
    },
    Expression(Expression),
    IfStatement {
        condition: Value,
        then_block: Vec<CStatement>,
        else_block: Option<Vec<CStatement>>,
    },
    SwitchStatement {
        expression: Value,
        cases: Vec<(Value, Vec<CStatement>)>,
        default: Option<Vec<CStatement>>,
    },
    WhileStatement {
        condition: Value,
        body: Vec<CStatement>,
    },
    DoStatement {
        body: Vec<CStatement>,
        condition: Value,
    },
    ForStatement {
        init: Option<Box<CStatement>>,
        condition: Value,
        step: Option<Box<CStatement>>,
        body: Vec<CStatement>,
    },
    Block(Block),
    NullStatement,
    GotoStatement(String),
    BreakStatement,
    ContinueStatement,
    ReturnStatement(Option<Expression>),
    TypedefStatement(String, Type),
    FunctionDeclaration {
        return_type: Type,
        name: String,
        parameters: Vec<(Type, Option<String>)>,
    },
    FunctionDefinition {
        return_type: Type,
        name: String,
        parameters: Vec<(Type, Option<String>)>,
        body: Vec<CStatement>,
    },
    StructDeclaration {
        name: String,
        fields: Vec<(Type, String)>,
    },
    EnumDeclaration {
        name: String,
        variants: Vec<(String, Option<Value>)>,
    },
    IncludeStatement(String),
    MacroDefinition {
        name: String,
        body: String,
    },
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for CStatement
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            CStatement::Label {
                identifier,
                statement,
            } => allocator
                .text(identifier)
                .append(allocator.text(":"))
                .append(allocator.hardline())
                .append(statement.pretty(allocator)),
            CStatement::Expression(expression) => {
                expression.pretty(allocator).append(allocator.text(";"))
            }
            CStatement::IfStatement {
                condition,
                then_block,
                else_block,
            } => {
                let mut doc = allocator
                    .text("if (")
                    .append(condition.to_string())
                    .append(allocator.text(") {"))
                    .append(allocator.hardline());

                for stmt in then_block {
                    doc = doc
                        .append(stmt.pretty(allocator))
                        .append(allocator.hardline());
                }

                doc = doc.append(allocator.text("}"));

                if let Some(else_block) = else_block {
                    doc = doc
                        .append(allocator.text(" else {"))
                        .append(allocator.hardline());

                    for stmt in else_block {
                        doc = doc
                            .append(stmt.pretty(allocator))
                            .append(allocator.hardline());
                    }

                    doc = doc.append(allocator.text("}"));
                }

                doc.nest(2)
            }
            CStatement::SwitchStatement {
                expression,
                cases,
                default,
            } => {
                let mut doc = allocator
                    .text("switch (")
                    .append(expression.to_string())
                    .append(allocator.text(") {"))
                    .append(allocator.hardline());

                for (case, block) in cases {
                    doc = doc
                        .append(allocator.text("case "))
                        .append(case.to_string())
                        .append(allocator.text(":"))
                        .append(allocator.hardline());

                    for stmt in block {
                        doc = doc
                            .append(stmt.pretty(allocator))
                            .append(allocator.hardline());
                    }
                    doc = doc
                        .append(allocator.text("break;"))
                        .append(allocator.hardline());
                }

                if let Some(block) = default {
                    doc = doc
                        .append(allocator.text("default:"))
                        .append(allocator.hardline());

                    for stmt in block {
                        doc = doc
                            .append(stmt.pretty(allocator))
                            .append(allocator.hardline());
                    }
                }

                doc.append(allocator.text("}")).nest(2)
            }
            CStatement::WhileStatement { condition, body } => {
                let mut doc = allocator
                    .text("while (")
                    .append(condition.to_string())
                    .append(allocator.text(") {"))
                    .append(allocator.hardline());

                for stmt in body {
                    doc = doc
                        .append(stmt.pretty(allocator))
                        .append(allocator.hardline());
                }

                doc.append(allocator.text("}")).nest(2)
            }
            CStatement::DoStatement { body, condition } => {
                let mut doc = allocator.text("do {").append(allocator.hardline());

                for stmt in body {
                    doc = doc
                        .append(stmt.pretty(allocator))
                        .append(allocator.hardline());
                }

                doc.append(allocator.text("} while ("))
                    .append(condition.to_string())
                    .append(allocator.text(");"))
                    .nest(2)
            }
            CStatement::ForStatement {
                init,
                condition,
                step,
                body,
            } => {
                let mut doc = allocator.text("for (");

                if let Some(init) = init {
                    doc = doc.append(init.pretty(allocator));
                } else {
                    doc = doc.append(allocator.text(";"));
                }

                doc = doc
                    .append(allocator.text(" "))
                    .append(condition.to_string())
                    .append(allocator.text("; "));

                if let Some(step) = step {
                    doc = doc.append(step.pretty(allocator));
                }

                doc = doc
                    .append(allocator.text(") {"))
                    .append(allocator.hardline());

                for stmt in body {
                    doc = doc
                        .append(stmt.pretty(allocator))
                        .append(allocator.hardline());
                }

                doc.append(allocator.text("}")).nest(2)
            }
            CStatement::Block(block) => block.pretty(allocator),
            CStatement::NullStatement => allocator.text(";"),
            CStatement::GotoStatement(label) => allocator
                .text("goto ")
                .append(allocator.text(label))
                .append(allocator.text(";")),
            CStatement::BreakStatement => allocator.text("break;"),
            CStatement::ContinueStatement => allocator.text("continue;"),
            CStatement::ReturnStatement(expression) => {
                let mut doc = allocator.text("return");
                if let Some(expression) = expression {
                    doc = doc
                        .append(allocator.text(" "))
                        .append(expression.pretty(allocator));
                }
                doc.append(allocator.text(";"))
            }
            CStatement::TypedefStatement(name, ty) => allocator
                .text("typedef ")
                .append(allocator.text(format!("{}", ty)))
                .append(allocator.text(" "))
                .append(allocator.text(name))
                .append(allocator.text(";")),
            CStatement::FunctionDeclaration {
                return_type,
                name,
                parameters,
            } => {
                let param_str = parameters
                    .iter()
                    .map(|(param_type, param_name)| match param_name {
                        Some(name) => format!("{} {}", param_type, name),
                        None => format!("{}", param_type),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                allocator.text(format!("{} {}({});", return_type, name, param_str))
            }
            CStatement::FunctionDefinition {
                return_type,
                name,
                parameters,
                body,
            } => {
                let param_str = parameters
                    .iter()
                    .map(|(param_type, param_name)| match param_name {
                        Some(name) => format!("{} {}", param_type, name),
                        None => format!("{}", param_type),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let mut doc = allocator
                    .text(format!("{} {}({}) {{", return_type, name, param_str))
                    .append(allocator.hardline());

                for stmt in body {
                    doc = doc
                        .append(stmt.pretty(allocator))
                        .append(allocator.hardline());
                }

                doc.append(allocator.text("}")).nest(2)
            }
            CStatement::StructDeclaration { name, fields } => {
                let mut doc = allocator
                    .text(format!("struct {} {{", name))
                    .append(allocator.hardline());

                for (field_type, field_name) in fields {
                    doc = doc
                        .append(allocator.text(format!("{} {};", field_type, field_name)))
                        .append(allocator.hardline());
                }

                doc.append(allocator.text("};")).nest(2)
            }
            CStatement::EnumDeclaration { name, variants } => {
                let mut doc = allocator
                    .text(format!("enum {} {{", name))
                    .append(allocator.hardline());

                for (variant, value) in variants {
                    let variant_str = if let Some(value) = value {
                        format!("{}__{} = {},", name, variant, value)
                    } else {
                        format!("{}__{},", name, variant)
                    };
                    doc = doc
                        .append(allocator.text(variant_str))
                        .append(allocator.hardline());
                }

                doc.append(allocator.text("};")).nest(2)
            }
            CStatement::IncludeStatement(header_file) => {
                allocator.text(format!("#include <{}>", header_file))
            }
            CStatement::MacroDefinition { name, body } => {
                allocator.text(format!("#define {} {}", name, body))
            }
        }
    }
}

impl_display_via_pretty!(CStatement, 80);

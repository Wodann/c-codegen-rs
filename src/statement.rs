mod control_flow;
mod label;

use pretty::Pretty;

pub use self::control_flow::{Do, For, If, Switch, While};
pub use self::label::Label;
use crate::{
    macros::impl_froms, pretty::impl_display_via_pretty, Block, Expression, Identifier, Type, Value,
};

/// # Source
///
/// https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Statements
#[derive(Clone)]
pub enum Statement {
    Expression(Expression),
    Label(Box<Label>),
    If(Box<If>),
    Switch(Box<Switch>),
    While(Box<While>),
    Do(Box<Do>),
    For(Box<For>),
    Block(Block),
    Null,
    Goto(Identifier),
    Break,
    Continue,
    Return(Option<Expression>),
    Typedef(Identifier, Type),
    FunctionDeclaration {
        return_type: Type,
        name: Identifier,
        parameters: Vec<(Type, Option<Identifier>)>,
    },
    FunctionDefinition {
        return_type: Type,
        name: Identifier,
        parameters: Vec<(Type, Option<Identifier>)>,
        body: Vec<Statement>,
    },
    StructDeclaration {
        name: Identifier,
        fields: Vec<(Type, Identifier)>,
    },
    EnumDeclaration {
        name: Identifier,
        variants: Vec<(Identifier, Option<Value>)>,
    },
    Include(String),
    MacroDefinition {
        name: Identifier,
        body: String,
    },
}

impl Statement {
    pub fn has_custom_indentation(&self) -> bool {
        matches!(self, Statement::Label(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Statement::Block(_))
    }
}

impl_froms!(Statement: Block, Expression, box Label);

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Statement
where
    AnnotationT: Clone + 'a,
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        match self {
            Statement::Label(label) => label.pretty(allocator),
            Statement::Expression(expression) => {
                expression.pretty(allocator).append(allocator.text(";"))
            }
            Statement::If(if_stmt) => if_stmt.pretty(allocator),
            Statement::Switch(switch_stmt) => switch_stmt.pretty(allocator),
            Statement::While(while_stmt) => while_stmt.pretty(allocator),
            Statement::Do(do_stmt) => do_stmt.pretty(allocator),
            Statement::For(for_stmt) => for_stmt.pretty(allocator),
            Statement::Block(block) => block.pretty(allocator),
            Statement::Null => allocator.text(";"),
            Statement::Goto(label) => allocator
                .text("goto ")
                .append(allocator.text(label))
                .append(allocator.text(";")),
            Statement::Break => allocator.text("break;"),
            Statement::Continue => allocator.text("continue;"),
            Statement::Return(expression) => {
                let mut doc = allocator.text("return");
                if let Some(expression) = expression {
                    doc = doc
                        .append(allocator.text(" "))
                        .append(expression.pretty(allocator));
                }
                doc.append(allocator.text(";"))
            }
            Statement::Typedef(name, ty) => allocator
                .text("typedef ")
                .append(allocator.text(format!("{}", ty)))
                .append(allocator.text(" "))
                .append(allocator.text(name))
                .append(allocator.text(";")),
            Statement::FunctionDeclaration {
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
            Statement::FunctionDefinition {
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
            Statement::StructDeclaration { name, fields } => {
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
            Statement::EnumDeclaration { name, variants } => {
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
            Statement::Include(header_file) => {
                allocator.text(format!("#include <{}>", header_file))
            }
            Statement::MacroDefinition { name, body } => {
                allocator.text(format!("#define {} {}", name, body))
            }
        }
    }
}

impl_display_via_pretty!(Statement, 80);

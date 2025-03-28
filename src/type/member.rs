use pretty::Pretty;

use crate::{non_empty_vec::NonEmptyVec, Expression, Identifier};

use super::ConcreteType;

#[derive(Clone)]
pub struct IndirectMemberAccess {
    pub left: Expression,
    pub member: Identifier,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for IndirectMemberAccess
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        self.left
            .pretty(allocator)
            .append(allocator.text("->"))
            .append(allocator.text(self.member.to_string()))
    }
}

#[derive(Clone)]
pub struct MemberAccess {
    pub left: Expression,
    pub member: Identifier,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for MemberAccess
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text(self.left.to_string())
            .append(allocator.text("."))
            .append(allocator.text(self.member.to_string()))
    }
}

#[derive(Clone)]
pub struct Group {
    pub ty: ConcreteType,
    pub members: NonEmptyVec<Member>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Group
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        allocator
            .text(self.ty.to_string())
            .append(allocator.space())
            .append(
                allocator.intersperse(
                    self.members
                        .into_iter()
                        .map(|member| member.pretty(allocator)),
                    allocator.text(",").append(allocator.space()),
                ),
            )
            .append(allocator.text(";"))
    }
}

#[derive(Clone)]
pub struct Member {
    pub name: Identifier,
    pub bit_field_size: Option<usize>,
}

impl<'a, AllocatorT, AnnotationT> Pretty<'a, AllocatorT, AnnotationT> for Member
where
    AllocatorT: pretty::DocAllocator<'a, AnnotationT>,
    AllocatorT::Doc: Clone,
    AnnotationT: Clone + 'a,
{
    fn pretty(self, allocator: &'a AllocatorT) -> pretty::DocBuilder<'a, AllocatorT, AnnotationT> {
        let mut builder = allocator.text(self.name.to_string());

        if let Some(size) = self.bit_field_size {
            builder = builder
                .append(allocator.space())
                .append(allocator.text(":"))
                .append(allocator.space())
                .append(allocator.text(size.to_string()));
        }

        builder
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use crate::{function::FunctionCall, operator, Statement, Value, Variable};

    use super::*;

    #[test]
    fn access() -> anyhow::Result<()> {
        let variable = Statement::Expression(
            operator::Assignment {
                left: MemberAccess {
                    left: Variable::new("first_number")?.into(),
                    member: Identifier::new("i")?,
                }
                .into(),
                right: Value::int(5).into(),
            }
            .into(),
        )
        .to_string();
        assert_eq!(variable, "first_number.i = 5;");

        Ok(())
    }

    #[test]
    fn indirect_access() -> anyhow::Result<()> {
        let variable = Statement::Expression(
            operator::Assignment {
                left: IndirectMemberAccess {
                    left: Variable::new("first_number")?.into(),
                    member: Identifier::new("i")?,
                }
                .into(),
                right: Value::int(5).into(),
            }
            .into(),
        )
        .to_string();
        assert_eq!(variable, "first_number->i = 5;");
        let function = Statement::Expression(
            operator::Assignment {
                left: IndirectMemberAccess {
                    left: FunctionCall {
                        name: Identifier::new("get_struct")?,
                        arguments: Vec::new(),
                    }
                    .into(),
                    member: Identifier::new("i")?,
                }
                .into(),
                right: Value::int(5).into(),
            }
            .into(),
        )
        .to_string();
        assert_eq!(function, "get_struct()->i = 5;");

        Ok(())
    }
}

use crate::{ast::FunDeclStatement, tree_walker::{Environment, TreeWalker}};


trait Callable {
    fn call(interpreter: TreeWalker, environment: Environment);
}

struct Function<'a> {
    declaration: FunDeclStatement<'a>
}
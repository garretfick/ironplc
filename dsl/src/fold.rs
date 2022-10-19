use crate::ast::*;
use crate::dsl::*;

// Defines an object as being able to be folded. That is, return a new
// folded version of itself.
pub(crate) trait Foldable {
    type Mapped;
    fn fold<F: Fold + ?Sized>(self, folder: &mut F) -> Self::Mapped;
}

impl<X> Foldable for Vec<X>
where
    X: Foldable,
{
    type Mapped = Vec<X::Mapped>;
    fn fold<F: Fold + ?Sized>(self, folder: &mut F) -> Self::Mapped {
        self.into_iter().map(|x| x.fold(folder)).collect()
    }
}

impl<X> Foldable for Option<X>
where
    X: Foldable,
{
    type Mapped = Option<X::Mapped>;
    fn fold<F: Fold + ?Sized>(self, folder: &mut F) -> Self::Mapped {
        self.map(|x| x.fold(folder))
    }
}

pub trait Fold {
    fn fold(&mut self, node: Library) -> Library {
        Library {
            elems: Foldable::fold(node.elems, self),
        }
    }
    fn fold_library_element_declaration(&mut self, node: LibraryElement) -> LibraryElement {
        match node {
            LibraryElement::FunctionBlockDeclaration(function_block_decl) => {
                LibraryElement::FunctionBlockDeclaration(
                    self.fold_function_block_declaration(function_block_decl),
                )
            }
            LibraryElement::FunctionDeclaration(function_decl) => {
                LibraryElement::FunctionDeclaration(self.fold_function_declaration(function_decl))
            }
            LibraryElement::ProgramDeclaration(program_decl) => {
                LibraryElement::ProgramDeclaration(self.fold_program_declaration(program_decl))
            }
            _ => node,
        }
    }

    fn fold_function_block_declaration(
        &mut self,
        node: FunctionBlockDeclaration,
    ) -> FunctionBlockDeclaration {
        FunctionBlockDeclaration {
            name: node.name.clone(),
            var_decls: Foldable::fold(node.var_decls, self),
            body: node.body.clone(),
        }
    }

    fn fold_function_declaration(&mut self, node: FunctionDeclaration) -> FunctionDeclaration {
        FunctionDeclaration {
            name: node.name.clone(),
            return_type: node.return_type.clone(),
            var_decls: Foldable::fold(node.var_decls, self),
            body: node.body.clone(),
        }
    }

    fn fold_program_declaration(&mut self, node: ProgramDeclaration) -> ProgramDeclaration {
        ProgramDeclaration {
            type_name: node.type_name.clone(),
            var_declarations: Foldable::fold(node.var_declarations, self),
            body: node.body.clone(),
        }
    }

    fn fold_var_init_kind(&mut self, node: VarInitKind) -> VarInitKind {
        match node {
            VarInitKind::VarInit(var_init) => {
                VarInitKind::VarInit(self.fold_var_init_decl(var_init))
            }
            _ => node,
        }
    }

    fn fold_var_init_decl(&mut self, node: VarInitDecl) -> VarInitDecl {
        VarInitDecl {
            name: node.name.clone(),
            storage_class: node.storage_class.clone(),
            initializer: Foldable::fold(node.initializer, self),
        }
    }

    fn fold_type_initializer(&mut self, node: TypeInitializer) -> TypeInitializer {
        node
    }
}

impl Foldable for LibraryElement {
    type Mapped = LibraryElement;
    fn fold<F: Fold + ?Sized>(self, folder: &mut F) -> Self::Mapped {
        folder.fold_library_element_declaration(self)
    }
}

impl Foldable for VarInitKind {
    type Mapped = VarInitKind;
    fn fold<F: Fold + ?Sized>(self, folder: &mut F) -> Self::Mapped {
        folder.fold_var_init_kind(self)
    }
}

impl Foldable for VarInitDecl {
    type Mapped = VarInitDecl;
    fn fold<F: Fold + ?Sized>(self, folder: &mut F) -> Self::Mapped {
        folder.fold_var_init_decl(self)
    }
}

impl Foldable for TypeInitializer {
    type Mapped = TypeInitializer;
    fn fold<F: Fold + ?Sized>(self, folder: &mut F) -> Self::Mapped {
        folder.fold_type_initializer(self)
    }
}
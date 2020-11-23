use crate::neat::Namespace;
use cjc_hir::{Parameter, Function};

pub fn struct_function(struct_func_def: &cjc_parser::StructFuncDef, namespace: &mut Namespace) -> bool {
    let mut success = true;

    let params = resolve_params(&struct_func_def.params, namespace);
    // println!("{:?}", params);

    let (returns, return_success) = resolve_returns(&struct_func_def.returns, namespace);

    let name = struct_func_def.name.name.to_owned();

    let function = Function::new(name, params, returns);
    println!("{:?}", function);

    success
}

pub fn resolve_returns(_returns: &Option<cjc_parser::Expression>, namespace: &mut Namespace) -> (Vec<Parameter>, bool) {
    let mut resolved_returns = Vec::new();
    let mut success = true;

    (resolved_returns, success)
}

pub fn resolve_params(parameters: &Vec<(cjc_lexer::Loc, Option<cjc_parser::Parameter>)>, namespace: &mut Namespace) -> Vec<Parameter> {
    let mut params = Vec::new();
    for (loc, p) in parameters {
        let p = match p {
            Some(p) => p,
            None => {
                continue;
            }
        };

        namespace.resolve_type(&p.ty);

        params.push(Parameter {
            location: *loc,
            name: p.get_name()
        })
    }
    return params;
}
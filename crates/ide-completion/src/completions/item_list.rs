//! Completion of paths and keywords at item list position.

use crate::{
    context::{ItemListKind, PathCompletionCtx, PathExprCtx, Qualified},
    CompletionContext, Completions,
};

pub(crate) mod trait_impl;

pub(crate) fn complete_item_list_in_expr(
    acc: &mut Completions,
    ctx: &CompletionContext<'_>,
    path_ctx: &PathCompletionCtx,
    expr_ctx: &PathExprCtx,
) {
    if !expr_ctx.in_block_expr {
        return;
    }
    if !path_ctx.is_trivial_path() {
        return;
    }
    add_keywords(acc, ctx, None);
}

pub(crate) fn complete_item_list(
    acc: &mut Completions,
    ctx: &CompletionContext<'_>,
    path_ctx @ PathCompletionCtx { qualified, .. }: &PathCompletionCtx,
    kind: &ItemListKind,
) {
    let _p = tracing::info_span!("complete_item_list").entered();
    if path_ctx.is_trivial_path() {
        add_keywords(acc, ctx, Some(kind));
    }

    match qualified {
        Qualified::With {
            resolution: Some(hir::PathResolution::Def(hir::ModuleDef::Module(module))),
            super_chain_len,
            ..
        } => {
            for (name, def) in module.scope(ctx.db, Some(ctx.module)) {
                match def {
                    hir::ScopeDef::ModuleDef(hir::ModuleDef::Macro(m)) if m.is_fn_like(ctx.db) => {
                        acc.add_macro(ctx, path_ctx, m, name)
                    }
                    hir::ScopeDef::ModuleDef(hir::ModuleDef::Module(m)) => {
                        acc.add_module(ctx, path_ctx, m, name, vec![])
                    }
                    _ => (),
                }
            }

            acc.add_super_keyword(ctx, *super_chain_len);
        }
        Qualified::Absolute => acc.add_crate_roots(ctx, path_ctx),
        Qualified::No if ctx.qualifier_ctx.none() => {
            ctx.process_all_names(&mut |name, def, doc_aliases| match def {
                hir::ScopeDef::ModuleDef(hir::ModuleDef::Macro(m)) if m.is_fn_like(ctx.db) => {
                    acc.add_macro(ctx, path_ctx, m, name)
                }
                hir::ScopeDef::ModuleDef(hir::ModuleDef::Module(m)) => {
                    acc.add_module(ctx, path_ctx, m, name, doc_aliases)
                }
                _ => (),
            });
            acc.add_nameref_keywords_with_colon(ctx);
        }
        Qualified::TypeAnchor { .. } | Qualified::No | Qualified::With { .. } => {}
    }
}

fn add_keywords(acc: &mut Completions, ctx: &CompletionContext<'_>, kind: Option<&ItemListKind>) {
    let mut add_keyword = |kw, snippet| acc.add_keyword_snippet(ctx, kw, snippet);

    dbg!(&kind);
    let in_item_list = matches!(kind, Some(ItemListKind::SourceFile | ItemListKind::Module) | None);
    let in_assoc_non_trait_impl = matches!(kind, Some(ItemListKind::Impl | ItemListKind::Trait));
    let in_extern_block = matches!(kind, Some(ItemListKind::ExternBlock));
    let in_trait = matches!(kind, Some(ItemListKind::Trait));
    let in_trait_impl = matches!(kind, Some(ItemListKind::TraitImpl(_)));
    let in_inherent_impl = matches!(kind, Some(ItemListKind::Impl));
    let no_vis_qualifiers = ctx.qualifier_ctx.vis_node.is_none();
    let in_block = kind.is_none();

    let missing_qualifiers = [
        ctx.qualifier_ctx.unsafe_tok.is_none().then_some(("unsafe", "unsafe $0")),
        ctx.qualifier_ctx.async_tok.is_none().then_some(("async", "async $0")),
    ];

    if true {
        // handle qualifier tokens
        if missing_qualifiers.iter().any(Option::is_none) {
            // only complete missing qualifiers
            missing_qualifiers.iter().filter_map(|x| *x).for_each(|(kw, snippet)| {
                add_keyword(kw, snippet);
            });

            if in_item_list || in_assoc_non_trait_impl {
                add_keyword("fn", "fn $1($2) {\n    $0\n}");
            }

            if ctx.qualifier_ctx.unsafe_tok.is_some() && in_item_list {
                add_keyword("trait", "trait $1 {\n    $0\n}");
                if no_vis_qualifiers {
                    add_keyword("impl", "impl $1 {\n    $0\n}");
                }
            }

            return;
        }

        if in_trait {
            add_keyword("method", "method $1($2);$0");
        }

        add_keyword("(*", "(* $0 *)");
        if in_item_list {
            add_keyword("begin", "begin\n    $0\nend");
            add_keyword("action", "action\n    $0\nendaction");
            add_keyword("actionvalue", "actionvalue\n    $0\nendactionvalue");

            add_keyword("interface", "interface $1;\n    $0\nendinterface");
            add_keyword("module", "module $1($2);\n    $0\nendmodule");
            add_keyword("function", "function $1($3);\n    $0\nendfunction");

            if !in_trait {
                add_keyword("method", "method $1($2);\n    $0\nendmethod");
            }
            add_keyword("import", "import $0::*;");
            add_keyword("typedef enum", "typedef enum {\n    \n} $0;");
            add_keyword("typedef struct", "typedef struct {\n    \n} $0;");
        }

        // if in_extern_block {
        //     add_keyword("fn", "fn $1($2);");
        // } else {
        //     if !in_inherent_impl {
        //         if !in_trait {
        //             add_keyword("extern", "extern $0");
        //         }
        //         add_keyword("type", "type $0");
        //     }

        //     add_keyword("fn", "fn $1($2) {\n    $0\n}");
        //     add_keyword("unsafe", "unsafe $0");
        //     add_keyword("const", "const $0");
        //     add_keyword("async", "async $0");
        // }
    }
}

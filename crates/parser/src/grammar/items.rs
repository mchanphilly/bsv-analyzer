mod adt;
mod consts;
pub(crate) mod traits;
mod use_item;

use entry::prefix::{expr, stmt};
use traits::provisos;

pub(crate) use self::{
    adt::{record_field_list, variant_list},
    expressions::{match_arm_list, record_expr_field_list},
    traits::assoc_item_list,
    use_item::use_tree_list,
};
use super::*;

// test package_contents
// package Top;
// import FIFOF::*;
// import BRAM   ::   *;
pub(super) fn package_contents_bsv(p: &mut Parser<'_>) {
    // let m = p.start();

    let expect_end = p.eat(T![package]);
    if expect_end {
        name(p);
        p.expect(T![;]);
    }

    while !(p.at(EOF) || (expect_end && p.eat(T![endpackage]))) {
        item_or_stmt_bsv(p, None, false);
    }

    if !p.at(EOF) && !p.at(T![package]) {
        p.error("Package ended but is followed by a non-package statement.");
    }

    // m.complete(p, PACKAGE);
}

// test mod_contents
// fn foo() {}
// macro_rules! foo {}
// foo::bar!();
// super::baz! {}
// struct S;
pub(super) fn mod_contents(p: &mut Parser<'_>, stop_on_r_curly: bool) {
    attributes::inner_attrs(p);
    while !(p.at(EOF) || (p.at(T!['}']) && stop_on_r_curly)) {
        item_or_macro(p, stop_on_r_curly);
    }
}

pub(super) const ITEM_RECOVERY_SET: TokenSet = TokenSet::new(&[
    T![fn],
    T![struct],
    T![enum],
    T![impl],
    T![trait],
    T![const],
    T![static],
    T![let],
    T![package],
    T![pub],
    T![crate],
    T![import],
    T![interface],
    T![endinterface],
    T![module],
    T![endmodule],
    T![method],
    T![endmethod],
    T![rule],
    T![endrule],
    T![function],
    T![endfunction],
    T![typedef],
    T![macro],
    T![;],
    T![deriving],
]);

// May also accept a statement.
pub(super) fn item_or_stmt_bsv(p: &mut Parser<'_>, end: Option<SyntaxKind>, sig_only: bool) {
    let m = p.start();
    attributes::outer_attrs_bsv(p);  // TODO BSV truly add support for outer_attrs

    let m = match opt_item(p, m, sig_only) {
        Ok(()) => {
            if p.at(T![;]) {
                p.err_and_bump(
                    "expected item, found `;`\n\
                     consider removing this semicolon",
                );
            }
            return;
        }
        Err(m) => m,
    };
    m.abandon(p);
    stmt(p);
    return;
}

pub(super) fn item_or_macro(p: &mut Parser<'_>, stop_on_r_curly: bool) {
    dbg!("Deprecated item_or_macro");
    return;
    let m = p.start();
    attributes::outer_attrs(p);

    // let m = match opt_item(p, m) {
    //     Ok(()) => {
    //         if p.at(T![;]) {
    //             p.err_and_bump(
    //                 "expected item, found `;`\n\
    //                  consider removing this semicolon",
    //             );
    //         }
    //         return;
    //     }
    //     Err(m) => m,
    // };

    // test macro_rules_as_macro_name
    // macro_rules! {}
    // macro_rules! ();
    // macro_rules! [];
    // fn main() {
    //     let foo = macro_rules!();
    // }

    // test_err macro_rules_as_macro_name
    // macro_rules! {};
    // macro_rules! ()
    // macro_rules! []
    if paths::is_use_path_start(p) {
        macro_call(p, m);
        return;
    }

    m.abandon(p);
    match p.current() {
        T!['{'] => error_block(p, "expected an item"),
        T!['}'] if !stop_on_r_curly => {
            let e = p.start();
            p.error("unmatched `}`");
            p.bump(T!['}']);
            e.complete(p, ERROR);
        }
        EOF | T!['}'] => p.error("expected an item"),
        T![let] => error_let_stmt(p, "expected an item"),
        _ => p.err_and_bump("expected an item"),
    }
}

/// Try to parse an item, completing `m` in case of success.
pub(super) fn opt_item(p: &mut Parser<'_>, m: Marker, sig_only: bool) -> Result<(), Marker> {
    // test_err pub_expr
    // fn foo() { pub 92; }
    let has_visibility = opt_visibility(p, false);

    let m = match opt_item_without_modifiers(p, m) {
        Ok(()) => return Ok(()),
        Err(m) => m,
    };

    let mut has_mods = false;
    let mut has_extern = false;

    // modifiers
    if p.at(T![const]) && p.nth(1) != T!['{'] {
        p.eat(T![const]);
        has_mods = true;
    }

    // test_err async_without_semicolon
    // fn foo() { let _ = async {} }
    if p.at(T![async])
        && (!matches!(p.nth(1), T!['{'] | T![|])
            || matches!((p.nth(1), p.nth(2)), (T![gen], T![fn])))
    {
        p.eat(T![async]);
        has_mods = true;
    }

    // test_err gen_fn
    // gen fn gen_fn() {}
    // async gen fn async_gen_fn() {}
    if p.at(T![gen]) && p.nth(1) == T![fn] {
        p.eat(T![gen]);
        has_mods = true;
    }

    // test_err unsafe_block_in_mod
    // fn foo(){} unsafe { } fn bar(){}
    if p.at(T![unsafe]) && p.nth(1) != T!['{'] {
        p.eat(T![unsafe]);
        has_mods = true;
    }

    if p.at(T![extern]) {
        has_extern = true;
        has_mods = true;
        abi(p);
    }
    if p.at_contextual_kw(T![auto]) && p.nth(1) == T![trait] {
        p.bump_remap(T![auto]);
        has_mods = true;
    }

    // test default_item
    // default impl T for Foo {}
    if p.at_contextual_kw(T![default]) {
        match p.nth(1) {
            T![fn] | T![type] | T![const] | T![impl] => {
                p.bump_remap(T![default]);
                has_mods = true;
            }
            // test default_unsafe_item
            // default unsafe impl T for Foo {
            //     default unsafe fn foo() {}
            // }
            T![unsafe] if matches!(p.nth(2), T![impl] | T![fn]) => {
                p.bump_remap(T![default]);
                p.bump(T![unsafe]);
                has_mods = true;
            }
            // test default_async_fn
            // impl T for Foo {
            //     default async fn foo() {}
            // }
            T![async]
                if p.nth_at(2, T![fn]) || (p.nth_at(2, T![unsafe]) && p.nth_at(3, T![fn])) =>
            {
                p.bump_remap(T![default]);
                p.bump(T![async]);

                // test default_async_unsafe_fn
                // impl T for Foo {
                //     default async unsafe fn foo() {}
                // }
                p.eat(T![unsafe]);

                has_mods = true;
            }
            _ => (),
        }
    }

    // items
    match p.current() {
        T![fn] => fn_(p, m),

        T![const] if p.nth(1) != T!['{'] => consts::konst(p, m),

        T![trait] => traits::trait_(p, m),
        T![impl] => traits::impl_(p, m),

        T![type] => type_alias(p, m),
        T![typedef] => typedef_(p, m),  // handles synonyms, structs, enums

        T![function] | T![method] | T![rule] => bsv_assoc(p, m, sig_only),
        T![interface] => {traits::interface_(p, m, true);},
        T![module] => traits::module_(p, m),
        // T![module] => traits::module_(p, m),  // TODO_BSV
        // test extern_block
        // unsafe extern "C" {}
        // extern {}
        T!['{'] if has_extern => {
            extern_item_list(p);
            m.complete(p, EXTERN_BLOCK);
        }

        _ if has_visibility || has_mods => {
            if has_mods {
                p.error("expected fn, trait or impl");
            } else {
                p.error("expected an item");
            }
            m.complete(p, ERROR);
        }

        _ => return Err(m),
    }
    Ok(())
}

// pub(super) fn module_stmt(p: &mut Parser<'_>) {
//     let m = p.start();
//     attributes::outer_attrs_bsv(p);

//     // TODO BSV: Consider folding in the other cases as expressions, or eliminating this altogether.
//     match p.current() {
//         T![rule] => rule(p),
//         T![method] => method_impl(p),
//         T![function] => function(p),  // TODO BSV add to outside of module
//         T![let] => expressions::let_stmt(p, expressions::Semicolon::Required),
//         _ => expressions::instantiation(p),  // i.e., assignment stuff
//         // everything else, e.g., for loops TODO BSV
//     }
//     m.complete(p, MODULE_STMT);
// }

// pub(super) fn interface_stmt(p: &mut Parser<'_>) {
//     let m = p.start();
//     match p.current() {
//         T![method] => expressions::method_decl(p),
//         _ => p.bump_any(),  // BSV TODO remove
//     }
//     m.complete(p, INTERFACE_STMT);
// }

// pub(super) fn method_impl(p: &mut Parser<'_>) {
//     let m = p.start();
//     expressions::method_signature(p);
//     if p.at(T![if]) {
//         assert!(opt_guard(p));  // not actually optional
//     }
//     if p.eat(T![;]) {  // TODO BSV: Support shorthand method impls e.g., method Bit#(3) M = 5;
//         expressions::stmt_list_bsv(p, T![endmethod]);
//         p.expect(T![endmethod]);
//     }
//     m.complete(p, METHOD_IMPL);
// }

// pub(super) fn function(p: &mut Parser<'_>) {  // TODO BSV add support for non-module-associated functions
//     let m = p.start();
//     expressions::function_signature(p);
//     if p.at(T![if]) {
//         assert!(opt_guard(p));  // not actually optional
//     }
//     if p.eat(T![;]) {  // TODO BSV: Support shorthand function impls e.g., method Bit#(3) M = 5;
//         expressions::stmt_list_bsv(p, T![endfunction]);
//         p.expect(T![endfunction]);
//     }
//     m.complete(p, FUNCTION);
// }

fn opt_item_without_modifiers(p: &mut Parser<'_>, m: Marker) -> Result<(), Marker> {
    let la = p.nth(1);
    match p.current() {
        T![extern] if la == T![crate] => extern_crate(p, m),
        T![import] => use_item::import_(p, m),
        // T![package] => package_item(p, m),

        T![typedef] => typedef_(p, m),
        T![type] => type_alias(p, m),
        T![struct] => adt::strukt(p, m),
        T![enum] => adt::enum_(p, m),
        IDENT if p.at_contextual_kw(T![union]) && p.nth(1) == IDENT => adt::union(p, m),

        T![macro] => macro_def(p, m),
        // check if current token is "macro_rules" followed by "!" followed by an identifier
        IDENT if p.at_contextual_kw(T![macro_rules]) && p.nth_at(1, BANG) && p.nth_at(2, IDENT) => {
            macro_rules(p, m)
        }

        T![const] if (la == IDENT || la == T![_] || la == T![mut]) => consts::konst(p, m),
        T![static] if (la == IDENT || la == T![_] || la == T![mut]) => consts::static_(p, m),

        _ => return Err(m),
    };
    Ok(())
}

// test extern_crate
// extern crate foo;
fn extern_crate(p: &mut Parser<'_>, m: Marker) {
    p.bump(T![extern]);
    p.bump(T![crate]);

    if p.at(T![self]) {
        // test extern_crate_self
        // extern crate self;
        let m = p.start();
        p.bump(T![self]);
        m.complete(p, NAME_REF);
    } else {
        name_ref(p);
    }

    // test extern_crate_rename
    // extern crate foo as bar;
    opt_rename(p);
    p.expect(T![;]);
    m.complete(p, EXTERN_CRATE);
}

pub(crate) fn typedef_(p: &mut Parser<'_>, m: Marker) {
    p.bump(T![typedef]);

    // m should really be consumed inside here
    match p.current() {
        T![struct] => adt::strukt_bsv(p, m),
        IDENT if p.at_contextual_kw(T![union]) => adt::union_(p, m),  // vestigial
        T![enum] => adt::enum_(p, m),
        _ => type_synonym_bsv(p, m),
    }
}

// test type_synonym
// typedef BRAM1Port#(Bit#(8), Bit#(32)) CacheBRAM;
fn type_synonym_bsv(p: &mut Parser<'_>, m: Marker) {

    types::type_(p);
    // types::type_bsv(p);  // We'll use this for now.

    // Needs to recover because we often have other items coming up.
    name_r(p, ITEM_RECOVERY_SET);
    generic_params::opt_generic_param_list(p);

    p.expect(T![;]);
    m.complete(p, TYPE_ALIAS);
}

// test type_alias
// type Foo = Bar;
fn type_alias(p: &mut Parser<'_>, m: Marker) {
    p.bump(T![type]);

    name(p);

    // test type_item_type_params
    // type Result<T> = ();
    generic_params::opt_generic_param_list(p);

    if p.at(T![:]) {
        generic_params::bounds(p);
    }

    // test type_item_where_clause_deprecated
    // type Foo where Foo: Copy = ();
    generic_params::opt_where_clause(p);
    if p.eat(T![=]) {
        types::type_(p);
    }

    // test type_item_where_clause
    // type Foo = () where Foo: Copy;
    generic_params::opt_where_clause(p);

    p.expect(T![;]);
    m.complete(p, TYPE_ALIAS);
}

pub(crate) fn item_list(p: &mut Parser<'_>) {
    assert!(p.at(T!['{']));
    let m = p.start();
    p.bump(T!['{']);
    mod_contents(p, true);
    p.expect(T!['}']);
    m.complete(p, ITEM_LIST);
}

pub(crate) fn extern_item_list(p: &mut Parser<'_>) {
    assert!(p.at(T!['{']));
    let m = p.start();
    p.bump(T!['{']);
    mod_contents(p, true);
    p.expect(T!['}']);
    m.complete(p, EXTERN_ITEM_LIST);
}

// test try_macro_rules 2015
// macro_rules! try { () => {} }
fn macro_rules(p: &mut Parser<'_>, m: Marker) {
    assert!(p.at_contextual_kw(T![macro_rules]));
    p.bump_remap(T![macro_rules]);
    p.expect(T![!]);

    name(p);

    match p.current() {
        // test macro_rules_non_brace
        // macro_rules! m ( ($i:ident) => {} );
        // macro_rules! m [ ($i:ident) => {} ];
        T!['['] | T!['('] => {
            token_tree(p);
            p.expect(T![;]);
        }
        T!['{'] => token_tree(p),
        _ => p.error("expected `{`, `[`, `(`"),
    }
    m.complete(p, MACRO_RULES);
}

// test macro_def
// macro m($i:ident) {}
fn macro_def(p: &mut Parser<'_>, m: Marker) {
    p.expect(T![macro]);
    name_r(p, ITEM_RECOVERY_SET);
    if p.at(T!['{']) {
        // test macro_def_curly
        // macro m { ($i:ident) => {} }
        token_tree(p);
    } else if p.at(T!['(']) {
        token_tree(p);
        match p.current() {
            T!['{'] | T!['['] | T!['('] => token_tree(p),
            _ => p.error("expected `{`, `[`, `(`"),
        }
    } else {
        p.error("unmatched `(`");
    }

    m.complete(p, MACRO_DEF);
}

enum BsvType {
    Function,
    Method,
    Rule,
}

// // test fn_
// // fn foo() {}
fn bsv_assoc(p: &mut Parser<'_>, m: Marker, sig_only: bool) {
    fn inner_guard(p: &mut Parser<'_>, has_if: bool) {
        let guard_m = p.start();
        if has_if {
            p.expect(T![if]);
        }
        p.expect(T!['(']);
        if p.at(T![;]) {
            p.error("expected guard");
        } else {
            expr(p);  // May need better error recovery
        }
        p.eat(T![')']);
        guard_m.complete(p, MATCH_GUARD);
    }

    // let has_self;
    // let has_return;
    // let ket;
    let item_type = match p.current() {
        T![function] => {
            // has_self = false;
            // has_return = true;
            p.bump(T![function]);
            // ket = T![endfunction];
            BsvType::Function
        },
        T![method] => {
            // has_self = true;
            // has_return = true;
            p.bump(T![method]);
            // ket = T![endmethod];
            BsvType::Method
        },
        T![rule] => {
            // has_self = false;
            // has_return = false;
            p.bump(T![rule]);
            // ket = T![endrule];
            BsvType::Rule
        }
        _ => {
            unreachable!("should only be reachable on `function`, `method`, `rule`");
        }
    };

    match item_type {
        BsvType::Function | BsvType::Method => {
            // Going to assume we *always* have a return type
            // even if Action. Actual language may be more permissive
            if p.at(IDENT) && !p.nth_at(1, T!['(']) {
                let ret_m = p.start();
                types::type_(p);  // Not sure the difference. It used to be below.
                // types::type_no_bounds(p);
                ret_m.complete(p, RET_TYPE);
            }
        }
        BsvType::Rule => {},
    }

    name_r(p, ITEM_RECOVERY_SET);

    // TODO BSV: Consider Self param for associated functions.
    // Currently we just do a dumb all-methods = Self, all functions = not Self
    let mut did_guard = false;
    if p.at(T!['(']) {
        match item_type {
            BsvType::Function => params::param_list_bsv_function(p),
            BsvType::Method => params::param_list_bsv_method(p),
            BsvType::Rule => {
                // Probable guard
                did_guard = true;
                inner_guard(p, false);
            }
        }
    }  else {
        // Arguments optional in Bluespec, unfortunately.
        // We replace with empty param list so we're sure to interpret
        // as "no parameters" and not "missing parameters"
        let param_list_m = p.start();
        if let BsvType::Method = item_type {
            p.start().complete(p, SELF_PARAM);
        }
        param_list_m.complete(p, PARAM_LIST);
    }

    if p.at(T![provisos]) {
        provisos(p);
    }

    // TODO add guard here
    if p.at(T![if]) {
        if did_guard {
            p.error("second guard detected; maybe rule has forbidden param list?");
        } else {
            did_guard = true;
            inner_guard(p, true);
        }
    }

    if sig_only {
        p.expect(T![;]);
    } else {
        // test function_or_method_no_semi
        // method Action get() = a.get();
        if p.eat(T![=]) {
            p.error("Language server doesn't yet fully implement shorthand assignment");
            expr(p);
            p.expect(T![;]);
        } else {
            let ket = match item_type {
                BsvType::Function => T![endfunction],
                BsvType::Method => T![endmethod],
                BsvType::Rule => T![endrule],
            };
            // TODO_BSV add body: also need to be resilient to nesting.
            expressions::block_expr_bsv(p, None, ket, false, false);
            p.expect(ket);
        }
    }
    m.complete(p, FN);
}

// test fn_
// fn foo() {}
fn fn_(p: &mut Parser<'_>, m: Marker) {
    p.bump(T![fn]);

    name_r(p, ITEM_RECOVERY_SET);
    // test function_type_params
    // fn foo<T: Clone + Copy>(){}
    generic_params::opt_generic_param_list(p);

    if p.at(T!['(']) {
        params::param_list_fn_def(p);
    } else {
        p.error("expected function arguments");
    }
    // test function_ret_type
    // fn foo() {}
    // fn bar() -> () {}
    opt_ret_type(p);

    // test function_where_clause
    // fn foo<T>() where T: Copy {}
    generic_params::opt_where_clause(p);

    // test fn_decl
    // trait T { fn foo(); }
    if !p.eat(T![;]) {
        expressions::block_expr(p);
    }
    m.complete(p, FN);
}

fn macro_call(p: &mut Parser<'_>, m: Marker) {
    assert!(paths::is_use_path_start(p));
    if p.at(T!['`']) {
        expressions::bsv_macro_call(p)
    } else {
        paths::use_path(p);
        match macro_call_after_excl(p) {
            BlockLike::Block => (),
            BlockLike::NotBlock => {
                p.expect(T![;]);
            }
        }
    }
    m.complete(p, MACRO_CALL);
}

// test opt_guard
// rule tick1 if (cond1);
// rule tick2 (cond2);
// rule tick2;
fn opt_guard(p: &mut Parser<'_>) -> bool {
    let guard = p.start();
    p.eat(T![if]);
    if p.at(T!['(']) {
        expr(p);  // TODO BSV: Check for breaking code calling convention
        p.expect(T![')']);
        guard.complete(p, GUARD);
        true
    } else {
        guard.abandon(p);
        false
    }
}

// fn rule(p: &mut Parser<'_>) {
//     let m = p.start();

//     p.bump(T![rule]);

//     // TODO BSV check if rules actually require names
//     name(p);

//     opt_guard(p);

//     if p.eat(T![;]) {
//         expressions::block_expr_bsv(p, None, T![endrule], );
//         p.expect(T![endrule]);
//     }

//     // TODO BSV check if this is an appropriate way to complete without
//     // a body, or if we should be doing an error or something.
//     m.complete(p, RULE);
// }

pub(super) fn macro_call_after_excl(p: &mut Parser<'_>) -> BlockLike {
    p.expect(T![!]);

    match p.current() {
        T!['{'] => {
            token_tree(p);
            BlockLike::Block
        }
        T!['('] | T!['['] => {
            token_tree(p);
            BlockLike::NotBlock
        }
        _ => {
            p.error("expected `{`, `[`, `(`");
            BlockLike::NotBlock
        }
    }
}

pub(crate) fn token_tree(p: &mut Parser<'_>) {
    let closing_paren_kind = match p.current() {
        T!['{'] => T!['}'],
        T!['('] => T![')'],
        T!['['] => T![']'],
        _ => unreachable!(),
    };
    let m = p.start();
    p.bump_any();
    while !p.at(EOF) && !p.at(closing_paren_kind) {
        match p.current() {
            T!['{'] | T!['('] | T!['['] => token_tree(p),
            T!['}'] => {
                p.error("unmatched `}`");
                m.complete(p, TOKEN_TREE);
                return;
            }
            T![')'] | T![']'] => p.err_and_bump("unmatched brace"),
            _ => p.bump_any(),
        }
    }
    p.expect(closing_paren_kind);
    m.complete(p, TOKEN_TREE);
}

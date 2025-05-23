mod atom;

use crate::grammar::attributes::ATTRIBUTE_FIRST;

use super::*;

pub(crate) use atom::{block_expr, match_arm_list, block_expr_bsv};
pub(super) use atom::{literal, LITERAL_FIRST};

#[derive(PartialEq, Eq)]
pub(super) enum Semicolon {
    Required,
    Optional,
    Forbidden,
}

const EXPR_FIRST: TokenSet = LHS_FIRST;

pub(super) fn expr(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let r = Restrictions { forbid_structs: false, prefer_stmt: false };
    expr_bp(p, None, r, 1).map(|(m, _)| m)
}

pub(super) fn expr_stmt(
    p: &mut Parser<'_>,
    m: Option<Marker>,
) -> Option<(CompletedMarker, BlockLike)> {
    let r = Restrictions { forbid_structs: false, prefer_stmt: true };
    expr_bp(p, m, r, 1)
}

fn expr_no_struct(p: &mut Parser<'_>) {
    let r = Restrictions { forbid_structs: true, prefer_stmt: false };
    expr_bp(p, None, r, 1);
}

/// Parses the expression in `let pattern = expression`.
/// It needs to be parsed with lower precedence than `&&`, so that
/// `if let true = true && false` is parsed as `if (let true = true) && (true)`
/// and not `if let true = (true && true)`.
fn expr_let(p: &mut Parser<'_>) {
    let r = Restrictions { forbid_structs: true, prefer_stmt: false };
    expr_bp(p, None, r, 5);
}

pub(super) fn stmt(p: &mut Parser<'_>, semicolon: Semicolon) {
    if p.eat(T![;]) {
        return;
    }

    let m = p.start();
    // test attr_on_expr_stmt
    // fn foo() {
    //     #[A] foo();
    //     #[B] bar!{}
    //     #[C] #[D] {}
    //     #[D] return ();
    // }
    attributes::outer_attrs(p);

    // Bluespec requires a little heuristic for explicit type variable declarations.
    let let_heuristic =
        p.at(IDENT) &&
        (
            (p.nth_at(1, IDENT) &&
            (p.nth_at(2, T![<-]) || p.nth_at(2, T![=])))
        |
            p.nth_at(1, T![#])
        );

    if p.at(T![let]) || p.at(T![match]) || let_heuristic {
        let_stmt(p, semicolon);
        m.complete(p, LET_STMT);
        return;
    }

    // test block_items
    // fn a() { fn b() {} }
    // This is a stopgap to prevent infinite loops, because we don't just terminate at `}`
    // anymore (Bluespec blocks aren't necessarily delimited by curly braces, so we may have)
    // a half open with no way to close. Note that a clean design might require figuring out
    // a better invariant.
    if p.at(T!['}']) {
        // notice p.err_and_bump() actually won't take `{}` tokens.
        p.error("not expected '}'");
        p.bump_any();
        m.abandon(p);
        return;
    }
    let m = match items::opt_item(p, m, false) {
        Ok(()) => return,
        Err(m) => m,
    };

    if !p.at_ts(EXPR_FIRST) {
        p.err_and_bump("expected expression, item or let statement");
        m.abandon(p);
        return;
    }

    if let Some((cm, blocklike)) = expr_stmt(p, Some(m)) {
        if !(p.at(T!['}']) || (semicolon != Semicolon::Required && p.at(EOF))) {
            // test no_semi_after_block
            // fn foo() {
            //     if true {}
            //     loop {}
            //     match () {}
            //     while true {}
            //     for _ in () {}
            //     {}
            //     {}
            //     macro_rules! test {
            //          () => {}
            //     }
            //     test!{}
            // }
            let m = cm.precede(p);
            match semicolon {
                Semicolon::Required => {
                    if blocklike.is_block() {
                        p.eat(T![;]);
                    } else {
                        p.expect(T![;]);
                    }
                }
                Semicolon::Optional => {
                    p.eat(T![;]);
                }
                Semicolon::Forbidden => (),
            }
            m.complete(p, EXPR_STMT);
        }
    }
}

pub(super) fn expect_call(p: &mut Parser<'_>) {
    if let Some((lhs, _)) = expressions::lhs(p, Restrictions { forbid_structs: false, prefer_stmt: false }) {
        match lhs.kind() {
            PATH_EXPR => {expressions::call_expr(p, lhs);}
            FIELD_EXPR => {
                // TODO this should really be named METHOD_CALL_EXPR for proper type
                // resolution later; otherwise it'll seem like we're accessing the
                // method pointer, not making a method call. Either way, it gets resolved.
            }
            CALL_EXPR | METHOD_CALL_EXPR => {}
            _ => {p.error("expected function or method call");}
        }
    } else {
        p.err_and_bump("expected call");
    }
}

// test let_stmt
// fn f() { let x: i32 = 92; }
pub(super) fn let_stmt(p: &mut Parser<'_>, with_semi: Semicolon) {
    // Either have a `let` at the beginning or a type.
    if !p.eat(T![let]) && !p.eat(T![match]) {
        types::ascription(p);
    }

    patterns::pattern(p);

    if p.eat(T![=]) {
        // test let_stmt_init
        // fn f() { let x = 92; }
        expressions::expr(p);
    } else if p.eat(T![<-]) {
        expect_call(p);
    }

    // Not supported in Bluespec
    // if p.at(T![else]) {
    //     // test_err let_else_right_curly_brace
    //     // fn func() { let Some(_) = {Some(1)} else { panic!("h") };}
    //     if let Some(expr) = expr_after_eq {
    //         if BlockLike::is_blocklike(expr.kind()) {
    //             p.error(
    //                 "right curly brace `}` before `else` in a `let...else` statement not allowed",
    //             )
    //         }
    //     }

    //     // test let_else
    //     // fn f() { let Some(x) = opt else { return }; }
    //     let m = p.start();
    //     p.bump(T![else]);
    //     block_expr(p);
    //     m.complete(p, LET_ELSE);
    // }

    match with_semi {
        Semicolon::Forbidden => (),
        Semicolon::Optional => {
            p.eat(T![;]);
        }
        Semicolon::Required => {
            p.expect(T![;]);
        }
    }
}

// // test method_decl
// // method Bit#(4) get_count(Bool tag);
// pub(super) fn method_decl(p: &mut Parser<'_>) {
//     let m = p.start();
//     method_signature(p);
//     p.expect(T![;]);
//     m.complete(p, METHOD_DECL);
// }

// // test method_signature
// // method Bit#(4) get_count(Bool tag);
// pub(super) fn method_signature(p: &mut Parser<'_>) {
//     let m = p.start();
//     p.bump(T![method]);
//     type_bsv(p);
//     name(p);
//     if p.at(T!['(']) {
//         params::param_list_bsv(p);
//     }
//     m.complete(p, METHOD_SIGNATURE);
// }

// // test function_signature
// // function Bit#(4) get_count(Bool tag);
// pub(super) fn function_signature(p: &mut Parser<'_>) {
//     let m = p.start();
//     p.bump(T![function]);
//     type_bsv(p);
//     name(p);
//     if p.at(T!['(']) {
//         params::param_list_bsv(p);
//     }
//     m.complete(p, FUNCTION_SIGNATURE);
// }

// pub(super) fn instantiation(p: &mut Parser<'_>) {
//     let m = p.start();
//     typed_var_bsv(p);

//     if !(p.eat(T![<-]) || p.eat(T![=])) {
//         p.err_and_bump("Expected a `<-` or `=` operator");
//     }
//     // test literal_in_module_inst
//     // String out_file = "output.log";
//     expr(p);

//     p.expect(T![;]);
//     m.complete(p, INSTANTIATION);
// }

pub(super) fn module_call_bsv(p: &mut Parser<'_>) {
    let m = p.start();
    name_ref(p);
    if p.at(T!['(']) {
        arg_list(p);  // TODO BSV: Migrate over to BSV arg_list
    }
    m.complete(p, MODULE_CALL);
}

// Not everything has a bra and ket. Function bodies only have a ket (whole signature is "bra")
// while `begin/end` and `beginaction` and `endaction`, etc., have both.
pub(super) fn stmt_list_bsv(p: &mut Parser<'_>, bra: Option<SyntaxKind>, end: SyntaxKind, include_ket: bool, single: bool) {
    let m = p.start();

    if let Some(bra) = bra {
        p.expect(bra);
    }
    // attributes::inner_attrs(p);

    while !p.at(EOF) && !p.at(end) {
        // Little bit of fault recovery.
        if p.at(T![endmodule]) {
            break;
        }
        stmt(p, Semicolon::Required);
        if single {  // e.g., only one statement expected, like in BSV if.
            break;
        }
    }

    if include_ket{
        p.expect(end);
    }
    m.complete(p, STMT_LIST);
}

pub(super) fn expr_block_contents(p: &mut Parser<'_>) {
    attributes::inner_attrs(p);

    while !p.at(EOF) && !p.at(T!['}']) {
        // test nocontentexpr
        // fn foo(){
        //     ;;;some_expr();;;;{;;;};;;;Ok(())
        // }

        // test nocontentexpr_after_item
        // fn simple_function() {
        //     enum LocalEnum {
        //         One,
        //         Two,
        //     };
        //     fn f() {};
        //     struct S {};
        // }
        stmt(p, Semicolon::Required);
    }
}

#[derive(Clone, Copy)]
struct Restrictions {
    forbid_structs: bool,
    prefer_stmt: bool,
}

enum Associativity {
    Left,
    Right,
}

/// Binding powers of operators for a Pratt parser.
///
/// See <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html>
///
/// Note that Rust doesn't define associativity for some infix operators (e.g. `==` and `..`) and
/// requires parentheses to disambiguate. We just treat them as left associative.
#[rustfmt::skip]
fn current_op(p: &Parser<'_>) -> (u8, SyntaxKind, Associativity) {
    use Associativity::*;
    const NOT_AN_OP: (u8, SyntaxKind, Associativity) = (0, T![@], Left);
    match p.current() {
        T![|] if p.at(T![||])  => (3,  T![||],  Left),
        T![|] if p.at(T![|=])  => (1,  T![|=],  Right),
        T![|]                  => (6,  T![|],   Left),
        T![?]                  => (2,  T![?],   Right),  // binds strongly
        T![:]                  => (3,  T![:],   Right),  // binds strongly
        T![>] if p.at(T![>>=]) => (1,  T![>>=], Right),
        T![>] if p.at(T![>>])  => (9,  T![>>],  Left),
        T![>] if p.at(T![>=])  => (5,  T![>=],  Left),
        T![>]                  => (5,  T![>],   Left),
        T![=] if p.at(T![==])  => (5,  T![==],  Left),
        T![=] if !p.at(T![=>]) => (1,  T![=],   Right),
        T![<] if p.at(T![<=])  => (5,  T![<=],  Left),
        T![<] if p.at(T![<<=]) => (1,  T![<<=], Right),
        T![<] if p.at(T![<<])  => (9,  T![<<],  Left),
        T![<] if p.at(T![<-])  => (1,  T![<-],  Left),
        T![<]                  => (5,  T![<],   Left),
        T![+] if p.at(T![+=])  => (1,  T![+=],  Right),
        T![+]                  => (10, T![+],   Left),
        T![^] if p.at(T![^=])  => (1,  T![^=],  Right),
        T![^]                  => (7,  T![^],   Left),
        T![%] if p.at(T![%=])  => (1,  T![%=],  Right),
        T![%]                  => (11, T![%],   Left),
        T![&] if p.at(T![&=])  => (1,  T![&=],  Right),
        // If you update this, remember to update `expr_let()` too.
        T![&] if p.at(T![&&])  => (4,  T![&&],  Left),
        T![&]                  => (8,  T![&],   Left),
        T![/] if p.at(T![/=])  => (1,  T![/=],  Right),
        T![/]                  => (11, T![/],   Left),
        T![*] if p.at(T![*=])  => (1,  T![*=],  Right),
        T![*]                  => (11, T![*],   Left),
        T![.] if p.at(T![..=]) => (2,  T![..=], Left),
        T![.] if p.at(T![..])  => (2,  T![..],  Left),
        T![!] if p.at(T![!=])  => (5,  T![!=],  Left),
        T![-] if p.at(T![-=])  => (1,  T![-=],  Right),
        T![-]                  => (10, T![-],   Left),
        T![as]                 => (12, T![as],  Left),

        _                      => NOT_AN_OP
    }
}

// Abandon previous marker and start a new one after the macro call.
// Two call sites: one for items, one for expressions. Hopefully it covers
// the full possibilities.
pub(super) fn opt_bsv_grave(p: &mut Parser<'_>, m: Marker) -> Marker {
    if opt_bsv_macro_calls(p) {
        m.abandon(p);
        p.start()
    } else {
        m
    }
}

fn opt_bsv_macro_calls(p: &mut Parser<'_>) -> bool {
    let mut any = false;
    while p.at(T!['`']) {
        let m = p.start();
        expressions::bsv_macro_call(p);
        m.complete(p, MACRO_CALL);
        any = true;
    }
    any
}

// Parses expression with binding power of at least bp.
fn expr_bp(
    p: &mut Parser<'_>,
    m: Option<Marker>,
    r: Restrictions,
    bp: u8,
) -> Option<(CompletedMarker, BlockLike)> {
    let m = m.unwrap_or_else(|| {
        let m = p.start();
        attributes::outer_attrs(p);
        m
    });

    // Bluespec allows arbitrary macro calls anywhere. We just take them here.
    let m = opt_bsv_grave(p, m);

    if !p.at_ts(EXPR_FIRST) {
        p.err_recover("expected expression", atom::EXPR_RECOVERY_SET);
        m.abandon(p);
        return None;
    }
    let mut lhs = match lhs(p, r) {
        Some((lhs, blocklike)) => {
            let lhs = lhs.extend_to(p, m);
            if r.prefer_stmt && blocklike.is_block() {
                // test stmt_bin_expr_ambiguity
                // fn f() {
                //     let _ = {1} & 2;
                //     {1} &2;
                // }
                return Some((lhs, BlockLike::Block));
            }
            lhs
        }
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        let is_range = p.at(T![..]) || p.at(T![..=]);
        let (op_bp, op, associativity) = current_op(p);
        if op_bp < bp {
            break;
        }
        // test as_precedence
        // fn f() { let _ = &1 as *const i32; }
        if p.at(T![as]) {
            lhs = cast_expr(p, lhs);
            continue;
        }
        let m = lhs.precede(p);
        p.bump(op);

        if is_range {
            // test postfix_range
            // fn foo() {
            //     let x = 1..;
            //     match 1.. { _ => () };
            //     match a.b()..S { _ => () };
            // }
            let has_trailing_expression =
                p.at_ts(EXPR_FIRST) && !(r.forbid_structs && p.at(T!['{']));
            if !has_trailing_expression {
                // no RHS
                lhs = m.complete(p, RANGE_EXPR);
                break;
            }
        }

        let op_bp = match associativity {
            Associativity::Left => op_bp + 1,
            Associativity::Right => op_bp,
        };

        // test binop_resets_statementness
        // fn f() { v = {1}&2; }
        expr_bp(p, None, Restrictions { prefer_stmt: false, ..r }, op_bp);
        lhs = m.complete(p, if is_range { RANGE_EXPR } else { BIN_EXPR });
    }
    Some((lhs, BlockLike::NotBlock))
}

const LHS_FIRST: TokenSet =
    atom::ATOM_EXPR_FIRST.union(TokenSet::new(&[T![&], T![*], T![!], T![.], T![-], T![~], T![_], T![?]]));

fn lhs(p: &mut Parser<'_>, r: Restrictions) -> Option<(CompletedMarker, BlockLike)> {
    let m;
    let kind = match p.current() {
        // test ref_expr
        // fn foo() {
        //     // reference operator
        //     let _ = &1;
        //     let _ = &mut &f();
        //     let _ = &raw;
        //     let _ = &raw.0;
        //     // raw reference operator
        //     let _ = &raw mut foo;
        //     let _ = &raw const foo;
        // }
        T![&] => {
            m = p.start();
            p.bump(T![&]);
            if p.at_contextual_kw(T![raw]) && [T![mut], T![const]].contains(&p.nth(1)) {
                p.bump_remap(T![raw]);
                p.bump_any();
            } else {
                p.eat(T![mut]);
            }
            REF_EXPR
        }
        // test unary_expr
        // fn foo() {
        //     **&1;
        //     !!true;
        //     --1;
        // }
        T![*] | T![!] | T![-] | T![~] => {
            m = p.start();
            p.bump_any();
            PREFIX_EXPR
        }
        _ => {
            // test full_range_expr
            // fn foo() { xs[..]; }
            for op in [T![..=], T![..]] {
                if p.at(op) {
                    m = p.start();
                    p.bump(op);

                    // test closure_range_method_call
                    // fn foo() {
                    //     || .. .method();
                    //     || .. .field;
                    // }
                    let has_access_after = p.at(T![.]) && p.nth_at(1, SyntaxKind::IDENT);
                    let struct_forbidden = r.forbid_structs && p.at(T!['{']);
                    if p.at_ts(EXPR_FIRST) && !has_access_after && !struct_forbidden {
                        expr_bp(p, None, r, 2);
                    }
                    let cm = m.complete(p, RANGE_EXPR);
                    return Some((cm, BlockLike::NotBlock));
                }
            }

            // test expression_after_block
            // fn foo() {
            //    let mut p = F{x: 5};
            //    {p}.x = 10;
            // }
            let (lhs, blocklike) = atom::atom_expr(p, r)?;
            let (cm, block_like) =
                postfix_expr(p, lhs, blocklike, !(r.prefer_stmt && blocklike.is_block()));
            return Some((cm, block_like));
        }
    };
    // parse the interior of the unary expression
    expr_bp(p, None, r, 255);
    let cm = m.complete(p, kind);
    Some((cm, BlockLike::NotBlock))
}

fn postfix_expr(
    p: &mut Parser<'_>,
    mut lhs: CompletedMarker,
    // Calls are disallowed if the type is a block and we prefer statements because the call cannot be disambiguated from a tuple
    // E.g. `while true {break}();` is parsed as
    // `while true {break}; ();`
    mut block_like: BlockLike,
    mut allow_calls: bool,
) -> (CompletedMarker, BlockLike) {
    loop {
        lhs = match p.current() {
            // test stmt_postfix_expr_ambiguity
            // fn foo() {
            //     match () {
            //         _ => {}
            //         () => {}
            //         [] => {}
            //     }
            // }
            T!['('] if allow_calls => call_expr(p, lhs),
            T!['['] if allow_calls => index_expr(p, lhs),
            T![.] => match postfix_dot_expr::<false>(p, lhs) {
                Ok(it) => it,
                Err(it) => {
                    lhs = it;
                    break;
                }
            },
            // T![?] => try_expr(p, lhs),
            _ => break,
        };
        allow_calls = true;
        block_like = BlockLike::NotBlock;
    }
    (lhs, block_like)
}

fn postfix_dot_expr<const FLOAT_RECOVERY: bool>(
    p: &mut Parser<'_>,
    lhs: CompletedMarker,
) -> Result<CompletedMarker, CompletedMarker> {
    if !FLOAT_RECOVERY {
        assert!(p.at(T![.]));
    }
    let nth1 = if FLOAT_RECOVERY { 0 } else { 1 };
    let nth2 = if FLOAT_RECOVERY { 1 } else { 2 };

    if p.nth(nth1) == IDENT && (p.nth(nth2) == T!['('] || p.nth_at(nth2, T![::])) {
        return Ok(method_call_expr::<FLOAT_RECOVERY>(p, lhs));
    }

    // test await_expr
    // fn foo() {
    //     x.await;
    //     x.0.await;
    //     x.0().await?.hello();
    //     x.0.0.await;
    //     x.0. await;
    // }
    if p.nth(nth1) == T![await] {
        let m = lhs.precede(p);
        if !FLOAT_RECOVERY {
            p.bump(T![.]);
        }
        p.bump(T![await]);
        return Ok(m.complete(p, AWAIT_EXPR));
    }

    if p.at(T![..=]) || p.at(T![..]) {
        return Err(lhs);
    }

    field_expr::<FLOAT_RECOVERY>(p, lhs)
}

// test call_expr
// fn foo() {
//     let _ = f();
//     let _ = f()(1)(1, 2,);
//     let _ = f(<Foo>::func());
//     f(<Foo as Trait>::func());
// }
fn call_expr(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    // assert!(p.at(T!['(']));
    let m = lhs.precede(p);
    arg_list(p);
    m.complete(p, CALL_EXPR)
}

// test index_expr
// fn foo() {
//     x[1][2];
// }
fn index_expr(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(T!['[']));
    let m = lhs.precede(p);
    p.bump(T!['[']);
    expr(p);
    p.expect(T![']']);
    m.complete(p, INDEX_EXPR)
}

// test method_call_expr
// fn foo() {
//     x.foo();
//     y.bar::<T>(1, 2,);
//     x.0.0.call();
//     x.0. call();
// }
fn method_call_expr<const FLOAT_RECOVERY: bool>(
    p: &mut Parser<'_>,
    lhs: CompletedMarker,
) -> CompletedMarker {
    if FLOAT_RECOVERY {
        assert!(p.nth(0) == IDENT && (p.nth(1) == T!['('] || p.nth_at(1, T![::])));
    } else {
        assert!(p.at(T![.]) && p.nth(1) == IDENT && (p.nth(2) == T!['('] || p.nth_at(2, T![::])));
    }
    let m = lhs.precede(p);
    if !FLOAT_RECOVERY {
        p.bump(T![.]);
    }
    name_ref(p);
    generic_args::opt_generic_arg_list(p, true);
    arg_list(p);
    m.complete(p, METHOD_CALL_EXPR)
}

// test field_expr
// fn foo() {
//     x.foo;
//     x.0.bar;
//     x.0.1;
//     x.0. bar;
//     x.0();
// }
fn field_expr<const FLOAT_RECOVERY: bool>(
    p: &mut Parser<'_>,
    lhs: CompletedMarker,
) -> Result<CompletedMarker, CompletedMarker> {
    if !FLOAT_RECOVERY {
        assert!(p.at(T![.]));
    }
    let m = lhs.precede(p);
    if !FLOAT_RECOVERY {
        p.bump(T![.]);
    }
    if p.at(IDENT) || p.at(INT_NUMBER) {
        name_ref_or_index(p);
    } else if p.at(FLOAT_NUMBER) {
        return match p.split_float(m) {
            (true, m) => {
                let lhs = m.complete(p, FIELD_EXPR);
                postfix_dot_expr::<true>(p, lhs)
            }
            (false, m) => Ok(m.complete(p, FIELD_EXPR)),
        };
    } else {
        p.error("expected field name or number");
    }
    Ok(m.complete(p, FIELD_EXPR))
}

// test try_expr
// fn foo() {
//     x?;
// }
fn try_expr(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(T![?]));
    let m = lhs.precede(p);
    p.bump(T![?]);
    m.complete(p, TRY_EXPR)
}

// test cast_expr
// fn foo() {
//     82 as i32;
//     81 as i8 + 1;
//     79 as i16 - 1;
//     0x36 as u8 <= 0x37;
// }
fn cast_expr(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(T![as]));
    let m = lhs.precede(p);
    p.bump(T![as]);
    // Use type_no_bounds(), because cast expressions are not
    // allowed to have bounds.
    types::type_no_bounds(p);
    m.complete(p, CAST_EXPR)
}

// test_err arg_list_recovery
// fn main() {
//     foo(bar::);
//     foo(bar:);
//     foo(bar+);
//     foo(a, , b);
// }
pub(crate) fn arg_list(p: &mut Parser<'_>) {
    let m = p.start();
    if !p.at(T!['(']) {
        p.error("parentheses strongly encouraged for arg-less calls");
        m.complete(p, ARG_LIST);  // empty args
        return;
    }
    // test arg_with_attr
    // fn main() {
    //     foo(#[attr] 92)
    // }
    delimited(
        p,
        T!['('],
        T![')'],
        T![,],
        || "expected expression".into(),
        EXPR_FIRST.union(ATTRIBUTE_FIRST),
        |p| expr(p).is_some(),
    );
    m.complete(p, ARG_LIST);
}

// `idfef
// notice how this has two call sites. In reality,
// there shouldn't be a difference between when we use
// this as an "item" vs when we use this as an "expr"
// and in fact we're not really supposed to use it as an
// expr. It's similar to C preprocessing.
pub(crate) fn bsv_macro_call(p: &mut Parser<'_>) {
    assert!(p.at(T!['`']));
    p.bump(T!['`']);
    match p.current() {
        T![else] | T![endif] | T![define]=> p.bump_any(),
        T![ifdef] | T![elsif]=> {
            p.bump_any();
            paths::expr_path(p);
        }
        _ => p.err_and_bump("Macro call not recognized"),
    }
}

// test path_expr
// fn foo() {
//     let _ = a;
//     let _ = a::b;
//     let _ = ::a::<b>;
//     let _ = format!();
// }
fn path_expr(p: &mut Parser<'_>, r: Restrictions) -> (CompletedMarker, BlockLike) {
    assert!(paths::is_path_start(p));
    let m = p.start();
    paths::expr_path(p);
    match p.current() {
        T!['{'] if !r.forbid_structs => {
            record_expr_field_list(p);
            (m.complete(p, RECORD_EXPR), BlockLike::NotBlock)
        }
        T![!] if !p.at(T![!=]) => {
            let block_like = items::macro_call_after_excl(p);
            (m.complete(p, MACRO_CALL).precede(p).complete(p, MACRO_EXPR), block_like)
        }
        _ => (m.complete(p, PATH_EXPR), BlockLike::NotBlock),
    }
}

// test record_lit
// fn foo() {
//     S {};
//     S { x };
//     S { x, y: 32, };
//     S { x, y: 32, ..Default::default() };
//     S { x: ::default() };
//     TupleStruct { 0: 1 };
// }
pub(crate) fn record_expr_field_list(p: &mut Parser<'_>) {
    assert!(p.at(T!['{']));
    let m = p.start();
    p.bump(T!['{']);
    while !p.at(EOF) && !p.at(T!['}']) {
        let m = p.start();
        // test record_literal_field_with_attr
        // fn main() {
        //     S { #[cfg(test)] field: 1 }
        // }
        attributes::outer_attrs(p);

        match p.current() {
            IDENT | INT_NUMBER if p.nth_at(1, T![::]) => {
                // test_err record_literal_missing_ellipsis_recovery
                // fn main() {
                //     S { S::default() }
                // }
                m.abandon(p);
                p.expect(T![..]);
                expr(p);
            }
            IDENT | INT_NUMBER => {
                if p.nth_at(1, T![..]) {
                    // test_err record_literal_before_ellipsis_recovery
                    // fn main() {
                    //     S { field ..S::default() }
                    // }
                    name_ref_or_index(p);
                    p.error("expected `:`");
                } else {
                    // test_err record_literal_field_eq_recovery
                    // fn main() {
                    //     S { field = foo }
                    // }
                    if p.nth_at(1, T![:]) {
                        name_ref_or_index(p);
                        p.bump(T![:]);
                    } else if p.nth_at(1, T![=]) {
                        name_ref_or_index(p);
                        p.err_and_bump("expected `:`");
                    }
                    expr(p);
                }
                m.complete(p, RECORD_EXPR_FIELD);
            }
            T![.] if p.at(T![..]) => {
                m.abandon(p);
                p.bump(T![..]);

                // test destructuring_assignment_struct_rest_pattern
                // fn foo() {
                //     S { .. } = S {};
                // }

                // We permit `.. }` on the left-hand side of a destructuring assignment.
                if !p.at(T!['}']) {
                    expr(p);

                    if p.at(T![,]) {
                        // test_err comma_after_functional_update_syntax
                        // fn foo() {
                        //     S { ..x, };
                        //     S { ..x, a: 0 }
                        // }

                        // Do not bump, so we can support additional fields after this comma.
                        p.error("cannot use a comma after the base struct");
                    }
                }
            }
            T!['{'] => {
                error_block(p, "expected a field");
                m.abandon(p);
            }
            _ => {
                p.err_and_bump("expected identifier");
                m.abandon(p);
            }
        }
        if !p.at(T!['}']) {
            p.expect(T![,]);
        }
    }
    p.expect(T!['}']);
    m.complete(p, RECORD_EXPR_FIELD_LIST);
}

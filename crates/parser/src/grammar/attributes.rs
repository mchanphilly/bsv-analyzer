use expressions::literal;

use super::*;

pub(super) const ATTRIBUTE_FIRST: TokenSet = TokenSet::new(&[T![#]]);

pub(super) fn inner_attrs(p: &mut Parser<'_>) {
    while p.at(T![#]) && p.nth(1) == T![!] {
        attr(p, true);
    }
}

pub(super) fn outer_attrs_bsv(p: &mut Parser<'_>) {
    if p.at(T!['(']) && p.nth(1) == T![*] {
        let m = p.start();
        p.bump(T!['(']);
        p.bump(T![*]);

        // Not at end of file or end of attribute.
        // TODO BSV resilience
        let mut cont = true;
        while cont {
            meta_bsv(p);
            cont = p.eat(T![,]);
            // TODO BSV: Shouldn't have trailing commas
        }

        // TODO BSV resilience
        p.eat(T![*]);
        p.eat(T![')']);
        m.complete(p, ATTR_BSV);
    }
}

pub(super) fn outer_attrs(p: &mut Parser<'_>) {
    while p.at(T![#]) {
        attr(p, false);
    }
}

fn attr(p: &mut Parser<'_>, inner: bool) {
    assert!(p.at(T![#]));

    let attr = p.start();
    p.bump(T![#]);

    if inner {
        p.bump(T![!]);
    }

    if p.eat(T!['[']) {
        meta(p);

        if !p.eat(T![']']) {
            p.error("expected `]`");
        }
    } else {
        p.error("expected `[`");
    }
    attr.complete(p, ATTR);
}

// test meta_bsv
// (* synthesize *)
// (* fire_when_enabled, no_implicit_conditions *)
// (* descending_urgency = "pRqTransfer, flushTransfer" *)
pub(super) fn meta_bsv(p: &mut Parser<'_>) {
    if p.current() == IDENT {
        let meta = p.start();
        name_ref(p);
        if p.eat(T![=]) {  // if it has a value
            literal(p);
        }
        meta.complete(p, ATTR_META_BSV);
    }
}

// test metas
// #![simple_ident]
// #![simple::path]
// #![simple_ident_expr = ""]
// #![simple::path::Expr = ""]
// #![simple_ident_tt(a b c)]
// #![simple_ident_tt[a b c]]
// #![simple_ident_tt{a b c}]
// #![simple::path::tt(a b c)]
// #![simple::path::tt[a b c]]
// #![simple::path::tt{a b c}]
// #![unsafe(simple_ident)]
// #![unsafe(simple::path)]
// #![unsafe(simple_ident_expr = "")]
// #![unsafe(simple::path::Expr = "")]
// #![unsafe(simple_ident_tt(a b c))]
// #![unsafe(simple_ident_tt[a b c])]
// #![unsafe(simple_ident_tt{a b c})]
// #![unsafe(simple::path::tt(a b c))]
// #![unsafe(simple::path::tt[a b c])]
// #![unsafe(simple::path::tt{a b c})]
pub(super) fn meta(p: &mut Parser<'_>) {
    let meta = p.start();
    let is_unsafe = p.eat(T![unsafe]);
    if is_unsafe {
        p.expect(T!['(']);
    }
    paths::use_path(p);

    match p.current() {
        T![=] => {
            p.bump(T![=]);
            if expressions::expr(p).is_none() {
                p.error("expected expression");
            }
        }
        T!['('] | T!['['] | T!['{'] => items::token_tree(p),
        _ => {}
    }
    if is_unsafe {
        p.expect(T![')']);
    }

    meta.complete(p, META);
}

use super::*;

// test interface_item
// interface Ifc; endinterface : Ifc
pub(crate) fn interface_(p: &mut Parser<'_>, m: Marker, sig_only: bool) -> CompletedMarker {
    p.bump(T![interface]);

    // Interface name
    name_r(p, ITEM_RECOVERY_SET);

    // test interface_item_generic_params
    // interface Ifc#(numeric type N, type T); endinterface;
    generic_params::opt_generic_param_list(p);

    if p.eat(T![;]) {
        borderless_assoc_item_list(p, T![endinterface], sig_only);
    } else {
        p.error("expected `;`");
    }

    p.expect(T![endinterface]);
    if p.eat(T![:]) {  // optional end tag
        name_ref_r(p, ITEM_RECOVERY_SET);
    }

    m.complete(p, TRAIT)
}

// test trait_item
// trait T { fn new() -> Self; }
pub(super) fn trait_(p: &mut Parser<'_>, m: Marker) {
    p.bump(T![trait]);
    name_r(p, ITEM_RECOVERY_SET);

    // test trait_item_generic_params
    // trait X<U: Debug + Display> {}
    generic_params::opt_generic_param_list(p);

    if p.eat(T![=]) {
        // test trait_alias
        // trait Z<U> = T<U>;
        generic_params::bounds_without_colon(p);

        // test trait_alias_where_clause
        // trait Z<U> = T<U> where U: Copy;
        // trait Z<U> = where Self: T<U>;
        generic_params::opt_where_clause(p);
        p.expect(T![;]);
        m.complete(p, TRAIT_ALIAS);
        return;
    }

    if p.at(T![:]) {
        // test trait_item_bounds
        // trait T: Hash + Clone {}
        generic_params::bounds(p);
    }

    // test trait_item_where_clause
    // trait T where Self: Copy {}
    generic_params::opt_where_clause(p);

    if p.at(T!['{']) {
        assoc_item_list(p);
    } else {
        p.error("expected `{`");
    }
    m.complete(p, TRAIT);
}

// test impl_item
// impl S {}
pub(super) fn impl_(p: &mut Parser<'_>, m: Marker) {
    p.bump(T![impl]);
    if p.at(T![<]) && not_a_qualified_path(p) {
        generic_params::opt_generic_param_list(p);
    }

    // test impl_item_const
    // impl const Send for S {}
    p.eat(T![const]);

    // FIXME: never type
    // impl ! {}

    // test impl_item_neg
    // impl !Send for S {}
    p.eat(T![!]);
    impl_type(p);
    if p.eat(T![for]) {
        impl_type(p);
    }
    generic_params::opt_where_clause(p);
    if p.at(T!['{']) {
        assoc_item_list(p);
    } else {
        p.error("expected `{`");
    }
    m.complete(p, IMPL);
}

pub(super) fn provisos(p: &mut Parser<'_>) {
    let m = p.start();
    p.bump(T![provisos]);

    generic_args::bsv_generic_arg_list(p);

    m.complete(p, PROVISOS);
}

pub(super) fn module_(p: &mut Parser<'_>, m: Marker) {
    p.bump(T![module]);

    name_r(p, ITEM_RECOVERY_SET);

    generic_params::opt_generic_param_list(p);

    p.expect(T!['(']);
    impl_type(p);
    p.expect(T![')']);

    if p.at(T![provisos]) {
        provisos(p);
    }
    p.expect(T![;]);

    expressions::block_expr_bsv(p, None, T![endmodule], false, false);

    p.expect(T![endmodule]);

    m.complete(p, IMPL);
}


// TODO_BSV
// // test module_item
// // module mkTop (Empty);
// // endmodule : mkTop
// pub(super) fn module_(p: &mut Parser<'_>, m: Marker) {
//     // TODO BSV: Consider consuming the signature separately
//     p.bump(T![module]);

//     name_r(p, ITEM_RECOVERY_SET);

//     // Interface name
//     if p.eat(T!['(']) {
//         name_ref(p);
//     } else {
//         p.error("expected `{`");
//     }
//     p.expect(T![')']);
//     p.expect(T![;]);

//     // TODO BSV: how do we make sure this doesn't eat file on bad inputs?
//     while !(p.at(EOF) || p.at(T![endmodule])) {
//         module_stmt(p);  // TODO BSV open up to different kinds
//     }

//     p.expect(T![endmodule]);
//     if p.eat(T![:]) {  // optional end tag
//         name(p);
//     }
//     m.complete(p, MODULE_BSV);
// }

// test borderless_assoc_item_list
// interface F {
//     method Action foo();
// endinterface
pub(crate) fn borderless_assoc_item_list(p: &mut Parser<'_>, end: SyntaxKind, sig_only: bool) {
    let m = p.start();

    // // test borderless_assoc_item_list_inner_attrs
    // // impl S { #![attr] }
    // attributes::inner_attrs(p);

    while !p.at(EOF) && !p.at(end) {
        // if p.at(T!['{']) {  // TODO_BSV error resilience
        //     error_block(p, "expected an item");
        //     continue;
        // }
        let item_m = p.start();
        attributes::outer_attrs_bsv(p);  // TODO BSV truly add support for outer_attrs

        let item_m = match opt_item(p, item_m, sig_only) {
            Ok(()) => {
                if p.at(T![;]) {
                    p.err_and_bump(
                        "expected item, found `;`\n\
                         consider removing this semicolon",
                    );
                }
                continue;
            }
            Err(m) => m,
        };
        if paths::is_use_path_start(p) {
            // Not really the appropriate thing to use here, 
            // since there are no macros like this in Bluespec,
            // but I'm having trouble getting completion hints
            // otherwise.
            macro_call(p, item_m);
            continue;
        }
        item_m.abandon(p);
        p.error("expected item");
    }
    m.complete(p, ASSOC_ITEM_LIST);
}

// test assoc_item_list
// impl F {
//     type A = i32;
//     const B: i32 = 92;
//     fn foo() {}
//     fn bar(&self) {}
// }
pub(crate) fn assoc_item_list(p: &mut Parser<'_>) {
    // TODO_BSV hypothesis: Reparser still relies on this for Trait and Impl
    // SyntaxNodes, but if they were derived from Bluespec-style parsing
    // then they will panic upon entering this. Check if that's true.
    panic!("assoc_item_list deprecated");
    assert!(p.at(T!['{']));

    let m = p.start();
    p.bump(T!['{']);
    // test assoc_item_list_inner_attrs
    // impl S { #![attr] }
    attributes::inner_attrs(p);

    while !p.at(EOF) && !p.at(T!['}']) {
        if p.at(T!['{']) {
            error_block(p, "expected an item");
            continue;
        }
        item_or_macro(p, true);
    }
    p.expect(T!['}']);
    m.complete(p, ASSOC_ITEM_LIST);
}

// test impl_type_params
// impl<const N: u32> Bar<N> {}
fn not_a_qualified_path(p: &Parser<'_>) -> bool {
    // There's an ambiguity between generic parameters and qualified paths in impls.
    // If we see `<` it may start both, so we have to inspect some following tokens.
    // The following combinations can only start generics,
    // but not qualified paths (with one exception):
    //     `<` `>` - empty generic parameters
    //     `<` `#` - generic parameters with attributes
    //     `<` `const` - const generic parameters
    //     `<` (LIFETIME_IDENT|IDENT) `>` - single generic parameter
    //     `<` (LIFETIME_IDENT|IDENT) `,` - first generic parameter in a list
    //     `<` (LIFETIME_IDENT|IDENT) `:` - generic parameter with bounds
    //     `<` (LIFETIME_IDENT|IDENT) `=` - generic parameter with a default
    // The only truly ambiguous case is
    //     `<` IDENT `>` `::` IDENT ...
    // we disambiguate it in favor of generics (`impl<T> ::absolute::Path<T> { ... }`)
    // because this is what almost always expected in practice, qualified paths in impls
    // (`impl <Type>::AssocTy { ... }`) aren't even allowed by type checker at the moment.
    if [T![#], T![>], T![const]].contains(&p.nth(1)) {
        return true;
    }
    ([LIFETIME_IDENT, IDENT].contains(&p.nth(1)))
        && ([T![>], T![,], T![:], T![=]].contains(&p.nth(2)))
}

// test_err impl_type
// impl Type {}
// impl Trait1 for T {}
// impl impl NotType {}
// impl Trait2 for impl NotType {}
pub(crate) fn impl_type(p: &mut Parser<'_>) {
    if p.at(T![impl]) {
        p.error("expected trait or type");
        return;
    }
    types::type_(p);
}

// test_err impl_type
// impl Type {}
// impl Trait1 for T {}
// impl impl NotType {}
// impl Trait2 for impl NotType {}
pub(crate) fn module_type(p: &mut Parser<'_>) {
    if p.at(T![module]) {
        p.error("expected type");
        return;
    }
    types::type_(p);
}

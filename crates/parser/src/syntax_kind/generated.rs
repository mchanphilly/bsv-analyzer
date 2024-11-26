//! Generated by `cargo codegen grammar`, do not edit by hand.

#![allow(bad_style, missing_docs, unreachable_pub)]
use crate::Edition;
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    DOLLAR,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    AT,
    POUND,
    TILDE,
    QUESTION,
    AMP,
    PIPE,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    UNDERSCORE,
    DOT,
    DOT2,
    DOT3,
    DOT2EQ,
    COLON,
    COLON2,
    EQ,
    EQ2,
    FAT_ARROW,
    BANG,
    NEQ,
    MINUS,
    R_THIN_ARROW,
    L_THIN_ARROW,
    LTEQ,
    GTEQ,
    PLUSEQ,
    MINUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AMP2,
    PIPE2,
    SHL,
    SHR,
    SHLEQ,
    SHREQ,
    ACTION_KW,
    ACTION_VALUE_KW,
    SELF_TYPE_KW,
    ABSTRACT_KW,
    AS_KW,
    BECOME_KW,
    BEGIN_KW,
    BOX_KW,
    BREAK_KW,
    CONST_KW,
    CONTINUE_KW,
    CRATE_KW,
    DO_KW,
    ELSE_KW,
    END_KW,
    ENDFUNCTION_KW,
    ENDINTERFACE_KW,
    ENDMETHOD_KW,
    ENDMODULE_KW,
    ENDRULE_KW,
    ENUM_KW,
    EXTERN_KW,
    FALSE_KW,
    FINAL_KW,
    FN_KW,
    FOR_KW,
    FUNCTION_KW,
    IF_KW,
    IMPL_KW,
    IMPORT_KW,
    IN_KW,
    INTERFACE_KW,
    LET_KW,
    LOOP_KW,
    MACRO_KW,
    MATCH_KW,
    METHOD_KW,
    MODULE_KW,
    MOVE_KW,
    MUT_KW,
    OVERRIDE_KW,
    PACKAGE_KW,
    PRIV_KW,
    PUB_KW,
    REF_KW,
    RETURN_KW,
    RULE_KW,
    SELF_KW,
    STATIC_KW,
    STRUCT_KW,
    SUPER_KW,
    TRAIT_KW,
    TRUE_KW,
    TYPE_KW,
    TYPEDEF_KW,
    TYPEOF_KW,
    UNSAFE_KW,
    UNSIZED_KW,
    USE_KW,
    VIRTUAL_KW,
    WHERE_KW,
    WHILE_KW,
    YIELD_KW,
    ASM_KW,
    ASYNC_KW,
    ATT_SYNTAX_KW,
    AUTO_KW,
    AWAIT_KW,
    BUILTIN_KW,
    CLOBBER_ABI_KW,
    DEFAULT_KW,
    DYN_KW,
    FORMAT_ARGS_KW,
    GEN_KW,
    INLATEOUT_KW,
    INOUT_KW,
    LABEL_KW,
    LATEOUT_KW,
    MACRO_RULES_KW,
    MAY_UNWIND_KW,
    NOMEM_KW,
    NORETURN_KW,
    NOSTACK_KW,
    OFFSET_OF_KW,
    OPTIONS_KW,
    OUT_KW,
    PRESERVES_FLAGS_KW,
    PURE_KW,
    RAW_KW,
    READONLY_KW,
    SYM_KW,
    TRY_KW,
    UNION_KW,
    YEET_KW,
    BYTE,
    BYTE_STRING,
    CHAR,
    C_STRING,
    FLOAT_NUMBER,
    INT_NUMBER,
    RAW_BYTE_STRING,
    RAW_C_STRING,
    RAW_STRING,
    STRING,
    COMMENT,
    ERROR,
    IDENT,
    LIFETIME_IDENT,
    NEWLINE,
    SHEBANG,
    WHITESPACE,
    ABI,
    ADT,
    ARG_LIST,
    ARRAY_EXPR,
    ARRAY_TYPE,
    ASM_CLOBBER_ABI,
    ASM_CONST,
    ASM_DIR_SPEC,
    ASM_EXPR,
    ASM_LABEL,
    ASM_OPERAND,
    ASM_OPERAND_EXPR,
    ASM_OPERAND_NAMED,
    ASM_OPTION,
    ASM_OPTIONS,
    ASM_PIECE,
    ASM_REG_OPERAND,
    ASM_REG_SPEC,
    ASM_SYM,
    ASSOC_ITEM,
    ASSOC_ITEM_LIST,
    ASSOC_TYPE_ARG,
    ATTR,
    ATTR_BSV,
    ATTR_META_BSV,
    AWAIT_EXPR,
    BECOME_EXPR,
    BIN_EXPR,
    BLOCK_EXPR,
    BOX_PAT,
    BREAK_EXPR,
    CALL_EXPR,
    CAST_EXPR,
    CLOSURE_BINDER,
    CLOSURE_EXPR,
    CONST,
    CONST_ARG,
    CONST_BLOCK_PAT,
    CONST_PARAM,
    CONTINUE_EXPR,
    DYN_TRAIT_TYPE,
    ENUM,
    EXPR,
    EXPR_BSV,
    EXPR_STMT,
    EXTERN_BLOCK,
    EXTERN_CRATE,
    EXTERN_ITEM,
    EXTERN_ITEM_LIST,
    FIELD_EXPR,
    FIELD_LIST,
    FN,
    FN_PTR_TYPE,
    FORMAT_ARGS_ARG,
    FORMAT_ARGS_EXPR,
    FOR_EXPR,
    FOR_TYPE,
    FUNCTION,
    FUNCTION_SIGNATURE,
    GENERIC_ARG,
    GENERIC_ARG_LIST,
    GENERIC_PARAM,
    GENERIC_PARAM_LIST,
    GUARD,
    IDENT_PAT,
    IF_EXPR,
    IMPL,
    IMPL_TRAIT_TYPE,
    IMPORT,
    INDEX_EXPR,
    INFER_TYPE,
    INTERFACE_BSV,
    INTERFACE_STMT,
    ITEM,
    ITEM_LIST,
    LABEL,
    LET_ELSE,
    LET_EXPR,
    LET_STMT,
    LIFETIME,
    LIFETIME_ARG,
    LIFETIME_PARAM,
    LITERAL,
    LITERAL_PAT,
    LOOP_EXPR,
    MACRO_CALL,
    MACRO_DEF,
    MACRO_EXPR,
    MACRO_ITEMS,
    MACRO_PAT,
    MACRO_RULES,
    MACRO_STMTS,
    MACRO_TYPE,
    MATCH_ARM,
    MATCH_ARM_LIST,
    MATCH_EXPR,
    MATCH_GUARD,
    META,
    METHOD_CALL_EXPR,
    METHOD_DECL,
    METHOD_IMPL,
    METHOD_SIGNATURE,
    MODULE,
    MODULE_BSV,
    MODULE_CALL,
    MODULE_INST,
    MODULE_STMT,
    NAME,
    NAME_REF,
    NEVER_TYPE,
    OFFSET_OF_EXPR,
    OR_PAT,
    PACKAGE,
    PARAM,
    PARAM_BSV,
    PARAM_LIST,
    PARAM_LIST_BSV,
    PAREN_EXPR,
    PAREN_PAT,
    PAREN_TYPE,
    PAT,
    PATH,
    PATH_EXPR,
    PATH_PAT,
    PATH_SEGMENT,
    PATH_TYPE,
    PREFIX_EXPR,
    PTR_TYPE,
    RANGE_EXPR,
    RANGE_PAT,
    RECORD_EXPR,
    RECORD_EXPR_FIELD,
    RECORD_EXPR_FIELD_LIST,
    RECORD_FIELD,
    RECORD_FIELD_LIST,
    RECORD_PAT,
    RECORD_PAT_FIELD,
    RECORD_PAT_FIELD_LIST,
    REF_EXPR,
    REF_PAT,
    REF_TYPE,
    RENAME,
    REST_PAT,
    RETURN_EXPR,
    RETURN_TYPE_SYNTAX,
    RET_TYPE,
    RULE,
    SELF_PARAM,
    SLICE_PAT,
    SLICE_TYPE,
    SOURCE_FILE,
    STATIC,
    STMT,
    STMT_LIST,
    STMT_LIST_BSV,
    STRUCT,
    TOKEN_TREE,
    TRAIT,
    TRAIT_ALIAS,
    TRY_EXPR,
    TUPLE_EXPR,
    TUPLE_FIELD,
    TUPLE_FIELD_LIST,
    TUPLE_PAT,
    TUPLE_STRUCT_PAT,
    TUPLE_TYPE,
    TYPE,
    TYPEDEF_BSV,
    TYPED_VAR,
    TYPE_ALIAS,
    TYPE_ARG,
    TYPE_BOUND,
    TYPE_BOUND_LIST,
    TYPE_BSV,
    TYPE_PARAM,
    TYPE_SYNONYM,
    UNDERSCORE_EXPR,
    UNION,
    USE,
    USE_TREE,
    USE_TREE_LIST,
    VARIANT,
    VARIANT_LIST,
    VISIBILITY,
    WHERE_CLAUSE,
    WHERE_PRED,
    WHILE_EXPR,
    WILDCARD_PAT,
    YEET_EXPR,
    YIELD_EXPR,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    #[doc = r" Checks whether this syntax kind is a strict keyword for the given edition."]
    #[doc = r" Strict keywords are identifiers that are always considered keywords."]
    pub fn is_strict_keyword(self, edition: Edition) -> bool {
        matches!(
            self,
            ACTION_KW
                | ACTION_VALUE_KW
                | SELF_TYPE_KW
                | ABSTRACT_KW
                | AS_KW
                | BECOME_KW
                | BEGIN_KW
                | BOX_KW
                | BREAK_KW
                | CONST_KW
                | CONTINUE_KW
                | CRATE_KW
                | DO_KW
                | ELSE_KW
                | END_KW
                | ENDFUNCTION_KW
                | ENDINTERFACE_KW
                | ENDMETHOD_KW
                | ENDMODULE_KW
                | ENDRULE_KW
                | ENUM_KW
                | EXTERN_KW
                | FALSE_KW
                | FINAL_KW
                | FN_KW
                | FOR_KW
                | FUNCTION_KW
                | IF_KW
                | IMPL_KW
                | IMPORT_KW
                | IN_KW
                | INTERFACE_KW
                | LET_KW
                | LOOP_KW
                | MACRO_KW
                | MATCH_KW
                | METHOD_KW
                | MODULE_KW
                | MOVE_KW
                | MUT_KW
                | OVERRIDE_KW
                | PACKAGE_KW
                | PRIV_KW
                | PUB_KW
                | REF_KW
                | RETURN_KW
                | RULE_KW
                | SELF_KW
                | STATIC_KW
                | STRUCT_KW
                | SUPER_KW
                | TRAIT_KW
                | TRUE_KW
                | TYPE_KW
                | TYPEDEF_KW
                | TYPEOF_KW
                | UNSAFE_KW
                | UNSIZED_KW
                | USE_KW
                | VIRTUAL_KW
                | WHERE_KW
                | WHILE_KW
                | YIELD_KW
        ) || match self {
            ASYNC_KW if Edition::Edition2018 <= edition => true,
            AWAIT_KW if Edition::Edition2018 <= edition => true,
            DYN_KW if Edition::Edition2018 <= edition => true,
            GEN_KW if Edition::Edition2024 <= edition => true,
            TRY_KW if Edition::Edition2018 <= edition => true,
            _ => false,
        }
    }
    #[doc = r" Checks whether this syntax kind is a weak keyword for the given edition."]
    #[doc = r" Weak keywords are identifiers that are considered keywords only in certain contexts."]
    pub fn is_contextual_keyword(self, edition: Edition) -> bool {
        match self {
            ASM_KW => true,
            ATT_SYNTAX_KW => true,
            AUTO_KW => true,
            BUILTIN_KW => true,
            CLOBBER_ABI_KW => true,
            DEFAULT_KW => true,
            DYN_KW if edition < Edition::Edition2018 => true,
            FORMAT_ARGS_KW => true,
            INLATEOUT_KW => true,
            INOUT_KW => true,
            LABEL_KW => true,
            LATEOUT_KW => true,
            MACRO_RULES_KW => true,
            MAY_UNWIND_KW => true,
            NOMEM_KW => true,
            NORETURN_KW => true,
            NOSTACK_KW => true,
            OFFSET_OF_KW => true,
            OPTIONS_KW => true,
            OUT_KW => true,
            PRESERVES_FLAGS_KW => true,
            PURE_KW => true,
            RAW_KW => true,
            READONLY_KW => true,
            SYM_KW => true,
            UNION_KW => true,
            YEET_KW => true,
            _ => false,
        }
    }
    #[doc = r" Checks whether this syntax kind is a strict or weak keyword for the given edition."]
    pub fn is_keyword(self, edition: Edition) -> bool {
        matches!(
            self,
            ACTION_KW
                | ACTION_VALUE_KW
                | SELF_TYPE_KW
                | ABSTRACT_KW
                | AS_KW
                | BECOME_KW
                | BEGIN_KW
                | BOX_KW
                | BREAK_KW
                | CONST_KW
                | CONTINUE_KW
                | CRATE_KW
                | DO_KW
                | ELSE_KW
                | END_KW
                | ENDFUNCTION_KW
                | ENDINTERFACE_KW
                | ENDMETHOD_KW
                | ENDMODULE_KW
                | ENDRULE_KW
                | ENUM_KW
                | EXTERN_KW
                | FALSE_KW
                | FINAL_KW
                | FN_KW
                | FOR_KW
                | FUNCTION_KW
                | IF_KW
                | IMPL_KW
                | IMPORT_KW
                | IN_KW
                | INTERFACE_KW
                | LET_KW
                | LOOP_KW
                | MACRO_KW
                | MATCH_KW
                | METHOD_KW
                | MODULE_KW
                | MOVE_KW
                | MUT_KW
                | OVERRIDE_KW
                | PACKAGE_KW
                | PRIV_KW
                | PUB_KW
                | REF_KW
                | RETURN_KW
                | RULE_KW
                | SELF_KW
                | STATIC_KW
                | STRUCT_KW
                | SUPER_KW
                | TRAIT_KW
                | TRUE_KW
                | TYPE_KW
                | TYPEDEF_KW
                | TYPEOF_KW
                | UNSAFE_KW
                | UNSIZED_KW
                | USE_KW
                | VIRTUAL_KW
                | WHERE_KW
                | WHILE_KW
                | YIELD_KW
        ) || match self {
            ASYNC_KW if Edition::Edition2018 <= edition => true,
            AWAIT_KW if Edition::Edition2018 <= edition => true,
            DYN_KW if Edition::Edition2018 <= edition => true,
            GEN_KW if Edition::Edition2024 <= edition => true,
            TRY_KW if Edition::Edition2018 <= edition => true,
            ASM_KW => true,
            ATT_SYNTAX_KW => true,
            AUTO_KW => true,
            BUILTIN_KW => true,
            CLOBBER_ABI_KW => true,
            DEFAULT_KW => true,
            DYN_KW if edition < Edition::Edition2018 => true,
            FORMAT_ARGS_KW => true,
            INLATEOUT_KW => true,
            INOUT_KW => true,
            LABEL_KW => true,
            LATEOUT_KW => true,
            MACRO_RULES_KW => true,
            MAY_UNWIND_KW => true,
            NOMEM_KW => true,
            NORETURN_KW => true,
            NOSTACK_KW => true,
            OFFSET_OF_KW => true,
            OPTIONS_KW => true,
            OUT_KW => true,
            PRESERVES_FLAGS_KW => true,
            PURE_KW => true,
            RAW_KW => true,
            READONLY_KW => true,
            SYM_KW => true,
            UNION_KW => true,
            YEET_KW => true,
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool {
        matches!(
            self,
            DOLLAR
                | SEMICOLON
                | COMMA
                | L_PAREN
                | R_PAREN
                | L_CURLY
                | R_CURLY
                | L_BRACK
                | R_BRACK
                | L_ANGLE
                | R_ANGLE
                | AT
                | POUND
                | TILDE
                | QUESTION
                | AMP
                | PIPE
                | PLUS
                | STAR
                | SLASH
                | CARET
                | PERCENT
                | UNDERSCORE
                | DOT
                | DOT2
                | DOT3
                | DOT2EQ
                | COLON
                | COLON2
                | EQ
                | EQ2
                | FAT_ARROW
                | BANG
                | NEQ
                | MINUS
                | R_THIN_ARROW
                | L_THIN_ARROW
                | LTEQ
                | GTEQ
                | PLUSEQ
                | MINUSEQ
                | PIPEEQ
                | AMPEQ
                | CARETEQ
                | SLASHEQ
                | STAREQ
                | PERCENTEQ
                | AMP2
                | PIPE2
                | SHL
                | SHR
                | SHLEQ
                | SHREQ
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            BYTE | BYTE_STRING
                | CHAR
                | C_STRING
                | FLOAT_NUMBER
                | INT_NUMBER
                | RAW_BYTE_STRING
                | RAW_C_STRING
                | RAW_STRING
                | STRING
        )
    }
    pub fn from_keyword(ident: &str, edition: Edition) -> Option<SyntaxKind> {
        let kw = match ident {
            "Action" => ACTION_KW,
            "ActionValue" => ACTION_VALUE_KW,
            "Self" => SELF_TYPE_KW,
            "abstract" => ABSTRACT_KW,
            "as" => AS_KW,
            "become" => BECOME_KW,
            "begin" => BEGIN_KW,
            "box" => BOX_KW,
            "break" => BREAK_KW,
            "const" => CONST_KW,
            "continue" => CONTINUE_KW,
            "crate" => CRATE_KW,
            "do" => DO_KW,
            "else" => ELSE_KW,
            "end" => END_KW,
            "endfunction" => ENDFUNCTION_KW,
            "endinterface" => ENDINTERFACE_KW,
            "endmethod" => ENDMETHOD_KW,
            "endmodule" => ENDMODULE_KW,
            "endrule" => ENDRULE_KW,
            "enum" => ENUM_KW,
            "extern" => EXTERN_KW,
            "false" => FALSE_KW,
            "final" => FINAL_KW,
            "fn" => FN_KW,
            "for" => FOR_KW,
            "function" => FUNCTION_KW,
            "if" => IF_KW,
            "impl" => IMPL_KW,
            "import" => IMPORT_KW,
            "in" => IN_KW,
            "interface" => INTERFACE_KW,
            "let" => LET_KW,
            "loop" => LOOP_KW,
            "macro" => MACRO_KW,
            "match" => MATCH_KW,
            "method" => METHOD_KW,
            "module" => MODULE_KW,
            "move" => MOVE_KW,
            "mut" => MUT_KW,
            "override" => OVERRIDE_KW,
            "package" => PACKAGE_KW,
            "priv" => PRIV_KW,
            "pub" => PUB_KW,
            "ref" => REF_KW,
            "return" => RETURN_KW,
            "rule" => RULE_KW,
            "self" => SELF_KW,
            "static" => STATIC_KW,
            "struct" => STRUCT_KW,
            "super" => SUPER_KW,
            "trait" => TRAIT_KW,
            "true" => TRUE_KW,
            "type" => TYPE_KW,
            "typedef" => TYPEDEF_KW,
            "typeof" => TYPEOF_KW,
            "unsafe" => UNSAFE_KW,
            "unsized" => UNSIZED_KW,
            "use" => USE_KW,
            "virtual" => VIRTUAL_KW,
            "where" => WHERE_KW,
            "while" => WHILE_KW,
            "yield" => YIELD_KW,
            "async" if Edition::Edition2018 <= edition => ASYNC_KW,
            "await" if Edition::Edition2018 <= edition => AWAIT_KW,
            "dyn" if Edition::Edition2018 <= edition => DYN_KW,
            "gen" if Edition::Edition2024 <= edition => GEN_KW,
            "try" if Edition::Edition2018 <= edition => TRY_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_contextual_keyword(ident: &str, edition: Edition) -> Option<SyntaxKind> {
        let kw = match ident {
            "asm" => ASM_KW,
            "att_syntax" => ATT_SYNTAX_KW,
            "auto" => AUTO_KW,
            "builtin" => BUILTIN_KW,
            "clobber_abi" => CLOBBER_ABI_KW,
            "default" => DEFAULT_KW,
            "dyn" if edition < Edition::Edition2018 => DYN_KW,
            "format_args" => FORMAT_ARGS_KW,
            "inlateout" => INLATEOUT_KW,
            "inout" => INOUT_KW,
            "label" => LABEL_KW,
            "lateout" => LATEOUT_KW,
            "macro_rules" => MACRO_RULES_KW,
            "may_unwind" => MAY_UNWIND_KW,
            "nomem" => NOMEM_KW,
            "noreturn" => NORETURN_KW,
            "nostack" => NOSTACK_KW,
            "offset_of" => OFFSET_OF_KW,
            "options" => OPTIONS_KW,
            "out" => OUT_KW,
            "preserves_flags" => PRESERVES_FLAGS_KW,
            "pure" => PURE_KW,
            "raw" => RAW_KW,
            "readonly" => READONLY_KW,
            "sym" => SYM_KW,
            "union" => UNION_KW,
            "yeet" => YEET_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '$' => DOLLAR,
            ';' => SEMICOLON,
            ',' => COMMA,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            '@' => AT,
            '#' => POUND,
            '~' => TILDE,
            '?' => QUESTION,
            '&' => AMP,
            '|' => PIPE,
            '+' => PLUS,
            '*' => STAR,
            '/' => SLASH,
            '^' => CARET,
            '%' => PERCENT,
            '_' => UNDERSCORE,
            '.' => DOT,
            ':' => COLON,
            '=' => EQ,
            '!' => BANG,
            '-' => MINUS,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { [$] => { $ crate :: SyntaxKind :: DOLLAR } ; [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: SyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: SyntaxKind :: R_CURLY } ; ['['] => { $ crate :: SyntaxKind :: L_BRACK } ; [']'] => { $ crate :: SyntaxKind :: R_BRACK } ; [<] => { $ crate :: SyntaxKind :: L_ANGLE } ; [>] => { $ crate :: SyntaxKind :: R_ANGLE } ; [@] => { $ crate :: SyntaxKind :: AT } ; [#] => { $ crate :: SyntaxKind :: POUND } ; [~] => { $ crate :: SyntaxKind :: TILDE } ; [?] => { $ crate :: SyntaxKind :: QUESTION } ; [&] => { $ crate :: SyntaxKind :: AMP } ; [|] => { $ crate :: SyntaxKind :: PIPE } ; [+] => { $ crate :: SyntaxKind :: PLUS } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [^] => { $ crate :: SyntaxKind :: CARET } ; [%] => { $ crate :: SyntaxKind :: PERCENT } ; [_] => { $ crate :: SyntaxKind :: UNDERSCORE } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [..] => { $ crate :: SyntaxKind :: DOT2 } ; [...] => { $ crate :: SyntaxKind :: DOT3 } ; [..=] => { $ crate :: SyntaxKind :: DOT2EQ } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [::] => { $ crate :: SyntaxKind :: COLON2 } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [==] => { $ crate :: SyntaxKind :: EQ2 } ; [=>] => { $ crate :: SyntaxKind :: FAT_ARROW } ; [!] => { $ crate :: SyntaxKind :: BANG } ; [!=] => { $ crate :: SyntaxKind :: NEQ } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [->] => { $ crate :: SyntaxKind :: R_THIN_ARROW } ; [<-] => { $ crate :: SyntaxKind :: L_THIN_ARROW } ; [<=] => { $ crate :: SyntaxKind :: LTEQ } ; [>=] => { $ crate :: SyntaxKind :: GTEQ } ; [+=] => { $ crate :: SyntaxKind :: PLUSEQ } ; [-=] => { $ crate :: SyntaxKind :: MINUSEQ } ; [|=] => { $ crate :: SyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: SyntaxKind :: AMPEQ } ; [^=] => { $ crate :: SyntaxKind :: CARETEQ } ; [/=] => { $ crate :: SyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: SyntaxKind :: STAREQ } ; [%=] => { $ crate :: SyntaxKind :: PERCENTEQ } ; [&&] => { $ crate :: SyntaxKind :: AMP2 } ; [||] => { $ crate :: SyntaxKind :: PIPE2 } ; [<<] => { $ crate :: SyntaxKind :: SHL } ; [>>] => { $ crate :: SyntaxKind :: SHR } ; [<<=] => { $ crate :: SyntaxKind :: SHLEQ } ; [>>=] => { $ crate :: SyntaxKind :: SHREQ } ; [Action] => { $ crate :: SyntaxKind :: ACTION_KW } ; [ActionValue] => { $ crate :: SyntaxKind :: ACTION_VALUE_KW } ; [Self] => { $ crate :: SyntaxKind :: SELF_TYPE_KW } ; [abstract] => { $ crate :: SyntaxKind :: ABSTRACT_KW } ; [as] => { $ crate :: SyntaxKind :: AS_KW } ; [become] => { $ crate :: SyntaxKind :: BECOME_KW } ; [begin] => { $ crate :: SyntaxKind :: BEGIN_KW } ; [box] => { $ crate :: SyntaxKind :: BOX_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [const] => { $ crate :: SyntaxKind :: CONST_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [crate] => { $ crate :: SyntaxKind :: CRATE_KW } ; [do] => { $ crate :: SyntaxKind :: DO_KW } ; [else] => { $ crate :: SyntaxKind :: ELSE_KW } ; [end] => { $ crate :: SyntaxKind :: END_KW } ; [endfunction] => { $ crate :: SyntaxKind :: ENDFUNCTION_KW } ; [endinterface] => { $ crate :: SyntaxKind :: ENDINTERFACE_KW } ; [endmethod] => { $ crate :: SyntaxKind :: ENDMETHOD_KW } ; [endmodule] => { $ crate :: SyntaxKind :: ENDMODULE_KW } ; [endrule] => { $ crate :: SyntaxKind :: ENDRULE_KW } ; [enum] => { $ crate :: SyntaxKind :: ENUM_KW } ; [extern] => { $ crate :: SyntaxKind :: EXTERN_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [final] => { $ crate :: SyntaxKind :: FINAL_KW } ; [fn] => { $ crate :: SyntaxKind :: FN_KW } ; [for] => { $ crate :: SyntaxKind :: FOR_KW } ; [function] => { $ crate :: SyntaxKind :: FUNCTION_KW } ; [if] => { $ crate :: SyntaxKind :: IF_KW } ; [impl] => { $ crate :: SyntaxKind :: IMPL_KW } ; [import] => { $ crate :: SyntaxKind :: IMPORT_KW } ; [in] => { $ crate :: SyntaxKind :: IN_KW } ; [interface] => { $ crate :: SyntaxKind :: INTERFACE_KW } ; [let] => { $ crate :: SyntaxKind :: LET_KW } ; [loop] => { $ crate :: SyntaxKind :: LOOP_KW } ; [macro] => { $ crate :: SyntaxKind :: MACRO_KW } ; [match] => { $ crate :: SyntaxKind :: MATCH_KW } ; [method] => { $ crate :: SyntaxKind :: METHOD_KW } ; [module] => { $ crate :: SyntaxKind :: MODULE_KW } ; [move] => { $ crate :: SyntaxKind :: MOVE_KW } ; [mut] => { $ crate :: SyntaxKind :: MUT_KW } ; [override] => { $ crate :: SyntaxKind :: OVERRIDE_KW } ; [package] => { $ crate :: SyntaxKind :: PACKAGE_KW } ; [priv] => { $ crate :: SyntaxKind :: PRIV_KW } ; [pub] => { $ crate :: SyntaxKind :: PUB_KW } ; [ref] => { $ crate :: SyntaxKind :: REF_KW } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [rule] => { $ crate :: SyntaxKind :: RULE_KW } ; [self] => { $ crate :: SyntaxKind :: SELF_KW } ; [static] => { $ crate :: SyntaxKind :: STATIC_KW } ; [struct] => { $ crate :: SyntaxKind :: STRUCT_KW } ; [super] => { $ crate :: SyntaxKind :: SUPER_KW } ; [trait] => { $ crate :: SyntaxKind :: TRAIT_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [type] => { $ crate :: SyntaxKind :: TYPE_KW } ; [typedef] => { $ crate :: SyntaxKind :: TYPEDEF_KW } ; [typeof] => { $ crate :: SyntaxKind :: TYPEOF_KW } ; [unsafe] => { $ crate :: SyntaxKind :: UNSAFE_KW } ; [unsized] => { $ crate :: SyntaxKind :: UNSIZED_KW } ; [use] => { $ crate :: SyntaxKind :: USE_KW } ; [virtual] => { $ crate :: SyntaxKind :: VIRTUAL_KW } ; [where] => { $ crate :: SyntaxKind :: WHERE_KW } ; [while] => { $ crate :: SyntaxKind :: WHILE_KW } ; [yield] => { $ crate :: SyntaxKind :: YIELD_KW } ; [asm] => { $ crate :: SyntaxKind :: ASM_KW } ; [att_syntax] => { $ crate :: SyntaxKind :: ATT_SYNTAX_KW } ; [auto] => { $ crate :: SyntaxKind :: AUTO_KW } ; [builtin] => { $ crate :: SyntaxKind :: BUILTIN_KW } ; [clobber_abi] => { $ crate :: SyntaxKind :: CLOBBER_ABI_KW } ; [default] => { $ crate :: SyntaxKind :: DEFAULT_KW } ; [dyn] => { $ crate :: SyntaxKind :: DYN_KW } ; [format_args] => { $ crate :: SyntaxKind :: FORMAT_ARGS_KW } ; [inlateout] => { $ crate :: SyntaxKind :: INLATEOUT_KW } ; [inout] => { $ crate :: SyntaxKind :: INOUT_KW } ; [label] => { $ crate :: SyntaxKind :: LABEL_KW } ; [lateout] => { $ crate :: SyntaxKind :: LATEOUT_KW } ; [macro_rules] => { $ crate :: SyntaxKind :: MACRO_RULES_KW } ; [may_unwind] => { $ crate :: SyntaxKind :: MAY_UNWIND_KW } ; [nomem] => { $ crate :: SyntaxKind :: NOMEM_KW } ; [noreturn] => { $ crate :: SyntaxKind :: NORETURN_KW } ; [nostack] => { $ crate :: SyntaxKind :: NOSTACK_KW } ; [offset_of] => { $ crate :: SyntaxKind :: OFFSET_OF_KW } ; [options] => { $ crate :: SyntaxKind :: OPTIONS_KW } ; [out] => { $ crate :: SyntaxKind :: OUT_KW } ; [preserves_flags] => { $ crate :: SyntaxKind :: PRESERVES_FLAGS_KW } ; [pure] => { $ crate :: SyntaxKind :: PURE_KW } ; [raw] => { $ crate :: SyntaxKind :: RAW_KW } ; [readonly] => { $ crate :: SyntaxKind :: READONLY_KW } ; [sym] => { $ crate :: SyntaxKind :: SYM_KW } ; [union] => { $ crate :: SyntaxKind :: UNION_KW } ; [yeet] => { $ crate :: SyntaxKind :: YEET_KW } ; [async] => { $ crate :: SyntaxKind :: ASYNC_KW } ; [await] => { $ crate :: SyntaxKind :: AWAIT_KW } ; [dyn] => { $ crate :: SyntaxKind :: DYN_KW } ; [gen] => { $ crate :: SyntaxKind :: GEN_KW } ; [try] => { $ crate :: SyntaxKind :: TRY_KW } ; [lifetime_ident] => { $ crate :: SyntaxKind :: LIFETIME_IDENT } ; [int_number] => { $ crate :: SyntaxKind :: INT_NUMBER } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [string] => { $ crate :: SyntaxKind :: STRING } ; [shebang] => { $ crate :: SyntaxKind :: SHEBANG } ; }

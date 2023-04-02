Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: UseExprRuleIdx(
                    0,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SuperModule {
                        current_module_path: `mnist_classifier::digits::five`,
                        super_module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist::MnistLabel`, `Enum`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist::MnistLabel`, `Enum`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)
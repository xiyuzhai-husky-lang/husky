Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i8`, `Foreign`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i16`, `Foreign`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i32`, `Foreign`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i64`, `Foreign`),
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl,
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl,
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl,
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl,
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)
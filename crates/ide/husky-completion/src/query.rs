use husky_text::FilePosition;
use lsp_types::CompletionResponse;

pub trait HuskyCompletionQuery {
    fn completion(
        &self,
        _fpos: FilePosition,
        _completion_trigger_character: Option<String>,
    ) -> Option<CompletionResponse> {
        todo!()
        // let term_sheet = self.term_sheet(fpos.file()).expect("todo");
        // let expr = term_sheet
        //     .ast_text()
        //     .find_last_expr_before(fpos.pos())
        //     .expect("todo");
        // match expr.variant {
        //     RawExprVariant::Opn {
        //         ref opn_variant,
        //         ref opds,
        //     } => match opn_variant {
        //         RawOpnVariant::Field(_) => {
        //             let ty = term_sheet.expr_ty_result(opds.start()).ok()?;
        //             let ty_decl = self.ty_decl(ty.intrinsic()).ok()?;
        //             Some(field_completion_response(&ty_decl))
        //         }
        //         _ => None,
        //     },
        //     _ => None,
        // }
    }
}

// fn field_completion_response(ty_decl: &TyDecl) -> CompletionResponse {
//     CompletionResponse::Array(
//         ty_decl
//             .members
//             .iter()
//             .filter_map(|member| completion_item_from_ty_member(member))
//             .collect(),
//     )
// }

// fn completion_item_from_ty_member(member: &MemberDecl) -> Option<CompletionItem> {
//     let label = member.ident().to_string();
//     let kind: CompletionItemKind = match member {
//         MemberDecl::TypeField(field) => CompletionItemKind::FIELD,
//         MemberDecl::TypeMethod(_) => CompletionItemKind::METHOD,
//         MemberDecl::TraitMethodImpl { .. } => CompletionItemKind::METHOD,
//         MemberDecl::AssociatedType
//         | MemberDecl::AssociatedCall
//         | MemberDecl::TypeAssociatedCall(_)
//         | MemberDecl::TraitAssociatedTypeImpl { .. }
//         | MemberDecl::TraitAssociatedConstSizeImpl { .. } => return None,
//     };
//     Some(CompletionItem {
//         label,
//         label_details: None,
//         kind: Some(kind),
//         detail: None,
//         documentation: None,
//         deprecated: None,
//         preselect: None,
//         sort_text: None,
//         filter_text: None,
//         insert_text: None,
//         insert_text_format: None,
//         insert_text_mode: None,
//         text_edit: None,
//         additional_text_edits: None,
//         command: None,
//         commit_characters: None,
//         data: None,
//         tags: None,
//     })
// }

// fn mimic_completion_response() -> CompletionResponse {
//     CompletionResponse::Array(vec![
//         CompletionItem {
//             label: "haha".to_string(),
//             label_details: Some(CompletionItemLabelDetails {
//                 detail: Some("haha has no details".to_string()),
//                 description: Some(format!("description haha")),
//             }),
//             kind: None,
//             detail: None,
//             documentation: None,
//             deprecated: None,
//             preselect: None,
//             sort_text: None,
//             filter_text: None,
//             insert_text: None,
//             insert_text_format: None,
//             insert_text_mode: None,
//             text_edit: None,
//             additional_text_edits: None,
//             command: None,
//             commit_characters: None,
//             data: None,
//             tags: None,
//         },
//         CompletionItem {
//             label: "haha_field".to_string(),
//             label_details: Some(CompletionItemLabelDetails {
//                 detail: Some("haha has no details".to_string()),
//                 description: Some(format!("description haha")),
//             }),
//             kind: Some(CompletionItemKind::FIELD),
//             detail: None,
//             documentation: None,
//             deprecated: None,
//             preselect: None,
//             sort_text: None,
//             filter_text: None,
//             insert_text: None,
//             insert_text_format: None,
//             insert_text_mode: None,
//             text_edit: None,
//             additional_text_edits: None,
//             command: None,
//             commit_characters: None,
//             data: None,
//             tags: None,
//         },
//         CompletionItem {
//             label: "haha_value".to_string(),
//             label_details: Some(CompletionItemLabelDetails {
//                 detail: Some("haha has no details".to_string()),
//                 description: Some(format!("description haha")),
//             }),
//             kind: Some(CompletionItemKind::VALUE),
//             detail: None,
//             documentation: None,
//             deprecated: None,
//             preselect: None,
//             sort_text: None,
//             filter_text: None,
//             insert_text: None,
//             insert_text_format: None,
//             insert_text_mode: None,
//             text_edit: None,
//             additional_text_edits: None,
//             command: None,
//             commit_characters: None,
//             data: None,
//             tags: None,
//         },
//         CompletionItem {
//             label: "haha_function".to_string(),
//             label_details: Some(CompletionItemLabelDetails {
//                 detail: Some("haha has no details".to_string()),
//                 description: Some(format!("description haha")),
//             }),
//             kind: Some(CompletionItemKind::FUNCTION),
//             detail: None,
//             documentation: None,
//             deprecated: Some(true),
//             preselect: None,
//             sort_text: None,
//             filter_text: None,
//             insert_text: None,
//             insert_text_format: None,
//             insert_text_mode: None,
//             text_edit: None,
//             additional_text_edits: None,
//             command: None,
//             commit_characters: None,
//             data: None,
//             tags: None,
//         },
//     ])
// }

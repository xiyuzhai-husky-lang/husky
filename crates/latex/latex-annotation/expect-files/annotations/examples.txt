[
    LxAnnotationsExample {
        root_mode: Math,
        input: "",
        annotations: LxAnnotations {
            token_annotations: [],
            space_annotations: [],
        },
    },
    LxAnnotationsExample {
        root_mode: Math,
        input: "xy",
        annotations: LxAnnotations {
            token_annotations: [
                LxAnnotationEntry {
                    start: 0,
                    end: 1,
                    annotation: Variable(
                        Usage,
                    ),
                },
                LxAnnotationEntry {
                    start: 1,
                    end: 2,
                    annotation: Variable(
                        Usage,
                    ),
                },
            ],
            space_annotations: [
                LxAnnotationEntry {
                    start: 1,
                    end: 2,
                    annotation: Apply(
                        ScalarMul,
                    ),
                },
            ],
        },
    },
    LxAnnotationsExample {
        root_mode: Math,
        input: "dx",
        annotations: LxAnnotations {
            token_annotations: [
                LxAnnotationEntry {
                    start: 0,
                    end: 1,
                    annotation: Differential,
                },
                LxAnnotationEntry {
                    start: 1,
                    end: 2,
                    annotation: Variable(
                        SingleVariableIntegralVariableDecl,
                    ),
                },
            ],
            space_annotations: [],
        },
    },
]
#[cfg(test)]
use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_typing::{
    FieldHavingType, MutableFieldHavingType,
    base::{
        BaseType, instance::BaseTypeInstance, kind::BaseTypeKind, structs::BaseStructContainer,
    },
    func::{DeclBlockAffectedType, MutableDeclBlockAffectedType, TypedFunction},
    params::TypeParameterHaving,
    tree::Type,
};

#[test]
fn test_type_parameter_resolver_field() {
    let source = PosDiagnosticSource::new(Default::default(), Default::default());

    let boolean_type = Type::Base(BaseTypeInstance::new(
        BaseType::new(BaseTypeKind::Boolean),
        vec![],
        vec![],
    ));

    let mut struct_type = BaseType::new(BaseTypeKind::Struct(BaseStructContainer::new(
        "test".into(),
    )));

    struct_type
        .append_type_parameter("K".into(), &source)
        .unwrap_cleanly();

    struct_type
        .add_field(
            "test_field".into(),
            struct_type.get_type_parameter_type("K".into()),
            &source,
        )
        .unwrap_cleanly();

    // Tests on the BaseType layer. Here, type parameters are still there

    assert!(struct_type.has_field("test_field".into()));
    assert_eq!(
        struct_type.get_field_type("test_field".into()),
        Type::TypeParameter {
            name: "K".into(),
            param_ind: 0
        }
    );

    // Tests on the BaseTypeInstance

    let struct_instance =
        BaseTypeInstance::new(struct_type.clone(), vec![], vec![boolean_type.clone()]);

    assert!(struct_instance.has_field("test_field".into()));
    assert_eq!(
        struct_instance.get_field_type("test_field".into()),
        boolean_type
    );
}

#[test]
fn test_type_parameter_resolver_functions() {
    let source = PosDiagnosticSource::new(Default::default(), Default::default());

    let boolean_type = Type::Base(BaseTypeInstance::new(
        BaseType::new(BaseTypeKind::Boolean),
        vec![],
        vec![],
    ));

    let int_type = Type::Base(BaseTypeInstance::new(
        BaseType::new(BaseTypeKind::Integer { signed: true }),
        vec![64],
        vec![],
    ));

    let mut struct_type = BaseType::new(BaseTypeKind::Struct(BaseStructContainer::new(
        "test_struct".into(),
    )));

    struct_type
        .append_type_parameter("K".into(), &source)
        .unwrap_cleanly();

    struct_type
        .append_type_parameter("V".into(), &source)
        .unwrap_cleanly();

    struct_type
        .add_function(
            "test_function".into(),
            TypedFunction::new(
                "test_function".into(),
                vec![
                    struct_type.get_type_parameter_type("K".into()),
                    struct_type.get_type_parameter_type("V".into()),
                ],
                struct_type.get_type_parameter_type("V".into()),
            ),
            &source,
        )
        .unwrap_cleanly();

    // Tests on the BaseType layer

    assert!(struct_type.has_function("test_function".into()));

    let signature = struct_type.get_func_signature("test_function".into());

    assert_eq!(
        signature.0,
        vec![
            Type::TypeParameter {
                name: "K".into(),
                param_ind: 0
            },
            Type::TypeParameter {
                name: "V".into(),
                param_ind: 1
            }
        ]
    );

    assert_eq!(
        signature.1,
        Type::TypeParameter {
            name: "V".into(),
            param_ind: 1
        }
    );

    // Tests on the BaseTypeInstance layer

    let struct_instance = BaseTypeInstance::new(
        struct_type.clone(),
        vec![],
        vec![int_type.clone(), boolean_type.clone()], // int is K, bool is V
    );

    assert!(struct_instance.has_function("test_function".into()));

    let signature = struct_instance.get_func_signature("test_function".into());

    assert_eq!(signature.0, vec![int_type, boolean_type.clone()]);
    assert_eq!(signature.1, boolean_type);
}

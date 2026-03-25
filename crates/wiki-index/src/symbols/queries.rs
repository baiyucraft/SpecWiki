//! definition、import、call、heritage 共用的一组 tree-sitter queries。

pub const TYPESCRIPT_QUERY: &str = r#"
(class_declaration name: (type_identifier) @name) @definition.class
(interface_declaration name: (type_identifier) @name) @definition.interface
(function_declaration name: (identifier) @name) @definition.function
(method_definition name: (property_identifier) @name) @definition.method
(lexical_declaration (variable_declarator name: (identifier) @name value: (arrow_function))) @definition.function
(lexical_declaration (variable_declarator name: (identifier) @name value: (function_expression))) @definition.function
(export_statement declaration: (lexical_declaration (variable_declarator name: (identifier) @name value: (arrow_function)))) @definition.function
(export_statement declaration: (lexical_declaration (variable_declarator name: (identifier) @name value: (function_expression)))) @definition.function
(import_statement source: (string) @import.source) @import
(call_expression function: (identifier) @call.name) @call
(call_expression function: (member_expression object: (_) @call.receiver property: (property_identifier) @call.name)) @call
(class_declaration
  name: (type_identifier) @heritage.owner
  (class_heritage (extends_clause value: (identifier) @heritage.target))) @heritage.extends
(class_declaration
  name: (type_identifier) @heritage.owner
  (class_heritage (implements_clause (type_identifier) @heritage.target))) @heritage.implements
"#;

pub const JAVASCRIPT_QUERY: &str = r#"
(class_declaration name: (identifier) @name) @definition.class
(function_declaration name: (identifier) @name) @definition.function
(method_definition name: (property_identifier) @name) @definition.method
(lexical_declaration (variable_declarator name: (identifier) @name value: (arrow_function))) @definition.function
(lexical_declaration (variable_declarator name: (identifier) @name value: (function_expression))) @definition.function
(export_statement declaration: (lexical_declaration (variable_declarator name: (identifier) @name value: (arrow_function)))) @definition.function
(export_statement declaration: (lexical_declaration (variable_declarator name: (identifier) @name value: (function_expression)))) @definition.function
(import_statement source: (string) @import.source) @import
(call_expression function: (identifier) @call.name) @call
(call_expression function: (member_expression object: (_) @call.receiver property: (property_identifier) @call.name)) @call
(class_declaration
  name: (identifier) @heritage.owner
  (class_heritage (identifier) @heritage.target)) @heritage.extends
"#;

pub const PYTHON_QUERY: &str = r#"
(class_definition name: (identifier) @name) @definition.class
(function_definition name: (identifier) @name) @definition.function
(import_statement name: (dotted_name) @import.source) @import
(import_from_statement module_name: (dotted_name) @import.source) @import
(call function: (identifier) @call.name) @call
(call function: (attribute object: (_) @call.receiver attribute: (identifier) @call.name)) @call
(class_definition
  name: (identifier) @heritage.owner
  superclasses: (argument_list (identifier) @heritage.target)) @heritage.extends
"#;

pub const JAVA_QUERY: &str = r#"
(class_declaration name: (identifier) @name) @definition.class
(interface_declaration name: (identifier) @name) @definition.interface
(enum_declaration name: (identifier) @name) @definition.enum
(annotation_type_declaration name: (identifier) @name) @definition.annotation
(method_declaration name: (identifier) @name) @definition.method
(constructor_declaration name: (identifier) @name) @definition.constructor
(import_declaration (_) @import.source) @import
(method_invocation name: (identifier) @call.name) @call
(method_invocation object: (_) @call.receiver name: (identifier) @call.name) @call
(class_declaration
  name: (identifier) @heritage.owner
  (superclass (type_identifier) @heritage.target)) @heritage.extends
(class_declaration
  name: (identifier) @heritage.owner
  (super_interfaces (type_list (type_identifier) @heritage.target))) @heritage.implements
"#;

pub const C_QUERY: &str = r#"
(function_definition declarator: (function_declarator declarator: (identifier) @name)) @definition.function
(declaration declarator: (function_declarator declarator: (identifier) @name)) @definition.function
(struct_specifier name: (type_identifier) @name) @definition.struct
(union_specifier name: (type_identifier) @name) @definition.union
(enum_specifier name: (type_identifier) @name) @definition.enum
(type_definition declarator: (type_identifier) @name) @definition.typedef
(preproc_function_def name: (identifier) @name) @definition.macro
(preproc_def name: (identifier) @name) @definition.macro
(preproc_include path: (_) @import.source) @import
(call_expression function: (identifier) @call.name) @call
(call_expression function: (field_expression argument: (_) @call.receiver field: (field_identifier) @call.name)) @call
"#;

pub const GO_QUERY: &str = r#"
(function_declaration name: (identifier) @name) @definition.function
(method_declaration name: (field_identifier) @name) @definition.method
(type_declaration (type_spec name: (type_identifier) @name type: (struct_type))) @definition.struct
(type_declaration (type_spec name: (type_identifier) @name type: (interface_type))) @definition.interface
(type_declaration (type_spec name: (type_identifier) @name)) @definition.type
(import_declaration (import_spec path: (interpreted_string_literal) @import.source)) @import
(import_declaration (import_spec_list (import_spec path: (interpreted_string_literal) @import.source))) @import
(call_expression function: (identifier) @call.name) @call
(call_expression function: (selector_expression operand: (_) @call.receiver field: (field_identifier) @call.name)) @call
"#;

pub const CPP_QUERY: &str = r#"
(class_specifier name: (type_identifier) @name) @definition.class
(struct_specifier name: (type_identifier) @name) @definition.struct
(namespace_definition name: (namespace_identifier) @name) @definition.namespace
(enum_specifier name: (type_identifier) @name) @definition.enum
(function_definition declarator: (function_declarator declarator: (identifier) @name)) @definition.function
(function_definition declarator: (function_declarator declarator: (qualified_identifier name: (identifier) @name))) @definition.method
(template_declaration (class_specifier name: (type_identifier) @name)) @definition.template
(template_declaration (function_definition declarator: (function_declarator declarator: (identifier) @name))) @definition.template
(preproc_include path: (_) @import.source) @import
(call_expression function: (identifier) @call.name) @call
(call_expression function: (field_expression argument: (_) @call.receiver field: (field_identifier) @call.name)) @call
(call_expression function: (qualified_identifier name: (identifier) @call.name)) @call
(class_specifier
  name: (type_identifier) @heritage.owner
  (base_class_clause (type_identifier) @heritage.target)) @heritage.extends
(class_specifier
  name: (type_identifier) @heritage.owner
  (base_class_clause (access_specifier) (type_identifier) @heritage.target)) @heritage.extends
"#;

pub const CSHARP_QUERY: &str = r#"
(class_declaration name: (identifier) @name) @definition.class
(interface_declaration name: (identifier) @name) @definition.interface
(struct_declaration name: (identifier) @name) @definition.struct
(enum_declaration name: (identifier) @name) @definition.enum
(record_declaration name: (identifier) @name) @definition.record
(delegate_declaration name: (identifier) @name) @definition.delegate
(namespace_declaration name: (identifier) @name) @definition.namespace
(namespace_declaration name: (qualified_name) @name) @definition.namespace
(method_declaration name: (identifier) @name) @definition.method
(local_function_statement name: (identifier) @name) @definition.function
(constructor_declaration name: (identifier) @name) @definition.constructor
(property_declaration name: (identifier) @name) @definition.property
(using_directive (_) @import.source) @import
(invocation_expression function: (identifier) @call.name) @call
(invocation_expression function: (member_access_expression expression: (_) @call.receiver name: (identifier) @call.name)) @call
(invocation_expression function: (member_access_expression expression: (_) @call.receiver name: (generic_name) @call.name)) @call
(class_declaration
  name: (identifier) @heritage.owner
  (base_list [(type) (primary_constructor_base_type)] @heritage.target)) @heritage.extends
"#;

pub const RUST_QUERY: &str = r#"
(function_item name: (identifier) @name) @definition.function
(struct_item name: (type_identifier) @name) @definition.struct
(enum_item name: (type_identifier) @name) @definition.enum
(trait_item name: (type_identifier) @name) @definition.trait
(impl_item type: (type_identifier) @name) @definition.impl
(mod_item name: (identifier) @name) @definition.module
(type_item name: (type_identifier) @name) @definition.type
(const_item name: (identifier) @name) @definition.const
(static_item name: (identifier) @name) @definition.static
(macro_definition name: (identifier) @name) @definition.macro
(use_declaration argument: (_) @import.source) @import
(call_expression function: (identifier) @call.name) @call
(call_expression function: (field_expression value: (_) @call.receiver field: (field_identifier) @call.name)) @call
(call_expression function: (scoped_identifier path: (_) @call.receiver name: (identifier) @call.name)) @call
(call_expression function: (generic_function function: (identifier) @call.name)) @call
(impl_item
  trait: (type_identifier) @heritage.target
  type: (type_identifier) @heritage.owner) @heritage.implements
(impl_item
  trait: (generic_type type: (type_identifier) @heritage.target)
  type: (type_identifier) @heritage.owner) @heritage.implements
"#;

pub const PHP_QUERY: &str = r#"
(namespace_definition name: (namespace_name) @name) @definition.namespace
(class_declaration name: (name) @name) @definition.class
(interface_declaration name: (name) @name) @definition.interface
(trait_declaration name: (name) @name) @definition.trait
(enum_declaration name: (name) @name) @definition.enum
(function_definition name: (name) @name) @definition.function
(method_declaration name: (name) @name) @definition.method
(property_declaration (property_element (variable_name (name) @name))) @definition.property
(namespace_use_declaration (namespace_use_clause (qualified_name) @import.source)) @import
(function_call_expression function: (name) @call.name) @call
(member_call_expression object: (_) @call.receiver name: (name) @call.name) @call
(nullsafe_member_call_expression object: (_) @call.receiver name: (name) @call.name) @call
(scoped_call_expression scope: (_) @call.receiver name: (name) @call.name) @call
(class_declaration
  name: (name) @heritage.owner
  (base_clause [(name) (qualified_name)] @heritage.target)) @heritage.extends
(class_declaration
  name: (name) @heritage.owner
  (class_interface_clause [(name) (qualified_name)] @heritage.target)) @heritage.implements
"#;

pub const KOTLIN_QUERY: &str = r#"
(class_declaration "interface" name: (identifier) @name) @definition.interface
(class_declaration "fun" "interface" name: (identifier) @name) @definition.interface
(class_declaration "class" name: (identifier) @name) @definition.class
(object_declaration name: (identifier) @name) @definition.class
(companion_object name: (identifier) @name) @definition.class
(function_declaration name: (identifier) @name) @definition.function
(property_declaration (variable_declaration (identifier) @name)) @definition.property
(property_declaration (multi_variable_declaration (variable_declaration (identifier) @name))) @definition.property
(enum_entry (identifier) @name) @definition.enum
(type_alias type: (identifier) @name) @definition.type
(import (qualified_identifier) @import.source) @import
(call_expression (identifier) @call.name) @call
(call_expression (navigation_expression (_) @call.receiver (identifier) @call.name)) @call
(constructor_invocation (_) @call.name) @call
(class_declaration
  name: (identifier) @heritage.owner
  (delegation_specifiers (delegation_specifier (_) @heritage.target))) @heritage.extends
(class_declaration
  name: (identifier) @heritage.owner
  (delegation_specifiers (delegation_specifier (constructor_invocation (_) @heritage.target)))) @heritage.extends
"#;

pub const SWIFT_QUERY: &str = r#"
(class_declaration "class" name: (type_identifier) @name) @definition.class
(class_declaration "struct" name: (type_identifier) @name) @definition.struct
(class_declaration "enum" name: (type_identifier) @name) @definition.enum
(class_declaration "extension" name: (user_type (type_identifier) @name)) @definition.class
(class_declaration "actor" name: (type_identifier) @name) @definition.class
(protocol_declaration name: (type_identifier) @name) @definition.interface
(typealias_declaration name: (type_identifier) @name) @definition.type
(function_declaration name: (simple_identifier) @name) @definition.function
(protocol_function_declaration name: (simple_identifier) @name) @definition.method
(init_declaration) @definition.constructor
(property_declaration (pattern (simple_identifier) @name)) @definition.property
(import_declaration (identifier (simple_identifier) @import.source)) @import
(call_expression (simple_identifier) @call.name) @call
(call_expression (navigation_expression (navigation_suffix (simple_identifier) @call.name))) @call
(class_declaration
  name: (type_identifier) @heritage.owner
  (inheritance_specifier inherits_from: (user_type (type_identifier) @heritage.target))) @heritage.extends
(protocol_declaration
  name: (type_identifier) @heritage.owner
  (inheritance_specifier inherits_from: (user_type (type_identifier) @heritage.target))) @heritage.extends
"#;


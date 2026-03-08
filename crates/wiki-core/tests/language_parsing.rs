use wiki_core::repo::parsers::{extract_source_aliases, extract_source_dependency_targets};

#[test]
fn language_parsing_supports_csharp_kotlin_php_and_swift() {
    let csharp_content = r#"
namespace Demo.Core;
using Demo.Shared.Services;
"#;
    let kotlin_content = r#"
package demo.core
import demo.shared.Service
"#;
    let php_content = r#"
<?php
namespace Demo\Core;
use Demo\Shared\Service;
require_once "../bootstrap.php";
"#;
    let swift_content = r#"
import Foundation
import SharedCore
"#;

    assert!(extract_source_aliases("csharp", csharp_content)
        .iter()
        .any(|alias| alias == "Demo.Core"));
    assert!(extract_source_dependency_targets("csharp", csharp_content)
        .iter()
        .any(|target| target == "Demo/Shared/Services"));

    assert!(extract_source_aliases("kotlin", kotlin_content)
        .iter()
        .any(|alias| alias == "demo.core"));
    assert!(extract_source_dependency_targets("kotlin", kotlin_content)
        .iter()
        .any(|target| target == "demo/shared/Service"));

    assert!(extract_source_aliases("php", php_content)
        .iter()
        .any(|alias| alias == "Demo\\Core"));
    assert!(extract_source_dependency_targets("php", php_content)
        .iter()
        .any(|target| target == "Demo/Shared/Service"));
    assert!(extract_source_dependency_targets("php", php_content)
        .iter()
        .any(|target| target == "../bootstrap.php"));

    assert!(extract_source_dependency_targets("swift", swift_content)
        .iter()
        .any(|target| target == "SharedCore"));
}

#[test]
fn language_parsing_supports_react_vue_and_svelte_imports() {
    let react_content = r#"
import { Button } from "./Button";
const lazyPage = import("./Page");
"#;
    let vue_content = r#"
<template><div /></template>
<script setup lang="ts">
import { useApi } from "@/api";
</script>
"#;
    let svelte_content = r#"
<script>
  import App from "./App.svelte";
</script>
"#;

    assert!(extract_source_dependency_targets("react", react_content)
        .iter()
        .any(|target| target == "./Button"));
    assert!(extract_source_dependency_targets("react", react_content)
        .iter()
        .any(|target| target == "./Page"));

    assert!(extract_source_dependency_targets("vue", vue_content)
        .iter()
        .any(|target| target == "@/api"));
    assert!(extract_source_dependency_targets("svelte", svelte_content)
        .iter()
        .any(|target| target == "./App.svelte"));
}

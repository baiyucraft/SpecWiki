use wiki_knowledge::{plan_projection_intents, project_page_plans, ProjectionPolicyError};
use wiki_model::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
use wiki_model::domain::projection::{
    PageProjectionOverride, PageProjectionPriority, ProjectionEligibility, ProjectionPolicy,
};

fn tree_with_many_leaf_units(reverse_input: bool) -> KnowledgeTree {
    let mut tree = KnowledgeTree::new("overview");
    let mut overview = KnowledgeUnit::new(UnitType::Overview, "Overview", "system", "INDEX.md");
    overview.id = "overview".to_string();
    overview.child_unit_ids = vec!["architecture".to_string(), "domain-a".to_string()];

    let mut architecture = KnowledgeUnit::new(
        UnitType::Architecture,
        "Architecture",
        "system",
        "00-项目总览/00-系统架构.md",
    );
    architecture.id = "architecture".to_string();
    architecture.parent_unit_id = Some("overview".to_string());

    let mut domain = KnowledgeUnit::new(
        UnitType::DomainIndex,
        "Domain A",
        "domain-a",
        "domain-a/INDEX.md",
    );
    domain.id = "domain-a".to_string();
    domain.parent_unit_id = Some("overview".to_string());
    domain.child_unit_ids = (0..5).map(|index| format!("leaf-{index}")).collect();

    let mut units = vec![overview, architecture, domain];
    for index in 0..5 {
        let mut leaf = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            format!("Leaf {index}"),
            "domain-a",
            format!("domain-a/leaf-{index}.md"),
        );
        leaf.id = format!("leaf-{index}");
        leaf.parent_unit_id = Some("domain-a".to_string());
        leaf.priority = (5 - index) as f32;
        units.push(leaf);
    }
    if reverse_input {
        units.reverse();
    }
    for unit in units {
        tree.add_unit(unit);
    }
    tree.build_processing_order();
    tree
}

#[test]
fn projection_policy_is_bounded_and_deterministic() {
    let policy = ProjectionPolicy {
        max_projected_leaf_pages_per_domain: 2,
        ..ProjectionPolicy::default()
    };
    let first_tree = tree_with_many_leaf_units(false);
    let second_tree = tree_with_many_leaf_units(true);
    let first = plan_projection_intents(&first_tree, &policy, &[]).unwrap();
    let second = plan_projection_intents(&second_tree, &policy, &[]).unwrap();

    assert_eq!(first, second);
    assert_eq!(first.len(), 8, "every unit must receive a decision");
    assert_eq!(
        first
            .iter()
            .filter(|decision| decision.eligibility == ProjectionEligibility::Required)
            .count(),
        3
    );
    assert_eq!(
        first
            .iter()
            .filter(|decision| {
                decision.domain_ref == "domain-a"
                    && decision.eligibility == ProjectionEligibility::Selected
            })
            .count(),
        2
    );
    let pages = project_page_plans(&first_tree, &first).unwrap();
    assert_eq!(
        pages.len(),
        5,
        "projection must keep three structural pages and only two domain leaves"
    );
}

#[test]
fn projection_overrides_priority_and_hints_follow_policy_authority() {
    let tree = tree_with_many_leaf_units(false);
    let policy = ProjectionPolicy {
        include: vec![PageProjectionOverride {
            unit_ref: "leaf-4".to_string(),
        }],
        exclude: vec![PageProjectionOverride {
            unit_ref: "leaf-0".to_string(),
        }],
        priority: vec![PageProjectionPriority {
            path: "domain-a/10-leaf-3.md".to_string(),
            boost: 100,
        }],
        max_projected_leaf_pages_per_domain: 1,
        ..ProjectionPolicy::default()
    };
    let decisions = plan_projection_intents(&tree, &policy, &[]).unwrap();
    let selected = decisions
        .iter()
        .filter(|decision| decision.eligibility == ProjectionEligibility::Selected)
        .map(|decision| decision.unit_ref.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        selected,
        std::collections::BTreeSet::from(["leaf-3", "leaf-4"])
    );

    let mut hints_only = policy.clone();
    hints_only.hint_refs = vec!["compose:extra-context".to_string()];
    let hinted = plan_projection_intents(&tree, &hints_only, &[]).unwrap();
    assert_eq!(
        decisions
            .iter()
            .map(|decision| decision.eligibility)
            .collect::<Vec<_>>(),
        hinted
            .iter()
            .map(|decision| decision.eligibility)
            .collect::<Vec<_>>()
    );

    let conflict = ProjectionPolicy {
        include: vec![PageProjectionOverride {
            unit_ref: "leaf-1".to_string(),
        }],
        exclude: vec![PageProjectionOverride {
            unit_ref: "leaf-1".to_string(),
        }],
        ..ProjectionPolicy::default()
    };
    assert_eq!(
        plan_projection_intents(&tree, &conflict, &[]),
        Err(ProjectionPolicyError::ConflictingOverride(
            "leaf-1".to_string()
        ))
    );
}

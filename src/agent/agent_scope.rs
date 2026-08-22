// AgentScope — defines and enforces scope boundaries per agent.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AgentScopeType {
    Project(String),
    Directory(String),
    ApiEndpoint(String),
    Dataset(String),
    Global,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AgentScopeConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Whether Global scope is allowed for non-admin agents.
    #[serde(default)]
    pub allow_global_for_non_admin: bool,
}

<<<<<<< HEAD
fn default_enabled() -> bool {
    true
}

impl Default for AgentScopeConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            allow_global_for_non_admin: false,
        }
    }
=======
fn default_enabled() -> bool { true }

impl Default for AgentScopeConfig {
    fn default() -> Self { Self { enabled: default_enabled(), allow_global_for_non_admin: false } }
>>>>>>> 4b60ced (docs: update README)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScopeVerdict {
    pub violated: bool,
    pub reason: String,
    pub scope_type: String,
}

<<<<<<< HEAD
pub struct AgentScope {
    config: AgentScopeConfig,
}

impl AgentScope {
    pub fn new(config: &AgentScopeConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn evaluate(
        &self,
        scope: &AgentScopeType,
        action: &str,
        target: &Option<String>,
    ) -> ScopeVerdict {
        if !self.config.enabled {
            return ScopeVerdict {
                violated: false,
                reason: "scope checking disabled".into(),
                scope_type: format!("{:?}", scope),
            };
=======
pub struct AgentScope { config: AgentScopeConfig }

impl AgentScope {
    pub fn new(config: &AgentScopeConfig) -> Self { Self { config: config.clone() } }

    pub fn evaluate(&self, scope: &AgentScopeType, action: &str, target: &Option<String>) -> ScopeVerdict {
        if !self.config.enabled {
            return ScopeVerdict { violated: false, reason: "scope checking disabled".into(), scope_type: format!("{:?}", scope) };
>>>>>>> 4b60ced (docs: update README)
        }

        // If no target specified, allow (can't validate).
        let target_str = match target {
            Some(t) if !t.is_empty() => t.as_str(),
<<<<<<< HEAD
            _ => {
                return ScopeVerdict {
                    violated: false,
                    reason: "no target to validate".into(),
                    scope_type: format!("{:?}", scope),
                }
            }
        };

        // Check dangerous actions outside Global scope.
        let dangerous_actions = [
            "write_file",
            "delete_file",
            "execute",
            "shell",
            "admin",
            "config",
        ];
        let is_dangerous = dangerous_actions
            .iter()
            .any(|d| action.to_lowercase().contains(d));
=======
            _ => return ScopeVerdict { violated: false, reason: "no target to validate".into(), scope_type: format!("{:?}", scope) },
        };

        // Check dangerous actions outside Global scope.
        let dangerous_actions = ["write_file", "delete_file", "execute", "shell", "admin", "config"];
        let is_dangerous = dangerous_actions.iter().any(|d| action.to_lowercase().contains(d));
>>>>>>> 4b60ced (docs: update README)

        match scope {
            AgentScopeType::Project(prefix) => {
                if is_dangerous && !target_str.starts_with(prefix) {
<<<<<<< HEAD
                    return ScopeVerdict {
                        violated: true,
                        reason: format!(
                            "action '{}' on '{}' outside project scope '{}'",
                            action, target_str, prefix
                        ),
                        scope_type: format!("Project({})", prefix),
                    };
=======
                    return ScopeVerdict { violated: true, reason: format!("action '{}' on '{}' outside project scope '{}'", action, target_str, prefix), scope_type: format!("Project({})", prefix) };
>>>>>>> 4b60ced (docs: update README)
                }
            }
            AgentScopeType::Directory(dir) => {
                if is_dangerous && !target_str.starts_with(dir) {
<<<<<<< HEAD
                    return ScopeVerdict {
                        violated: true,
                        reason: format!(
                            "action '{}' on '{}' outside directory scope '{}'",
                            action, target_str, dir
                        ),
                        scope_type: format!("Directory({})", dir),
                    };
=======
                    return ScopeVerdict { violated: true, reason: format!("action '{}' on '{}' outside directory scope '{}'", action, target_str, dir), scope_type: format!("Directory({})", dir) };
>>>>>>> 4b60ced (docs: update README)
                }
            }
            AgentScopeType::ApiEndpoint(endpoint) => {
                if !target_str.starts_with(endpoint) && target_str.contains("://") {
<<<<<<< HEAD
                    return ScopeVerdict {
                        violated: true,
                        reason: format!(
                            "API call to '{}' outside endpoint scope '{}'",
                            target_str, endpoint
                        ),
                        scope_type: format!("ApiEndpoint({})", endpoint),
                    };
=======
                    return ScopeVerdict { violated: true, reason: format!("API call to '{}' outside endpoint scope '{}'", target_str, endpoint), scope_type: format!("ApiEndpoint({})", endpoint) };
>>>>>>> 4b60ced (docs: update README)
                }
            }
            AgentScopeType::Dataset(dataset) => {
                if is_dangerous && !target_str.starts_with(dataset) {
<<<<<<< HEAD
                    return ScopeVerdict {
                        violated: true,
                        reason: format!(
                            "action on dataset '{}' outside scope '{}'",
                            target_str, dataset
                        ),
                        scope_type: format!("Dataset({})", dataset),
                    };
=======
                    return ScopeVerdict { violated: true, reason: format!("action on dataset '{}' outside scope '{}'", target_str, dataset), scope_type: format!("Dataset({})", dataset) };
>>>>>>> 4b60ced (docs: update README)
                }
            }
            AgentScopeType::Global => {
                if is_dangerous && !self.config.allow_global_for_non_admin {
<<<<<<< HEAD
                    return ScopeVerdict {
                        violated: true,
                        reason: "dangerous actions not allowed in global scope".into(),
                        scope_type: "Global".into(),
                    };
=======
                    return ScopeVerdict { violated: true, reason: "dangerous actions not allowed in global scope".into(), scope_type: "Global".into() };
>>>>>>> 4b60ced (docs: update README)
                }
            }
        }

<<<<<<< HEAD
        ScopeVerdict {
            violated: false,
            reason: format!("action '{}' on '{}' within scope", action, target_str),
            scope_type: format!("{:?}", scope),
        }
=======
        ScopeVerdict { violated: false, reason: format!("action '{}' on '{}' within scope", action, target_str), scope_type: format!("{:?}", scope) }
>>>>>>> 4b60ced (docs: update README)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

<<<<<<< HEAD
    fn default_scope() -> AgentScope {
        AgentScope::new(&AgentScopeConfig::default())
    }
=======
    fn default_scope() -> AgentScope { AgentScope::new(&AgentScopeConfig::default()) }
>>>>>>> 4b60ced (docs: update README)

    #[test]
    fn within_project_scope() {
        let s = default_scope();
<<<<<<< HEAD
        let v = s.evaluate(
            &AgentScopeType::Project("/my-project".into()),
            "write_file",
            &Some("/my-project/src/main.rs".into()),
        );
=======
        let v = s.evaluate(&AgentScopeType::Project("/my-project".into()), "write_file", &Some("/my-project/src/main.rs".into()));
>>>>>>> 4b60ced (docs: update README)
        assert!(!v.violated);
    }

    #[test]
    fn outside_project_scope() {
        let s = default_scope();
<<<<<<< HEAD
        let v = s.evaluate(
            &AgentScopeType::Project("/my-project".into()),
            "write_file",
            &Some("/etc/passwd".into()),
        );
=======
        let v = s.evaluate(&AgentScopeType::Project("/my-project".into()), "write_file", &Some("/etc/passwd".into()));
>>>>>>> 4b60ced (docs: update README)
        assert!(v.violated);
    }

    #[test]
    fn global_scope_dangerous() {
        let s = default_scope();
<<<<<<< HEAD
        let v = s.evaluate(
            &AgentScopeType::Global,
            "shell_exec",
            &Some("anything".into()),
        );
=======
        let v = s.evaluate(&AgentScopeType::Global, "shell_exec", &Some("anything".into()));
>>>>>>> 4b60ced (docs: update README)
        assert!(v.violated);
    }

    #[test]
    fn read_within_scope() {
        let s = default_scope();
<<<<<<< HEAD
        let v = s.evaluate(
            &AgentScopeType::Project("/proj".into()),
            "read_file",
            &Some("/other/file.txt".into()),
        );
=======
        let v = s.evaluate(&AgentScopeType::Project("/proj".into()), "read_file", &Some("/other/file.txt".into()));
>>>>>>> 4b60ced (docs: update README)
        // Read is not dangerous, so it passes.
        assert!(!v.violated);
    }

    #[test]
    fn disabled_allows_all() {
<<<<<<< HEAD
        let s = AgentScope::new(&AgentScopeConfig {
            enabled: false,
            ..Default::default()
        });
        let v = s.evaluate(
            &AgentScopeType::Project("/a".into()),
            "shell",
            &Some("/etc/shadow".into()),
        );
=======
        let s = AgentScope::new(&AgentScopeConfig { enabled: false, ..Default::default() });
        let v = s.evaluate(&AgentScopeType::Project("/a".into()), "shell", &Some("/etc/shadow".into()));
>>>>>>> 4b60ced (docs: update README)
        assert!(!v.violated);
    }
}

use std::collections::HashSet;

use crate::domain::instruction::*;
use crate::domain::validation::{ValidationReport, ValidationSeverity};
use crate::domain::AgentError;

pub struct InstructionAnalyzer;

impl InstructionAnalyzer {
    fn push_capability(
        caps: &mut Vec<Capability>,
        name: &str,
        description: &str,
        category: CapabilityCategory,
        priority: CapabilityPriority,
    ) {
        caps.push(Capability {
            name: name.to_string(),
            description: description.to_string(),
            category,
            priority,
            required_tools: Vec::new(),
            estimated_tokens: 512,
        });
    }

    pub fn analyze_instruction(
        instruction: UserInstruction,
    ) -> Result<AnalyzedInstruction, String> {
        Self::validate(&instruction).map_err(|err| err.to_string())?;
        let complexity = Self::estimate_complexity(&instruction.instruction_text);
        let capabilities = Self::extract_capabilities(&instruction.instruction_text);
        let model_requirements =
            Self::derive_model_requirements(&instruction, complexity, &capabilities);
        let agent_configuration = Self::build_agent_configuration(&instruction, &capabilities);
        let coordination = Self::determine_coordination(&instruction, &capabilities);
        let duration = Self::estimate_duration(complexity, instruction.context.as_ref());
        let confidence = Self::confidence_score(&instruction, &capabilities, complexity);

        Ok(AnalyzedInstruction {
            original_instruction: instruction,
            extracted_capabilities: capabilities,
            model_requirements,
            agent_configuration,
            coordination_requirements: coordination,
            estimated_complexity: complexity,
            estimated_duration: duration,
            confidence_score: confidence,
        })
    }

    fn validate(instruction: &UserInstruction) -> Result<(), AgentError> {
        let mut report = ValidationReport::ok();
        if instruction.instruction_text.trim().is_empty() {
            report = report.with_issue(
                "instruction_text",
                "Instruction text cannot be empty",
                ValidationSeverity::Error,
            );
        }
        if instruction.user_id.trim().is_empty() {
            report = report.with_issue(
                "user_id",
                "User identifier is required",
                ValidationSeverity::Error,
            );
        }
        if report.valid {
            Ok(())
        } else {
            Err(AgentError::ValidationFailed(
                report
                    .issues
                    .into_iter()
                    .map(|issue| format!("{}: {}", issue.field, issue.message))
                    .collect::<Vec<_>>()
                    .join(", "),
            ))
        }
    }

    fn estimate_complexity(text: &str) -> ComplexityLevel {
        let word_count = text.split_whitespace().count();
        let has_multi_stage = text.contains("step by step") || text.contains("plan");
        let has_specialized_keywords = [
            "optimize",
            "architecture",
            "multi-agent",
            "distributed",
            "pipeline",
        ]
        .iter()
        .any(|k| text.to_lowercase().contains(k));

        match (word_count, has_multi_stage, has_specialized_keywords) {
            (0..=10, false, false) => ComplexityLevel::Simple,
            (0..=25, _, false) => ComplexityLevel::Moderate,
            (_, true, false) => ComplexityLevel::Complex,
            (_, _, true) => ComplexityLevel::Expert,
            _ => ComplexityLevel::Complex,
        }
    }

    fn extract_capabilities(text: &str) -> Vec<Capability> {
        let lower = text.to_lowercase();
        let mut caps = Vec::new();

        if lower.contains("code") || lower.contains("build") || lower.contains("implement") {
            Self::push_capability(
                &mut caps,
                "Code Generation",
                "Generate and reason about source code",
                CapabilityCategory::CodeGeneration,
                CapabilityPriority::Essential,
            );
        }
        if lower.contains("test") || lower.contains("qa") || lower.contains("validate") {
            Self::push_capability(
                &mut caps,
                "Quality Assurance",
                "Design and run validation tasks",
                CapabilityCategory::Coordination,
                CapabilityPriority::Important,
            );
        }
        if lower.contains("analyze") || lower.contains("data") {
            Self::push_capability(
                &mut caps,
                "Data Analysis",
                "Perform structured reasoning over datasets",
                CapabilityCategory::DataAnalysis,
                CapabilityPriority::Important,
            );
        }
        if lower.contains("summarize") || lower.contains("report") {
            Self::push_capability(
                &mut caps,
                "Summarization",
                "Create concise reports or summaries",
                CapabilityCategory::ContentCreation,
                CapabilityPriority::Helpful,
            );
        }
        if lower.contains("coordinate") || lower.contains("team") || lower.contains("multi-agent") {
            Self::push_capability(
                &mut caps,
                "Coordination",
                "Manage collaboration between specialized agents",
                CapabilityCategory::Coordination,
                CapabilityPriority::Important,
            );
        }
        if caps.is_empty() {
            Self::push_capability(
                &mut caps,
                "General Reasoning",
                "Interpret instructions and produce structured plans",
                CapabilityCategory::ProblemSolving,
                CapabilityPriority::Essential,
            );
        }

        let mut unique = Vec::new();
        let mut seen = HashSet::new();
        for cap in caps.into_iter() {
            if seen.insert(cap.name.clone()) {
                unique.push(cap);
            }
        }
        unique
    }
    fn derive_model_requirements(
        instruction: &UserInstruction,
        complexity: ComplexityLevel,
        capabilities: &[Capability],
    ) -> ModelRequirements {
        let mut requirements = ModelRequirements::default();
        requirements.minimum_context_length = match complexity {
            ComplexityLevel::Simple => 1024,
            ComplexityLevel::Moderate => 2048,
            ComplexityLevel::Complex => 4096,
            ComplexityLevel::Expert => 8192,
        };

        if capabilities.iter().any(|cap| {
            matches!(
                cap.category,
                CapabilityCategory::CodeGeneration | CapabilityCategory::DataAnalysis
            )
        }) {
            requirements.preferred_precision = ModelPrecision::FP16;
        }

        requirements.specialized_requirements = capabilities
            .iter()
            .filter(|cap| matches!(cap.priority, CapabilityPriority::Essential))
            .map(|cap| cap.name.clone())
            .collect();

        requirements.reasoning_capability = match complexity {
            ComplexityLevel::Simple => ReasoningLevel::Basic,
            ComplexityLevel::Moderate => ReasoningLevel::Intermediate,
            ComplexityLevel::Complex => ReasoningLevel::Advanced,
            ComplexityLevel::Expert => ReasoningLevel::Expert,
        };

        requirements.creativity_requirement =
            match instruction.preferences.as_ref().map(|p| p.creativity_level) {
                Some(CreativityLevel::Conservative) => CreativityRequirement::Low,
                Some(CreativityLevel::Balanced) => CreativityRequirement::Medium,
                Some(CreativityLevel::Creative) => CreativityRequirement::High,
                Some(CreativityLevel::Experimental) => CreativityRequirement::High,
                None => CreativityRequirement::Medium,
            };

        requirements
    }

    fn build_agent_configuration(
        instruction: &UserInstruction,
        capabilities: &[Capability],
    ) -> AgentConfiguration {
        let agent_type = if capabilities
            .iter()
            .any(|cap| matches!(cap.category, CapabilityCategory::CodeGeneration))
        {
            AgentType::CodeAssistant
        } else if capabilities
            .iter()
            .any(|cap| matches!(cap.category, CapabilityCategory::DataAnalysis))
        {
            AgentType::DataAnalyst
        } else {
            AgentType::GeneralAssistant
        };

        let mut config = AgentConfiguration::default();
        config.agent_type = agent_type;

        if let Some(preferences) = instruction.preferences.as_ref() {
            config.communication_style = match preferences.response_style {
                ResponseStyle::Concise => CommunicationStyle::Direct,
                ResponseStyle::Detailed => CommunicationStyle::Professional,
                ResponseStyle::Conversational => CommunicationStyle::Conversational,
                ResponseStyle::Technical => CommunicationStyle::Technical,
            };

            config.decision_making = match preferences.detail_level {
                DetailLevel::Summary => DecisionMakingStyle::Conservative,
                DetailLevel::Standard => DecisionMakingStyle::Balanced,
                DetailLevel::Comprehensive => DecisionMakingStyle::Collaborative,
                DetailLevel::Expert => DecisionMakingStyle::Aggressive,
            };
        }

        config.tool_access = capabilities
            .iter()
            .map(|cap| cap.name.to_lowercase().replace(' ', "_"))
            .collect();

        config
            .behavior_rules
            .push("Always verify safety constraints before execution".to_string());
        config
    }

    fn determine_coordination(
        instruction: &UserInstruction,
        capabilities: &[Capability],
    ) -> CoordinationRequirements {
        let mut requirements = CoordinationRequirements::default();
        let ctx = instruction.context.as_ref();
        let requires_team = ctx.map(|ctx| ctx.collaboration_needed).unwrap_or(false)
            || instruction.instruction_text.to_lowercase().contains("team");

        if requires_team {
            requirements.requires_coordination = true;
            requirements.agent_count = ctx
                .and_then(|c| Some(c.external_tools_required.len() as u32 + 2))
                .unwrap_or(2);
            requirements.coordination_type = CoordinationType::Collaborative;
            requirements.communication_protocol = CommunicationProtocol::Centralized;
            requirements.task_distribution = TaskDistributionStrategy::CapabilityBased;
        } else if capabilities.len() > 2 {
            requirements.requires_coordination = true;
            requirements.agent_count = capabilities.len() as u32;
            requirements.coordination_type = CoordinationType::Parallel;
            requirements.communication_protocol = CommunicationProtocol::Direct;
        }

        requirements
    }

    fn estimate_duration(
        complexity: ComplexityLevel,
        context: Option<&InstructionContext>,
    ) -> DurationEstimate {
        let base = match complexity {
            ComplexityLevel::Simple => 10,
            ComplexityLevel::Moderate => 30,
            ComplexityLevel::Complex => 90,
            ComplexityLevel::Expert => 240,
        } as u64;

        let urgency_multiplier = match context.and_then(|ctx| ctx.urgency) {
            Some(UrgencyLevel::Low) => 1.2,
            Some(UrgencyLevel::Normal) => 1.0,
            Some(UrgencyLevel::High) => 0.8,
            Some(UrgencyLevel::Critical) => 0.6,
            None => 1.0,
        };

        let expected = (base as f64 * urgency_multiplier) as u64;

        DurationEstimate {
            min_duration_seconds: (expected as f64 * 0.5) as u64,
            expected_duration_seconds: expected,
            max_duration_seconds: (expected as f64 * 1.5) as u64,
            confidence: match complexity {
                ComplexityLevel::Simple => 0.9,
                ComplexityLevel::Moderate => 0.75,
                ComplexityLevel::Complex => 0.6,
                ComplexityLevel::Expert => 0.45,
            },
        }
    }

    fn confidence_score(
        instruction: &UserInstruction,
        capabilities: &[Capability],
        complexity: ComplexityLevel,
    ) -> f32 {
        let base = match complexity {
            ComplexityLevel::Simple => 0.92_f32,
            ComplexityLevel::Moderate => 0.85_f32,
            ComplexityLevel::Complex => 0.75_f32,
            ComplexityLevel::Expert => 0.62_f32,
        };
        let instruction_bonus = if instruction.context.is_some() {
            0.03_f32
        } else {
            -0.04_f32
        };
        let diversity_penalty = if capabilities.len() > 4 {
            0.05_f32
        } else {
            0.0_f32
        };
        (base + instruction_bonus - diversity_penalty).clamp(0.4_f32, 0.97_f32)
    }
}

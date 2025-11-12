<!--
Sync Impact Report - Constitution Update
=========================================
Version: 0.0.0 → 1.0.0
Change Type: MAJOR (Initial ratification)
Ratification Date: 2025-11-12
Last Amended: 2025-11-12

Modified Principles:
- All principles newly defined (initial constitution)

Added Sections:
- Core Principles (5 principles)
- Development Workflow
- Quality Standards
- Governance

Templates Status:
✅ plan-template.md - Reviewed, Constitution Check section aligns
✅ spec-template.md - Reviewed, user story requirements align
✅ tasks-template.md - Reviewed, task categorization aligns
⚠ commands/*.md - No command files found to update

Follow-up TODOs:
- None (all placeholders filled)
-->

# ChenChen Constitution

## Core Principles

### I. Library-First

Every feature in ChenChen MUST start as a standalone library. This principle ensures modularity, reusability, and independent testability across the codebase.

**Requirements**:
- Libraries MUST be self-contained with clear boundaries
- Libraries MUST be independently testable without external dependencies
- Libraries MUST have documented purpose and API surface
- Libraries MUST NOT be created solely for organizational purposes (no "utility dumping grounds")

**Rationale**: Library-first architecture enforces separation of concerns, enables code reuse, and simplifies testing. It prevents the creation of monolithic, tightly-coupled systems that become difficult to maintain and evolve.

### II. Test-First (NON-NEGOTIABLE)

Test-Driven Development (TDD) is MANDATORY for all code in ChenChen. Tests MUST be written before implementation, reviewed, and confirmed to fail before any production code is written.

**Requirements**:
- Tests MUST be written first and approved by stakeholders
- Tests MUST fail initially (Red state)
- Implementation MUST make tests pass (Green state)
- Code MUST be refactored while keeping tests green (Refactor state)
- Red-Green-Refactor cycle MUST be strictly followed
- NO exceptions - untested code MUST NOT be merged

**Rationale**: TDD ensures that every piece of functionality is testable by design, has clear acceptance criteria, and maintains high code quality. It prevents regressions and serves as living documentation of system behavior.

### III. Simplicity & YAGNI (You Aren't Gonna Need It)

ChenChen MUST start with the simplest possible solution. Complexity MUST be justified and approved before introduction.

**Requirements**:
- Default to simple, direct implementations
- Reject premature abstraction and generalization
- Complexity introductions MUST be documented in plan.md Complexity Tracking table
- Each complexity justification MUST explain:
  - Why the complexity is needed
  - What simpler alternatives were considered
  - Why those alternatives were rejected
- Favor composition over inheritance
- Favor explicit over implicit behavior

**Rationale**: Simplicity reduces cognitive load, improves maintainability, and accelerates development. Most anticipated future needs never materialize; building for them wastes effort and creates maintenance burden.

### IV. Observability

All ChenChen components MUST be observable and debuggable through text-based interfaces.

**Requirements**:
- Libraries MUST support text input/output protocols where applicable
- Stdin/arguments for input, stdout for results, stderr for errors
- Support both JSON (for machine consumption) and human-readable formats
- Structured logging MUST be implemented for all significant operations
- Logs MUST include context (timestamps, correlation IDs, operation names)
- Error messages MUST be actionable and include diagnostic information

**Rationale**: Text-based I/O ensures debuggability across different environments and tools. Structured logging enables effective monitoring, troubleshooting, and system understanding in production.

### V. Versioning & Breaking Changes

All public interfaces in ChenChen MUST follow semantic versioning (MAJOR.MINOR.PATCH).

**Requirements**:
- MAJOR version: Backward-incompatible API changes
- MINOR version: Backward-compatible new functionality
- PATCH version: Backward-compatible bug fixes
- Breaking changes MUST be documented in CHANGELOG.md
- Breaking changes MUST include migration guide
- Deprecation warnings MUST precede breaking changes by at least one MINOR version
- Version numbers MUST be updated in lockstep with changes

**Rationale**: Semantic versioning provides clear communication about the impact of updates, enabling consumers to make informed decisions about upgrades and manage their own dependencies safely.

## Development Workflow

All development in ChenChen follows a structured specification-first workflow:

1. **Specification** (`/speckit.specify`): Define feature requirements, user stories, and acceptance criteria
2. **Clarification** (`/speckit.clarify`): Identify and resolve underspecified areas through targeted questions
3. **Planning** (`/speckit.plan`): Research technical approach, design data models, define contracts
4. **Analysis** (`/speckit.analyze`): Verify consistency across specification, plan, and tasks
5. **Task Generation** (`/speckit.tasks`): Create dependency-ordered, executable task list
6. **Implementation** (`/speckit.implement`): Execute tasks following TDD red-green-refactor cycle
7. **Review**: Verify all acceptance criteria met, all tests passing, all principles satisfied

**Requirements**:
- Specifications MUST define user stories with priorities (P1, P2, P3)
- Each user story MUST be independently testable and deliverable
- Tasks MUST be organized by user story to enable incremental delivery
- MVP (Minimum Viable Product) MUST be deliverable after completing P1 user story
- All changes MUST go through plan.md Constitution Check section

## Quality Standards

### Testing Requirements

All code MUST have:
- **Contract Tests**: Verify public API contracts for all libraries
- **Integration Tests**: Verify interactions between components
- **Unit Tests**: Verify individual function behavior (when complexity warrants)

Testing focus areas requiring integration tests:
- New library contract tests
- Changes to existing contracts
- Inter-service communication
- Shared schema validation

### Code Review Requirements

All pull requests MUST:
- Pass all automated tests (zero failures)
- Verify compliance with all five Core Principles
- Include migration guide for breaking changes
- Update documentation for behavior changes
- Update CHANGELOG.md with user-facing changes

## Governance

This constitution supersedes all other development practices and guidelines for ChenChen.

**Amendment Process**:
1. Proposed amendments MUST be documented with rationale
2. Amendments MUST be reviewed and approved by project maintainers
3. Amendments requiring breaking changes MUST include migration plan
4. Amendments MUST update constitution version according to semantic versioning:
   - MAJOR: Principle removal, redefinition, or incompatible governance changes
   - MINOR: New principle addition or material expansion
   - PATCH: Clarifications, wording improvements, typo fixes

**Compliance**:
- All pull requests and code reviews MUST verify constitutional compliance
- Violations MUST be justified in plan.md Complexity Tracking table before approval
- Repeated unjustified violations warrant rejection

**Version**: 1.0.0 | **Ratified**: 2025-11-12 | **Last Amended**: 2025-11-12

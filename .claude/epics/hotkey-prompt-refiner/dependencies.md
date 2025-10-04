# Task Dependencies Graph

## Mermaid Diagram

```mermaid
graph TD
    %% Epic
    Epic["#1 Epic: Hotkey Prompt Refiner"]

    %% Sequential Tasks (solid boxes)
    T8["#8 Project Setup and Dependencies<br/>⏱️ in_progress<br/>📦 Sequential"]
    T9["#9 Global Hotkey System<br/>📦 Sequential"]
    T2["#2 Direct HTTP API Integration<br/>📦 Sequential"]
    T4["#4 Integrate Complete Workflow<br/>📦 Sequential"]
    T5["#5 Cross-platform Testing<br/>📦 Sequential"]

    %% Parallel Tasks (dashed boxes)
    T10["#10 Clipboard Integration<br/>⚡ Parallel"]
    T11["#11 Configuration System<br/>⚡ Parallel"]
    T3["#3 Auto-paste System<br/>⚡ Parallel"]
    T6["#6 Performance Optimization<br/>⚡ Parallel"]
    T7["#7 Documentation<br/>⚡ Parallel"]

    %% Epic to Foundation
    Epic --> T8

    %% Foundation dependencies (from T8)
    T8 --> T9
    T8 --> T10
    T8 --> T11
    T8 --> T3

    %% API Integration dependencies
    T8 --> T2
    T11 --> T2

    %% Workflow integration (all core components needed)
    T9 --> T4
    T10 --> T4
    T2 --> T4
    T3 --> T4

    %% Final phase dependencies
    T4 --> T5
    T4 --> T6
    T4 --> T7
    T5 --> T7

    %% Styling
    classDef sequential fill:#e1f5fe,stroke:#01579b,stroke-width:2px
    classDef parallel fill:#f3e5f5,stroke:#4a148c,stroke-width:2px,stroke-dasharray: 5 5
    classDef inProgress fill:#fff9c4,stroke:#f57f17,stroke-width:3px
    classDef epic fill:#c8e6c9,stroke:#1b5e20,stroke-width:3px

    class T8,T9,T2,T4,T5 sequential
    class T10,T11,T3,T6,T7 parallel
    class T8 inProgress
    class Epic epic
```

## Legend

- **🟦 Sequential Tasks (solid border)**: Must be completed before dependent tasks can start
- **🟪 Parallel Tasks (dashed border)**: Can be worked on simultaneously with other parallel tasks at the same level
- **🟨 In Progress**: Currently being worked on
- **🟩 Epic**: Root task that encompasses all subtasks

## Critical Path

The critical path (longest sequence of dependencies) is:
1. #8 Project Setup and Dependencies _(in progress)_
2. #9 Global Hotkey System OR #2 Direct HTTP API Integration
3. #4 Integrate Complete Workflow
4. #5 Cross-platform Testing and Refinement
5. #7 Documentation and User Guide

**Estimated Duration:** ~49-68 hours (1.5-2 weeks)

## Parallelization Opportunities

### Phase 1: Foundation (after #8 completes)
Can work in parallel:
- #9 Global Hotkey System (sequential)
- #10 Clipboard Integration (parallel)
- #11 Configuration System (parallel)
- #3 Auto-paste System (parallel)

### Phase 2: API Integration (after #8 + #11 complete)
- #2 Direct HTTP API Integration

### Phase 3: Integration (after all core components complete)
- #4 Integrate Complete Workflow

### Phase 4: Polish (after #4 completes)
Can work in parallel:
- #5 Cross-platform Testing (sequential, but feeds into #7)
- #6 Performance Optimization (parallel)
- #7 Documentation (parallel, needs #5 for platform notes)

## Dependency Matrix

| Task | Depends On | Can Run In Parallel |
|------|-----------|---------------------|
| #8 Project Setup | - | No (foundation) |
| #9 Global Hotkey | #8 | No |
| #10 Clipboard | #8 | Yes |
| #11 Configuration | #8 | Yes |
| #2 API Integration | #8, #11 | No |
| #3 Auto-paste | #8 | Yes |
| #4 Workflow Integration | #9, #10, #2, #3 | No |
| #5 Cross-platform Testing | #4 | No |
| #6 Performance Optimization | #4 | Yes |
| #7 Documentation | #4, #5 | Yes |

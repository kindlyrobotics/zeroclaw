# Spec to Code Task

Convert a project specification into working code.

## Steps

1. Parse the specification for:
   - Core requirements and features
   - Tech stack preferences
   - Constraints and non-goals
2. Create a task breakdown in memory (tag "project:tasks")
3. Set up project structure using shell/file tools
4. Implement each feature:
   - Delegate frontend to `frontend` agent if needed
   - Delegate backend to `backend` agent if needed
5. Delegate testing to `tester` agent
6. Package deliverables and send via channel
7. Update project status in memory

## Deliverable Format

- Complete source code in workspace
- README with setup instructions
- Summary of what was built
- Known limitations or TODOs

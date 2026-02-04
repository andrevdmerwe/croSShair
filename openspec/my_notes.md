Quick start after setup:
    /opsx:new      Create a change
    /opsx:continue Next artifact
    /opsx:apply    Implement tasks


    Core /opsx Commands
/opsx:explore: Used for brainstorming, investigating problems, and clarifying requirements before committing to a change.
/opsx:new [name]: Starts a new change, creating the directory structure (openspec/changes/<name>) and initializing the .openspec.yaml file.
/opsx:continue: Generates the next necessary artifact (e.g., proposal.md, design.md, tasks.md) based on what is ready in the workflow.
/opsx:ff: Fast-forwards by creating all planning artifacts (proposal, specs, design, tasks) at once. Ideal when the scope is clear.
/opsx:apply: Instructs the AI to implement the tasks defined in tasks.md.
/opsx:verify: Validates that the code implementation matches the defined specification artifacts.
/opsx:sync: Synchronizes delta specs from a change to the main project specs, ensuring documentation is up-to-date.
/opsx:archive: Archives the completed change folder, moving it to openspec/changes/archive/.
/opsx:bulk-archive: Archives multiple completed changes at once.
/opsx:onboard: Starts a guided 15-minute tutorial walkthrough of the full OpenSpec workflow. 
Usage Tips
Workflow: Start with /opsx:new, use /opsx:ff to build the plan, edit the generated tasks.md if necessary, then run /opsx:apply.
Correction: If the AI makes a mistake during /opsx:apply, you can manually update the .md files and ask it to continue.
AI Support: These commands are designed to work within IDE AI agents like Cursor, Windsurf, and Claude Code, provided OpenSpec is initialized (openspec init). 
Example Workflow
You: /opsx:new add-dark-mode
You: /opsx:ff (Creates all files: proposal.md, specs/, design.md, tasks.md)
You: /opsx:apply (AI reads tasks.md and writes the code)
You: /opsx:archive (Moves the files to archive and updates main specs). 
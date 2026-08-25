---
name: mme-implementation-planning
description: Use when Claude needs to plan MMD/MME .fx/.fxsub implementation details such as file layout, render targets, MMDPass techniques, postprocess scripts, DefaultEffect routing, controller parameters, examples, and debug checkpoints before writing code.
---

# MME Implementation Planning

## Overview

Turn an approved effect route into an implementable MME plan. The plan must be specific enough that `mme-authoring` can write files without guessing pass order or resource names.

## Plan Checklist

1. File layout:
   - Main `.fx`, helper `.fxsub`, textures, controller `.pmx/.x`, readme notes if needed.
2. Attachment model:
   - Which object/accessory loads the effect and which MME tab assignments are required.
3. Render targets:
   - Name, semantic, format, size, clear color/depth, antialias, mip levels, and `DefaultEffect`.
4. Technique/pass graph:
   - `ScriptClass`, `ScriptOrder`, technique `Script`, pass `Script`, `MMDPass` variants.
5. Data flow:
   - Inputs, intermediate buffers, final output, framework buffers.
6. Controls:
   - UI annotations, `CONTROLOBJECT` names/items, defaults and ranges.
7. Examples:
   - Local files to mirror or adapt.
8. Verification:
   - Static review items and runtime debug checkpoints.

## Output Template

```markdown
Implementation plan:
- Files:
- Load/assignment steps:
- Render targets:
- Techniques and passes:
- Controller/UI parameters:
- Example sources:
- Compatibility notes:
- Debug checkpoints:
```

## Required Reference Use

Before finalizing a plan touching MME scripts or target semantics, verify with:

- `../mmd-mme-workflow/references/mme-core-reference.md`

For framework work, also verify:

- `../mmd-mme-workflow/references/framework-notes.md`

For examples, use:

- `../mmd-mme-workflow/references/example-index.md`

## Planning Rules

- Put `Draw=Buffer` or `Draw=Geometry` on pass `Script`, not just in the technique script.
- Include all necessary `object` and `object_ss` variants for model shaders.
- Preserve `MMDSamp0..2` register declarations when adapting full shaders that rely on MMD samplers.
- Plan a temporary "draw intermediate buffer to screen" pass for complex post effects.
- For ray-mmd and sdPBR, prefer adding a material/preset/config-level extension over editing core buffers unless core editing is truly required.

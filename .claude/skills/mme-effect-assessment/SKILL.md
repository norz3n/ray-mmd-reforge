---
name: mme-effect-assessment
description: Use when Claude needs to interpret a user's desired MMD/MME visual effect, look, shader behavior, postprocess, material response, lighting change, shadow style, or ray-mmd/sdPBR/MME effect idea before feasibility or implementation.
---

# MME Effect Assessment

## Overview

Translate the user's visual request into a concrete MME effect brief. Stay visual and behavioral here; do not jump straight into HLSL.

## Assessment Steps

1. Identify the requested look: color, lighting, shadow, material, distortion, screen-space effect, controller behavior, or framework integration.
2. Classify the effect type: material/model shader, post effect, accessory/controller, offscreen target, or framework extension.
3. Name the target scope: specific model, material subset, accessory, whole scene, background, shadow map, G-buffer, or post chain.
4. Extract inputs: textures, depth, normals, shadow map, camera, light, controller morphs/bones, motion, time, model UVs, or framework buffers.
5. Define success criteria visible in MMD: what should be seen, when it should change, and what must remain unaffected.
6. List unknowns that block planning.

## Output Template

```markdown
Effect brief:
- Visual goal:
- Target scope:
- Effect type:
- Required inputs:
- User controls:
- Compatibility target:
- Acceptance criteria:
- Unknowns:
```

## Use Local References When Needed

If the user names a framework or known effect, read:

- `../mmd-mme-workflow/references/framework-notes.md`
- `../mmd-mme-workflow/references/example-index.md`

If the effect classification is unclear, read:

- `../mmd-mme-workflow/references/mme-core-reference.md`

## Assessment Heuristics

- "Make material look like skin/metal/glass/wet cloth" usually means material shader or framework material preset.
- "Make the whole screen bloom/blur/warp/fog" usually means postprocess.
- "Use model position/morphs/bones as knobs" means controller or `CONTROLOBJECT`.
- "Needs depth/normal/shadow information" likely needs offscreen targets or a framework buffer.
- "Works with ray/sdPBR" is not enough; decide whether it is a native framework extension or an external effect that must coexist.

## Common Mistakes

- Treating a framework material request as standalone MME.
- Ignoring draw-order and MME tab assignment during assessment.
- Failing to ask whether the output targets MMD+MME, MMM, ray-mmd, sdPBR, or vanilla MME.
- Accepting vague words like "cinematic" without translating them into shader-observable changes.

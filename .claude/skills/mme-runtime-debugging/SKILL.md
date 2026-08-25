---
name: mme-runtime-debugging
description: Use when Claude needs to diagnose MMD/MME runtime failures or visual bugs such as black screens, blank output, compile errors, some techniques cannot run on this hardware, missing textures/includes, wrong draw order, ray-mmd or sdPBR setup problems, ExcellentShadow issues, or broken controller parameters.
---

# MME Runtime Debugging

## Overview

Diagnose runtime behavior from symptoms and local framework rules. Change one variable at a time and prefer minimal reproduction scenes.

## Debug Flow

1. Capture exact symptom:
   - error text, black screen, blank buffer, wrong object, missing shadow, flicker, crash, extreme slowness.
2. Identify environment:
   - MMD+MME or MMM, GPU/VRAM if known, output size, framework loaded.
3. Verify load/assignment:
   - effects attached to correct MME tabs, controllers loaded, draw order, self-shadow state.
4. Reduce:
   - minimal model/stage, default material, no optional post effects.
5. Isolate pipeline:
   - draw intermediate target to screen, bypass blur, bypass framework extras.
6. Fix the smallest proven cause.
7. Re-test the original scene.

## Symptom Map

- `some techniques cannot run on this hardware`: shader model, target format, instruction count, sampler limit, or GPU float texture support.
- Black/blank post effect: missing `ScriptExternal=Color`, wrong `Draw=Buffer`, wrong final target reset, uninitialized RT, bad full-screen VS/UV.
- Object unaffected: wrong MME tab, missing `object_ss`, wrong `UseTexture`/`UseSphereMap`/`UseToon`, technique selection mismatch.
- Missing framework output: missing controller/accessory, wrong draw order, missing generated config/resource, stale MME cache.
- Crash/very slow: VRAM pressure from high-resolution RTs, Omni/volume lights, large G-buffer, MSAA, high shadow cascades, output size.

## Framework Checks

ray-mmd:

- Confirm `ray.x`, `ray_controller.pmx`, materials, lights/fog/skybox assets, and required textures exist.
- Check `ray.conf` feature switches and refresh all after edits.

ExcellentShadow2:

- Confirm `ExcellentShadow.x` is loaded near subjects.
- Confirm all models/accessories use `full_ES.fx` or `full_ES_pmx.fx`.
- Confirm self-shadow is enabled.
- Lower `EXSHADOW_QUALITY` if performance or format support is suspect.

sdPBR480:

- Confirm `sdPBR.pmx` and `sdPBRGBuffer.x` are loaded.
- Put `sdPBRGBuffer.x` early/high in draw order.
- Follow quick-start order if output is black.
- Increase preview panel size if very small; reduce extra lights and volume/Omni features under VRAM pressure.

## Output Format

```markdown
Diagnosis:
- Symptom:
- Most likely cause:
- Evidence:
- Next test:
- Fix:
- Remaining risks:
```

## References

Always read when the symptom is not trivial:

- `../mmd-mme-workflow/references/debug-checklist.md`

Read as needed:

- `../mmd-mme-workflow/references/framework-notes.md`
- `../mmd-mme-workflow/references/mme-core-reference.md`

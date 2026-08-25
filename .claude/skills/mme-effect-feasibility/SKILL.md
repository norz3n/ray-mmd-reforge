---
name: mme-effect-feasibility
description: Use when Claude must decide whether an MMD/MME visual effect can be implemented, whether it should be standalone or framework-based, and what shader model, render target, controller, hardware, or compatibility risks apply.
---

# MME Effect Feasibility

## Overview

Decide whether the assessed MME effect is practical before planning files and passes. Prefer a boring workable route over a clever fragile route.

## Feasibility Checks

1. Choose implementation family:
   - Standalone material shader
   - Standalone post effect
   - Accessory/controller effect
   - Offscreen render target helper
   - ray-mmd material/config/postprocess extension
   - sdPBR material/light/postprocess/G-buffer extension
   - ExcellentShadow-compatible shader adaptation
2. Check required data availability:
   - color buffer, depth, normals, shadow map, material textures, sphere/toon maps, time, camera/light matrices, controller values, framework shared buffers.
3. Check render cost:
   - number of render targets, target formats, blur loops, shadow-map resolution, extra geometry passes, and Shader Model 3.0 dependency.
4. Check MMD/MME compatibility:
   - MMD vs MMM, self-shadow, alpha materials, model/accessory assignment, draw order, hardware float texture support.
5. Name risks and fallback options.

## Framework Decision Rules

- ray-mmd: use native material/config paths when the effect changes PBR material response, IBL, lighting, fog, skybox, SSR, SSDO, outline, or tone mapping.
- sdPBR: use native material/config/light paths when the effect needs sdPBR G-buffer, additional lights, volume lights, SSDO, PBR materials, or sdPBR post chain.
- ExcellentShadow: adapt `full.fx`-derived model shaders with the ExcellentShadow system sections when the goal is to keep ExcellentShadow shadows with a custom object shader.
- Standalone: use only when the effect can operate from standard MME inputs without fighting framework render order.

## Output Template

```markdown
Feasibility:
- Recommended route:
- Why:
- Required data/resources:
- Shader model / RT requirements:
- Controller or asset requirements:
- Compatibility risks:
- Fallback route:
- Decision:
```

## References

Load as needed:

- `../mmd-mme-workflow/references/mme-core-reference.md`
- `../mmd-mme-workflow/references/framework-notes.md`
- `../mmd-mme-workflow/references/debug-checklist.md`

## Red Flags

- Requires arbitrary scene normals but no normal/G-buffer source exists.
- Requires multi-bounce/global illumination in vanilla MME without approximations.
- Requires many high-precision full-resolution buffers on older DirectX9 hardware.
- Requires changing ray-mmd or sdPBR internals without respecting their config and shared target layout.
- Requires reliable shadows from semi-transparent objects.

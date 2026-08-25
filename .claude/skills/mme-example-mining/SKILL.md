---
name: mme-example-mining
description: Use when Claude should inspect local MMD/MME effect examples, official samples, ray-mmd, sdPBR, ExcellentShadow, or existing .fx/.fxsub/readme files to find implementation patterns before planning, authoring, reviewing, or debugging.
---

# MME Example Mining

## Overview

Mine local examples for patterns that can be reused safely. MME behavior is full of host-specific conventions; an existing working effect is often the best spec.

## Mining Steps

1. Identify the closest effect family:
   - full model shader, postprocess, mirror/offscreen, shadow, DOF/bloom, face shadow, PBR material, framework integration.
2. Use `rg --files` and `rg -n` to find candidate `.fx`, `.fxsub`, readme, config, controller, and texture files.
3. Read both shader and usage notes. Many effects depend on MME tab assignment and accessory parameters.
4. Extract reusable structure:
   - render targets, script order, passes, technique variants, controller names, formats, fallback behavior.
5. Identify non-reusable assumptions:
   - hard-coded object names, framework buffer layouts, paths, language-specific control names, hardware assumptions.

## Output Template

```markdown
Example mining:
- Candidate examples:
- Closest match:
- Pattern to reuse:
- Dependencies:
- Differences from our target:
- Risks:
```

## Reference Index

Start from:

- `../mmd-mme-workflow/references/example-index.md`
- `../mmd-mme-workflow/references/framework-notes.md`

## Useful Search Patterns

```powershell
rg -n "STANDARDSGLOBAL|ScriptClass|ScriptOrder|string Script|Draw=Buffer|Draw=Geometry|MMDPass" <dir>
rg -n "OFFSCREENRENDERTARGET|DefaultEffect|RENDERCOLORTARGET|RENDERDEPTHSTENCILTARGET" <dir>
rg -n "CONTROLOBJECT|UseTexture|UseSphereMap|UseToon|Subset" <dir>
rg -n "ps_3_0|vs_3_0|R32F|A16B16G16R16F|A32B32G32R32F" <dir>
```

## Framework Mining Notes

- ray-mmd: start with `ray.conf`, `Shader/textures.fxsub`, `Shader/gbuffer.fxsub`, then the relevant material/light/fog/post folder.
- ExcellentShadow2: read `Readme.txt`, `ExcellentShadow.fx`, `ExcellentShadowCommonSystem.fx`, `full_ES.fx`, and the relevant `fxsub`.
- sdPBR480: read `うまく動かない時は.txt`, migration guide, `sdPBR.fx`, `sdPBRGBuffer.fx`, config, material head/tail, and matching material/light examples.

## Common Mistakes

- Copying a pass graph without its `DefaultEffect` render target.
- Copying a material shader without all `UseTexture`/`UseSphereMap`/`UseToon` variants.
- Using framework internal buffers without checking version-specific layout.
- Missing controller `.pmx/.x` assets and morph names.

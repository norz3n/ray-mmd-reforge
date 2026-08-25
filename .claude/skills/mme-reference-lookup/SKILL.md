---
name: mme-reference-lookup
description: Use when Claude needs to look up local MMD/MME documentation, MME syntax, HLSL semantics, annotations, Script commands, STANDARDSGLOBAL, CONTROLOBJECT, DefaultEffect, MMDPass, render target parameters, or framework-specific reference files.
---

# MME Reference Lookup

## Overview

Find and quote or summarize the local source of truth for MME syntax and framework behavior. Do not rely on memory when exact annotations or script semantics matter.

## Search Order

No official MME REFERENCE.txt is available locally. When exact official syntax matters and the suite references do not cover it, use web search (the official MME sample docs are mirrored online) instead of guessing.

1. Suite shared references:
   - `../mmd-mme-workflow/references/mme-core-reference.md`
   - `../mmd-mme-workflow/references/framework-notes.md`
2. This repository (ray-mmd ReForge) — the primary working framework:
   - `ray.fx`, `ray.conf`, `ray_advanced.conf`
   - `Shader/textures.fxsub`, `Shader/gbuffer.fxsub`, other `Shader/*.fxsub`
   - `Materials/material_2.0.fx`, `Materials/material_common_2.0.fxsub`, per-family folders under `Materials/` (Skin, Hair, Metallic, ...)
3. Local effect sources under `ref-code/` — real-world MME code to mine for conventions:
   - `ref-code/ray-mmd/`, `ref-code/ray-mmd-dev/` — stock ray-mmd 1.5.2 for comparison with ReForge
   - `ref-code/sdPBR480/` — sdPBR framework
   - `ref-code/MES40-3.1/`, `ref-code/ikPolishShader_v028/`, `ref-code/WorkingFloorX_v006/`, `ref-code/dendewaLAB-*` and others — standalone effect examples

## Encoding Rules

- Try UTF-8 first for modern Chinese docs and some newer Japanese files.
- Try CP932/Shift-JIS for many Japanese readmes and legacy `.fx` files.
- If text is garbled, do not infer meaning until reread with another encoding.

PowerShell snippets:

```powershell
$t=[Text.Encoding]::UTF8.GetString([IO.File]::ReadAllBytes($p))
$t=[Text.Encoding]::GetEncoding(932).GetString([IO.File]::ReadAllBytes($p))
```

## Lookup Output

```markdown
Reference lookup:
- Question:
- Source files checked:
- Relevant rule:
- Exact implication for this effect:
- Uncertainty:
```

Keep quotations short. Prefer paraphrase plus file path and line number when available.

## Common Targets

- `STANDARDSGLOBAL`, `ScriptClass`, `ScriptOrder`, `ScriptOutput`
- `RenderColorTarget[n]`, `RenderDepthStencilTarget`, `ClearSetColor`, `ClearSetDepth`, `Clear`, `ScriptExternal`, `Pass`, `LoopByCount`, `LoopEnd`
- pass `Script` with `Draw=Buffer` or `Draw=Geometry`
- `MMDPass`, `UseTexture`, `UseSphereMap`, `UseToon`, `Subset`
- `OFFSCREENRENDERTARGET`, `RENDERCOLORTARGET`, `RENDERDEPTHSTENCILTARGET`
- `CONTROLOBJECT`, `(self)`, `(OffscreenOwner)`
- material texture semantics and MMD sampler preservation

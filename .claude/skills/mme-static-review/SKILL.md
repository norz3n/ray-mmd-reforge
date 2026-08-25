---
name: mme-static-review
description: Use when Claude needs to review MMD/MME .fx/.fxsub shader code before runtime testing, looking for HLSL compile issues, MME Script mistakes, missing passes, wrong MMDPass variants, render target problems, includes, encoding, or framework integration errors.
---

# MME Static Review

## Overview

Review MME code as both HLSL and MME pipeline configuration. Findings should lead, with file and line references whenever possible.

## Review Checklist

MME script:

- `STANDARDSGLOBAL` has correct `ScriptClass`, `ScriptOrder`, and `ScriptOutput`.
- Technique `Script` routes render targets and passes in the intended order.
- Pass-level `Script` contains `Draw=Buffer` or `Draw=Geometry` where required.
- Final screen draw restores empty color/depth targets when needed.
- `ScriptExternal=Color` appears before sampling scene color.
- Loop counters and `LoopEnd` are paired.

Targets/resources:

- Render target names match samplers and script strings exactly.
- Formats are supported by intended hardware.
- Clear values make sense.
- `DefaultEffect` routes include required wildcards and hide controllers.
- Include/resource paths are relative to the effect file.

Model shader:

- Required `object`, `object_ss`, `shadow`, `edge`, `zplot` techniques exist or intentionally no-op.
- `UseTexture`, `UseSphereMap`, `UseToon`, and `Subset` variants cover target materials.
- MMD samplers are not overwritten accidentally.
- Alpha and transparent materials are handled intentionally.

Framework:

- ray-mmd/sdPBR buffers and G-buffer layouts match the local version.
- ExcellentShadow adaptations include the required system sections and fallback path.
- Config `.fxsub` changes are documented as requiring MME refresh.

## Output Format

```markdown
Findings:
- [Severity] file:line - Issue and impact.

Open questions:
- ...

Review notes:
- ...
```

If no issues are found, say so clearly and list remaining runtime risks.

## Useful Commands

```powershell
rg -n "TODO|FIXME|STANDARDSGLOBAL|ScriptClass|ScriptOrder|string Script|Draw=|MMDPass|OFFSCREENRENDERTARGET|RENDERCOLORTARGET|CONTROLOBJECT" <file-or-dir>
rg -n "RenderColorTarget|RenderDepthStencilTarget|Pass=|LoopByCount|LoopEnd|ScriptExternal|Clear=" <file-or-dir>
```

## References

- `../mmd-mme-workflow/references/mme-core-reference.md`
- `../mmd-mme-workflow/references/debug-checklist.md`
- `../mmd-mme-workflow/references/framework-notes.md`

---
name: mme-authoring
description: Use when Claude needs to write or modify MMD/MME shader files such as .fx or .fxsub, including model shaders, post effects, render-target chains, controller-driven effects, or ray-mmd/sdPBR/ExcellentShadow-compatible effect code.
---

# MME Authoring

## Overview

Write MME shader code from a concrete plan. Follow local examples and keep the first implementation debuggable.

## Before Writing

Confirm the plan includes:

- File paths and include paths.
- Attachment/load instructions.
- Render target definitions.
- Technique/pass graph.
- Controller/UI parameters.
- Shader model target.
- Example source files.

If any of these are missing for a nontrivial effect, return to `mme-implementation-planning`.

## Authoring Rules

- Preserve existing framework layout and include conventions.
- Use ASCII filenames for new effect files when possible.
- Keep comments concise and useful; explain pass purpose, not obvious assignments.
- Preserve `MMDSamp0`, `MMDSamp1`, `MMDSamp2` declarations when adapting full shaders.
- Put `Draw=Buffer` or `Draw=Geometry` on pass `Script`.
- Use `vs_2_0`/`ps_2_0` for simple model passes; use `vs_3_0`/`ps_3_0` for complex postprocess, loops, SSDO/SSAO, or framework parity.
- Add temporary debug output passes for complex intermediate buffers if runtime testing is expected.
- For model shaders, include all needed `MMDPass` variants and material resource variants.
- For framework work, prefer extension points and preset patterns over core rewrites.

## Minimal Postprocess Shape

```hlsl
float Script : STANDARDSGLOBAL <
    string ScriptOutput = "color";
    string ScriptClass = "scene";
    string ScriptOrder = "postprocess";
> = 0.8;

texture2D ScnMap : RENDERCOLORTARGET < float2 ViewPortRatio = {1,1}; >;
texture2D DepthBuffer : RENDERDEPTHSTENCILTARGET < string Format = "D24S8"; >;

technique Main <
    string Script =
        "RenderColorTarget0=ScnMap;"
        "RenderDepthStencilTarget=DepthBuffer;"
        "Clear=Color;"
        "Clear=Depth;"
        "ScriptExternal=Color;"
        "RenderColorTarget0=;"
        "RenderDepthStencilTarget=;"
        "Pass=DrawFinal;";
> {
    pass DrawFinal < string Script="Draw=Buffer;"; > {
        VertexShader = compile vs_3_0 VS();
        PixelShader  = compile ps_3_0 PS();
    }
}
```

Adapt from official examples rather than using this as a full template for production.

## After Writing

Immediately use `mme-static-review`. Do not claim the shader is correct until it has been reviewed and, when possible, tested in MMD/MME.

## References

- `../mmd-mme-workflow/references/mme-core-reference.md`
- `../mmd-mme-workflow/references/example-index.md`
- `../mmd-mme-workflow/references/framework-notes.md`

# MME Core Reference

## Authoritative Sources

No official MME REFERENCE.txt is stored locally. When exact official syntax is disputed or missing here, verify by web search (the official MME docs are mirrored online) and by reading working `.fx` code under `ref-code/`. This file is a condensed summary — treat real effect sources as ground truth over any summary, including this one.

## Effect Types

- Model/material shader: handles `object`, `object_ss`, `shadow`, `edge`, `zplot` techniques.
- Post effect: usually `STANDARDSGLOBAL` with `ScriptClass="scene"`, `ScriptOrder="postprocess"`, and buffer drawing passes.
- Preprocess effect: runs before main scene drawing, often to clear or prepare G-buffers.
- Accessory/controller effect: `.x` or `.pmx` object plus `CONTROLOBJECT` parameters.
- Offscreen render target effect: creates `OFFSCREENRENDERTARGET` with `DefaultEffect`.
- Framework effect: ray-mmd, sdPBR, ExcellentShadow, M4Layer-like systems with shared targets and strict load/assignment rules.

## Technique Matching

MME selects the first valid matching technique. If no matching technique can run, MMD falls back to the standard shader.

Common technique annotations:

- `string MMDPass = "object"`: normal object drawing.
- `string MMDPass = "object_ss"`: object drawing with self-shadow enabled.
- `string MMDPass = "shadow"`: ground shadow pass.
- `string MMDPass = "edge"`: edge outline pass.
- `string MMDPass = "zplot"`: MMD self-shadow depth plot.
- `bool UseTexture`, `bool UseSphereMap`, `bool UseToon`: match material resource usage.
- `int Subset`: target material subset.

Provide all relevant technique variants when replacing full-model shaders; otherwise some materials silently fall back or render without the intended path.

## Script Semantics

`STANDARDSGLOBAL` declares the effect class and order:

```hlsl
float Script : STANDARDSGLOBAL <
    string ScriptOutput = "color";
    string ScriptClass = "scene";
    string ScriptOrder = "postprocess";
> = 0.8;
```

Technique `Script` strings orchestrate render steps:

- `RenderColorTarget0=TargetName;`
- `RenderDepthStencilTarget=DepthBuffer;`
- `ClearSetColor=ClearColor;`
- `ClearSetDepth=ClearDepth;`
- `Clear=Color;`
- `Clear=Depth;`
- `ScriptExternal=Color;`
- `Pass=PassName;`
- `LoopByCount=Counter;`
- `LoopEnd=;`

Pass `Script` annotations specify the draw source:

```hlsl
pass BlurX < string Script = "Draw=Buffer;"; > { ... }
pass DrawObjects < string Script = "Draw=Geometry;"; > { ... }
```

Do not put `Draw=Buffer` or `Draw=Geometry` only in a technique script; official examples put them on passes.

## Render Targets

Common semantics:

- `RENDERCOLORTARGET`: scene color target or named pass target.
- `RENDERDEPTHSTENCILTARGET`: depth/stencil target, commonly `D24S8`.
- `OFFSCREENRENDERTARGET`: target rendered through `DefaultEffect` before it is sampled.
- `MATERIALTEXTURE`, `MATERIALSPHEREMAP`, `MATERIALTOONTEXTURE`: MMD material textures.

Typical annotations include:

- `string Format = "D3DFMT_A16B16G16R16F";`
- `float2 ViewPortRatio = {1.0, 1.0};`
- `int Width`, `int Height`
- `bool AntiAlias`
- `string DefaultEffect = "self=hide;* = some.fxsub;";`

Float formats and high-resolution targets are common failure points on older hardware.

## Controller Access

Use `CONTROLOBJECT` for controller PMX/X/accessory parameters:

```hlsl
float value : CONTROLOBJECT < string name = "Controller.pmx"; string item = "MorphName"; >;
float4x4 mat : CONTROLOBJECT < string name = "(self)"; >;
```

Special names:

- `(self)`: the current object/accessory.
- `(OffscreenOwner)`: object that owns the offscreen target.

Control names are encoding-sensitive. Prefer ASCII controller names when designing new assets, and match Japanese/Chinese names exactly when integrating existing frameworks.

## Shader Model and Encoding

- MME is DirectX9/HLSL; practical ceiling is Shader Model 3.0.
- Prefer `vs_2_0`/`ps_2_0` unless the pass needs loops, many instructions, or framework parity.
- Many Japanese readmes and `.fx` files are Shift-JIS/CP932; many newer docs are UTF-8.
- If text is garbled, inspect BOM/bytes and retry UTF-8 or CP932 before drawing conclusions.
- Non-ASCII paths can work but are fragile across locale and tooling. Prefer ASCII effect paths for new work.

PowerShell CP932 read pattern:

```powershell
$t=[Text.Encoding]::GetEncoding(932).GetString([IO.File]::ReadAllBytes($p))
```

UTF-8 read pattern:

```powershell
$t=[Text.Encoding]::UTF8.GetString([IO.File]::ReadAllBytes($p))
```

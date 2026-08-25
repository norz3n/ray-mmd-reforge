# MME Example Index

All paths are relative to the repository root. Use `rg --files` first, then read only the files relevant to the effect being designed. No official MME sample effects are stored locally; for baseline `object`/`object_ss`/`shadow`/`edge`/`zplot` and `Draw=Buffer` patterns, mine `ref-code/ray-mmd/Materials/material_2.0.fx` or any standalone effect below.

## Frameworks

- ray-mmd ReForge (this repository):
  - `ray.fx`, `ray.conf`, `ray_advanced.conf`
  - `Shader/textures.fxsub`, `Shader/gbuffer.fxsub`
  - `Materials/material_2.0.fx`, `Materials/material_common_2.0.fxsub`
- Stock ray-mmd 1.5.2 (baseline for comparison):
  - `ref-code/ray-mmd/` — same layout as above
  - `ref-code/ray-mmd-dev/` — development variant
- ExcellentShadow2:
  - `ref-code/N3+ C Shader v005 - Translated controls + extra normal maps 2021/ExcellentShadow2/`
  - `ExcellentShadow.fx`, `ExcellentShadowCommonSystem.fx`, `ExcellentShadowObject.fxsub`, `ExcellentShadowZBufDraw.fxsub`, `ExcellentShadowZBufDrawFar.fxsub`, `_Readme.txt`
- sdPBR480:
  - `ref-code/sdPBR480/sdPBR.fx`, `ref-code/sdPBR480/shader/sdPBRGBuffer.fxsub`
  - `shader/sdPBRMaterialHead.fxsub`, `shader/sdPBRMaterialTail.fxsub`, `preset/`, `lighting/`, `material/`

## Standalone Effects

- `ref-code/KrSS_Heavy2/` — full deferred pipeline with MRT G-buffer, shadow maps, sky (`MainSystem.fx`, `MRT/`, `ShadowMap/`, `Post/`)
- `ref-code/MES40-3.1/` — light controllers (`MES40 SpotLight.fxsub`, `MES40 PointLight.fxsub`) and sync helpers
- `ref-code/ikPolishShader_v028/Sources/` — material/toon shader family
- `ref-code/ikVXGI_v002a/Sources/` — VXGI experiment
- `ref-code/WorkingFloorX_v006/WorkingFloorX.fx` — reflective floor accessory pattern
- `ref-code/o_DLAA_v0_1/o_DLAA.fx` — postprocess AA
- `ref-code/dendewaLAB - HBAO Horizon Based Ambient Occlusion V3.0/` — standalone HBAO post effect
- `ref-code/なんちゃってGI改/` — lightweight GI post effect
- `ref-code/nlWARPv1_50.conf` — screen warp config example

## Algorithm References (non-MME)

- `ref-code/smaa/SMAA.hlsl` — SMAA HLSL port
- `ref-code/AgX/agx.glsl` — AgX tonemapping
- `ref-code/h3r2tic/` — rendering technique references
- `ref-docs/` — papers/notes used by ReForge (PCSS, SSSS, Hi-Z SSR, pre-integrated skin)

## Mining Heuristics

- Use this repository first when the task targets ray-mmd; use `ref-code/ray-mmd/` only to diff against stock behavior.
- Use modern examples for practical parameter naming, UI annotations, and compatibility workarounds.
- Use framework examples only after deciding the task belongs inside that framework.
- Compare both the `.fx` and any readme/config/controller assets; many MME effects only make sense as a loaded object plus shader plus tab assignment.

# Framework Notes

## ray-mmd ReForge (this repository)

This repo is a ReForge of stock ray-mmd (1.5.2 baseline) that folds community external `.fx` effects into one pipeline. Current version is tracked in `README.md` (v1.14.0 at time of writing). Stock ray-mmd sources for comparison live in `ref-code/ray-mmd/` and `ref-code/ray-mmd-dev/`; when notes here disagree with the code, trust the code.

Treat ray-mmd as a rendering framework, not a single shader. First classify the request:

- ray material preset or material edit
- ray controller/config tuning
- ray lighting/fog/skybox extension
- ray postprocess extension
- standalone MME that merely coexists with ray

Important files:

- `ray.fx`: main entry.
- `ray.conf`: compile-time feature and quality switches.
- `ray_advanced.conf`: numeric constants for lighting, exposure, PSSM, SSR, SSDO, AA, etc.
- `Shader/textures.fxsub`: central shared render target and `DefaultEffect` routing.
- `Shader/gbuffer.fxsub`: material/G-buffer packing, shading model IDs, decode helpers.
- `Materials/material_2.0.fx`: macro-configured material front end.
- `Materials/material_common_2.0.fxsub`: shared material implementation and compile-time guards.

Key `ray.conf` switches include `SUN_LIGHT_ENABLE`, `SUN_SHADOW_QUALITY`, `CONTACT_SHADOW_QUALITY`, `IBL_QUALITY`, `FOG_ENABLE`, `MULTI_LIGHT_ENABLE`, `OUTLINE_QUALITY`, `TOON_ENABLE`, `SSAO_TYPE`, `SSDO_QUALITY`, `SSR_QUALITY`, `GLASS_REFRACTION`, `GI_ENABLE`/`GI_QUALITY` (SSGI), `SSSS_QUALITY`, `BOKEH_MODE`, `HDR_EYE_ADAPTATION`, `HDR_BLOOM_MODE`, `HDR_FLARE_MODE`, `HDR_STAR_MODE`, `HDR_TONEMAP_OPERATOR`, `AA_QUALITY`, `POST_DISPERSION_MODE`, `POST_SHARPEN_*`, and `FILM_GRAIN_*`. Numeric tuning constants live in `ray_advanced.conf`.

`Shader/textures.fxsub` defines shared resources such as `DepthBuffer`, `ScnMap`, `FogMap`, `LightMap`, `LightSpecMap`, `LightAlphaMap`, `EnvLightMap`, `EnvLightAlphaMap`, `MaterialMap`, `OutlineMap`, `SSAOMap`, `SSRMap`, `SSGIVisibilityMap`, `SSDOMap`, `PSSM1..4`, `ShadowMap`, `Gbuffer2RT..8RT`, and `ShadingMap`. `MaterialMap` routes PMD/PMX objects to `./materials/material_2.0.fx`; controllers, skyboxes, fog, and lights are routed separately or hidden.

ReForge-specific structure beyond stock ray-mmd:

- `Materials/` is split into per-family folders (`Skin`, `Hair`, `Metallic`, `Cloth`, `ClearCoat`, `Eye`, `Emissive`, `Foliage`, `Subsurface`, `Transparent`, `Video`, `Programmable/Water|Wetness`) plus the shared `material_2.0.fx` front end.
- `Shader/` holds post-process modules per effect (`PostProcessBloom.fxsub`, `PostProcessSSGI.fxsub`, `PostProcessOcclusion*.fxsub`, `ContactShadow.fxsub`, `PreIntegratedSkin.fxsub`, `ColorGrading.fxsub`, ...).
- Skin look baseline: SSSS defaults 0.04/0.02 with `SKIN_AO_STRENGTH 0.75` — treat as the approved artistic baseline when touching skin materials.

Common ray risks:

- Editing a material without respecting the G-buffer packing.
- Enabling expensive features without checking GPU/VRAM.
- Missing `.fxsub`, LUT, texture, IBL, or controller assets.
- Forgetting that config changes may require MME "refresh all".
- Using a standalone post effect that fights ray tone mapping, depth, or render order.

## ExcellentShadow2

ExcellentShadow2 is a controller-plus-shader-plus-postprocess shadow framework.

Usage from the readme:

1. Load `ExcellentShadow.x`.
2. Apply `full_ES.fx` to all models/accessories.
3. Enable self-shadow on models/accessories.

`ExcellentShadow.x` position is the shadow-map center. `Si` controls shadow-map size. Larger size covers a wider range but loses fine shadow detail. `Tr` controls shadow density; smaller `Tr` makes shadows darker. Accessory X rotation adjusts blur strength.

Requirements and limits:

- MikuMikuEffect 0.27+ or MikuMikuMoving 1.1.8.2+.
- Shader Model 3.0 and floating-point texture mipmap support.
- Recommended VRAM 1GB+.
- PMD models receive weaker default shadow via `PMD_SHADOWPOWER`.
- Use `full_ES_pmx.fx` for PMX material morph support in MMD+MME.
- Not compatible with AbsoluteShadow.
- Semi-transparent object shadows are inherently unreliable.

Important files:

- `ExcellentShadow.fx`: scene postprocess that creates/filters `ScreenShadowMapProcessed`.
- `ExcellentShadowCommonSystem.fx`: quality, MMD shadow use, multisampling, alpha threshold, matrices.
- `ExcellentShadowObject.fxsub`: object rendering into the screen shadow map.
- `ExcellentShadowZBufDraw.fxsub` and `ExcellentShadowZBufDrawFar.fxsub`: internal depth maps.
- `full_ES.fx`: full.fx-derived model shader with ExcellentShadow integration.
- `ExShadowSSAO/ExShadowSSAO.fx`: SSAO exporter that feeds `ExShadowSSAOMapOut`; not useful standalone.

Implementation pattern:

- `ExcellentShadow.fx` declares `ScriptClass="scene"` and `ScriptOrder="postprocess"`.
- It uses `OFFSCREENRENDERTARGET` `ScreenShadowMap`, `ExcellentShadowZMap`, and `ExcellentShadowZMapFar` with `DefaultEffect` routes.
- It uses ping-pong `RENDERCOLORTARGET` buffers and `Pass=...` technique script steps.
- Its buffer passes use pass annotations like `string Script="Draw=Buffer;"`.
- `full_ES.fx` samples `ScreenShadowMapProcessed` and optionally `ExShadowSSAOMapOut`.

When adapting another `full.fx`-based shader, copy the ExcellentShadow system sections from `full_ES.fx` and keep fallback behavior for when `ExcellentShadow.x` is absent.

## sdPBR480

sdPBR is a large PBR/deferred rendering framework. Do not treat it as a single material shader.

Basic load-order note from migration docs:

- Load `sdPBR.pmx`.
- Also load `sdPBRGBuffer.x`.
- Put `sdPBRGBuffer.x` draw order as high/early as practical.

Key files:

- `sdPBR.fx`: scene postprocess for directional shadow map, shared depth, SSDO/SSAO-like occlusion, volume light, final composition, and AutoLuminous integration.
- `sdPBRGBuffer.fx`: `ScriptOrder="preprocess"` entry that clears `sdPBR_GBuffer0..3` and optional alpha G-buffers.
- `shader/sdPBRconfig.fxsub`: generated/edited macro configuration.
- `shader/sdPBRconfig解説.fxsub`: annotated configuration guide.
- `shader/sdPBRCommon.fxsub`: common constants and texture source macros.
- `shader/sdPBRMaterialHead.fxsub` and `shader/sdPBRMaterialTail.fxsub`: material wrapper.
- `material/body/sdPBR_skin.fx`: compact material example.

Material pattern:

```hlsl
#define SDPBR_MATERIAL_VER 100
#include "../../shader/sdPBRMaterialHead.fxsub"

void SetMaterialParam(inout Material m, float3 n, float3 l, float3 Eye, float2 uv)
{
    m.subsurface = 0.5;
    m.specular = 0.35;
    m.roughness = 0.65;
}

#define NORMAL_FROM NORMAL_FROM_FILE
#define NORMAL_FILE "../../texture/Skin_Human_002_NRM.png"

#include "../../shader/sdPBRMaterialTail.fxsub"
```

G-buffer layout changes between versions; verify `sdPBRGBuffer.fx` before writing external consumers. Version 4 notes show `GB0` depth+normal, `GB1` base/SSS color, `GB2` alpha/material/light visibility/specular/roughness, and `GB3` anisotropy/iridescence/clearcoat/toon flags.

Debug notes from local docs:

- If `sdPBRconfig.exe` UI language/resources fail on Japanese Windows, unblock `ja/sdPBRconfig.resources.dll`.
- If `sdPBR.pmx` loads with errors, check readme caution and environment first.
- If it loads but renders black, follow quick-start order and make the main preview panel large enough; GeForce may require roughly 256x256 or larger.
- Export crashes or extreme slowness are usually VRAM pressure. Extra lights, especially Omni/volume variants, are expensive. ScreenSkybox can crash when output resolution exceeds monitor resolution.

Light families:

- SpotLight: standard, IES, screen-textured, volume, smoke, rainbow variants.
- RectLight: area light, slow loading; does not reflect every advanced material parameter.
- OmniLight: omnidirectional shadow map, high VRAM cost.
- LiteLight: no shadow map, lighter.
- AreaLight: Disk/Quad/Tube approximations, often lighter than RectLight.
- ProceduralIES: generated caustics/grid style profiles.

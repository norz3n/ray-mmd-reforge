# MME Debug Checklist

## First Questions

- Which host is used: MMD+MME or MikuMikuMoving?
- Exact error text from MME, if any.
- Which object has which `.fx` assigned in the MME tabs?
- Which accessories/controllers are loaded, and in what draw order?
- Does the effect require self-shadow, `sdPBRGBuffer.x`, `ray_controller.pmx`, `ExcellentShadow.x`, or another controller asset?
- Does the project path contain non-ASCII characters or very long paths?

## Compile or Load Errors

Check:

- Missing `#include` `.fxsub` paths relative to the effect file.
- Missing texture/LUT/resource files.
- Wrong encoding for Japanese/Chinese control names or comments.
- Shader model too low/high for the hardware; SM3.0 is often required by advanced effects.
- Unsupported render target format such as `A32B32G32R32F`, `A16B16G16R16F`, `R32F`, or mipmapped float textures.
- Too many instructions/samplers/registers for `ps_2_0`.
- Stale generated settings; run MME "refresh all" after changing config `.fxsub` files.

## Black, Blank, or Frozen Output

Check:

- Post effect script has `ScriptClass="scene"` and correct `ScriptOrder`.
- `ScriptExternal=Color` is present when the scene color must be captured.
- Buffer passes include pass-level `string Script="Draw=Buffer;"`.
- Geometry passes include pass-level `Draw=Geometry` when needed.
- Render targets are cleared before use.
- Final pass restores `RenderColorTarget0=;` and `RenderDepthStencilTarget=;` when drawing to the screen.
- Full-screen quad vertex shader outputs valid clip-space coordinates and UVs.
- Preview panel/output size is not too small for sdPBR-style effects.

## Wrong Object Rendering

Check:

- Missing `object_ss` variants when self-shadow is enabled.
- Missing `UseTexture`, `UseSphereMap`, or `UseToon` technique variants.
- Wrong `Subset` technique selection.
- Edge/shadow/zplot techniques are empty when they should draw, or draw when they should hide.
- `DefaultEffect` routes use the correct wildcard pattern and hide controller objects.
- Alpha threshold logic excludes intended semi-transparent materials.

## Framework-Specific Symptoms

ray-mmd:

- Verify whether the task belongs in material, ray config, lighting/fog/skybox, or postprocess.
- Check `ray.conf`, `ray_advanced.conf`, and `Shader/textures.fxsub`.
- Missing IBL/LUT/controller assets often appear as black or flat lighting.
- Refresh all after config changes.

ExcellentShadow2:

- Confirm `ExcellentShadow.x` is loaded and positioned near the subject.
- Confirm all models/accessories use `full_ES.fx` or `full_ES_pmx.fx`.
- Confirm self-shadow is enabled.
- Reduce `EXSHADOW_QUALITY` if too heavy.
- AbsoluteShadow is incompatible; semi-transparent shadows are unreliable.

sdPBR480:

- Confirm `sdPBR.pmx` and `sdPBRGBuffer.x` are both loaded.
- Put `sdPBRGBuffer.x` high/early in draw order.
- Check `shader/sdPBRconfig.fxsub` and generated resource DLL issues.
- Reduce extra light, OmniLight, volume, shadow-map, SSDO, and GBuffer alpha settings under VRAM pressure.
- For black output, first re-run the documented quick-start load order.

## Debug Method

Change one variable at a time:

1. Reproduce with a minimal scene.
2. Disable optional framework features.
3. Replace custom shader with official `full.fx` or framework default material.
4. Add render targets/passes back one by one.
5. Verify each intermediate texture by temporarily drawing it to screen.
6. Only then edit shader logic.

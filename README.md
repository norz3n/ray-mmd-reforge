Ray-MMD Reforge
========
### Heavyweight graphics. Reforged for perfection. ###
<div align="center">
  <img src="./Shader/screenshots/logo.png" alt="logo" width="600">
</div>

　　**Ray-MMD Reforge** is a modified fork of the original Ray-MMD library for [mikumikudance](http://www.geocities.jp/higuchuu4/index_e.htm).
　　Focused on bridging realistic lighting with stylized rendering, Reforge rebuilds the engine pipeline around direct, high-precision screen-space raymarching and ultrafast root-finding algorithms. On top of it sit modern graphics techniques: **Temporal Anti-Aliasing (TAA)**, **Screen Space Global Illumination (SSGI)**, **Hybrid HBAO/SSDO**, and the **AgX tone mapper** — for a stable, physically grounded image out of the box.

Screenshots:
------------
> *Note: These are legacy screenshots from the original Ray-MMD. New showcases demonstrating SSGI, HBAO/SSDO, and TAA are coming soon!*

[![link text](./Shader/screenshots/screen1_small.jpg)](https://raw.githubusercontent.com/ray-cast/ray-mmd/master/Shader/screenshots/screen1.jpg)
[![link text](./Shader/screenshots/screen2_small.png)](https://raw.githubusercontent.com/ray-cast/ray-mmd/master/Shader/screenshots/screen2.png)
[![link text](./Shader/screenshots/screen3_small.jpg)](https://raw.githubusercontent.com/ray-cast/ray-mmd/master/Shader/screenshots/screen3.png)
[![link text](./Shader/screenshots/screen4_small.jpg)](https://raw.githubusercontent.com/ray-cast/ray-mmd/master/Shader/screenshots/screen4.png)

Requirement :
------------
* [MikuMikuDance](http://www.geocities.jp/higuchuu4/index_e.htm) - 926ver (x64) (Without Anti-Aliasing)
* [MikuMikuEffect](https://bowlroll.net/file/35012) - 037ver (x64)
* Direct3D 9 With Shader Model 3.0 (ps_3_0)
* **Powerful GPU recommended** due to advanced shading techniques.

Reforge Exclusive Features (through v1.17.0) :
------------

**Direct Screen-Space Core**
* **Direct 1:1 Screen-Space Architecture**: eliminated the heavy 11-pass hierarchical depth pyramid (Hi-Z), freeing 11 RenderTarget textures in VRAM and removing per-frame downsampling passes in favor of direct G-buffer raymarching and Newton root-finding.

**Global Illumination**
* **Screen Space Global Illumination (SSGI)**: rewritten modular architecture using hybrid linear-quadratic raymarching at full 1:1 resolution, replacing both legacy VXGI and Hi-Z dependent traces.
* **Indirect Multi-Bounce Mode**: Jimenez-style albedo compensation, hemispherical Lambertian emission lobe, quadratic AO attenuation, and outdoor sky radiance gather.
* **Rough Specular GI**: cone-angle-controlled glossy indirect reflections with tunable debug morphs.
* **Visibility Masking**: dedicated SSGI visibility pass with quality presets to suppress self-illumination feedback on skin.
* **Photometric Compression**: Reinhard soft-knee curve for natural dynamic range in lit scenes.

**Reflections & Occlusion**
* **PBR Screen-Space Reflections**: binary-search refinement, depth-aware bilateral blur, edge fade, and energy-conserving BRDF integration.
* **Hybrid HBAO / SSDO**: horizon-based and directional occlusion for accurate contact shading.
* **Contact Shadows**: screen-space contact shadows with depth-discontinuity artifact fixes.
* **Directional Bent Normals**: occlusion-aware normal bending for more believable indirect shading.

**Lighting & Shadows**
* **Variance Shadow Maps**: ultra-clean high-resolution sun shadows, completely grain-free on character faces, with rotated Vogel-disk PCF filtering to eliminate staircasing and acne.
* **Screen-Space Global Shadows (SSGS)**: long-range directional raymarched shadows with distance-adaptive soft penumbra expansion, grounding characters and geometry without shadow map dependence.
* **Dithered Volumetrics**: interleaved gradient noise for godrays and volumetric fog — smooth gradients without banding.

**Materials**
* **Dual-Lobe Skin Specular**: physically based dermal reflectance with strict light absorption for natural skin tone under GI.
* **Analytic Pre-Integrated Skin Shading**: dynamic-curvature subsurface scattering, screen-space depth-thickness measurement, and physical forward transmission for organic translucency. Scale-invariant, so miniature models scatter correctly too.
* **Procedural Hair Materials**: mathematical anisotropic hair normals plus Kajiya-Kay highlights — ported hbee hair presets with no heavy static textures.
* **Physical Cloth & ClearCoat**: rewritten BRDFs with cloth-DFG and Charlie Sheen distribution.
* **Procedural Eyes**: spherical corneal dome normal generation with limbal ring, UV-atlas safe, and CONVEX_NORMAL inversion mode for concave eye meshes. Dedicated Cornea ClearCoat eye materials included.
* **Glass Pipeline**: tinted glass presets inheriting MMD diffuse color, chromatic-dispersion refraction, and ultrafast Newton's method screen-space refraction root-finding (Mayer et al. 2026) converging in 3–4 iterations directly against the G-buffer.
* **Forced Transparency Presets**: make an opaque PMX material transparent without editing the model — plain and glass (SHADINGMODELID_GLASS refraction) variants, shaded through the alpha gbuffer with the model's own texture and MMD diffuse.
* **Wetness Special-Case Material**: now with ordered-dither alpha clipping; alpha cutout threshold unified at 0.5 across all passes.
* **Procedural Foliage Wind**: vertex wind animation engine with four vegetation presets.
* **Advanced Surface Detail**: thin-film iridescence, specular geometric anti-aliasing, Ultra Quality bump maps, and expanded Auto-Normal material presets.

**Post-Processing**
* **AgX Tone Mapping**: exact 6th-order polynomial implementation of the official Blender 4.0 AgX mapper (default), with an ACES-fitted option.
* **Camera Motion Blur**: cinematic screen-space velocity motion blur reconstructed from camera view-projection history, universal across all AA modes with 12-tap linear accumulation and shutter control.
* **Spectral / Chromatic Bloom**: physical radial wavelength dispersion across bloom tiers simulating anamorphic/optical lens dispersion.
* **Temporal Anti-Aliasing (TAA)**: 5-tap Catmull-Rom bicubic history reconstruction, Karis luma weighting, variance clipping, and depth-validated history.
* **Cinematic Bokeh DOF**: clean-room hexagonal and cinematic bokeh with blade count control.
* **Anti-Firefly Bloom**: Karis 13-tap downsampling filter with a soft-knee curve.
* **Image Enhancement**: AMD FidelityFX CAS sharpening, procedural cinematic film grain, and a Panavision anamorphic lens flare profile.
* **2-Band Cel-Shading**: optional stylized ramp integrated into the lighting path.

**Toon Rendering**
* **Community Toon Grading**: shadow color grade (multiply / add / hue-rotate / luminance-preserving warmth), low-saturation auto-tint for colorless albedos, Jashin self-power shadow fallback, and silhouette rim light folded into the cel bands — ported from HAToon2/PAToon2, M4Toon2, Jashin Toon and T_ToonShader as `ray.conf` knobs.
* **Crisp Cel Ramp**: anti-aliased two-band terminator with a half-strength core shadow, hardened cast-shadow edges, stepped anime highlights, flat two-step ambient fill, and full SSGI reception on toon materials.
* **Materials/Toon Library**: the entire material tree duplicated as ready-made cel-shaded presets with per-category shadow colors — skin, hair (including procedural presets), eyes, cloth, foliage — plus base cel and tone-based templates and a README.

Standard Features (Inherited) :
------------
* Physically-Based Material: albedo, metallic, smoothness/roughness, specular/reflectance, emissive, etc
* Clear coat material with absorption to simulate a second layer
* Cloth material with cloth-DFG to simulate a specular reflection
* Anisotropic material to simulate a specular reflection
* Special-Case Materials Wetness
* Approximation subsurface scattering materials
* Cook-Torrance microfacet specular BRDF (GGX) and burley diffuse BRDF
* Physical light units & Multiple light sources (Point, spot, sun, reactangle, disk, sphere, tube, ies)
* IES light profiles (point and spot light support)
* HDR linear lighting
* Volumetric light and Volumetric fog (cube and sphere fog support)
* Approximation atmospheric fog and sky scattering
* Image-based lighting based on RGBT encode
* Post-Process Bokeh Depth Of Field, Bloom, Tone-mapping, Color Balance

Resources :
------------
- HDRi
	- sIBL Archive - Hdrlabs.com \[[link](http://www.hdrlabs.com/sibl/archive.html)\].
	- ++skies; - **[aokcub](https://twitter.com/aokcub_cg)** \[[link](https://aokcub.net/cg/incskies/)\].
	- USC Institute \[[link](http://gl.ict.usc.edu/Data/HighResProbes)\].

Credits:
-------------
**Reforge Integrations & Special Thanks:**
* **hbee** - Original "DX RayCast Hair Pack" materials (adapted for procedural generation).
* Credit: material by dendewa (https://dendewa.vercel.app/) - Used for HBAO rendering concepts.
* **ikeno** - Referencing algorithms and mathematical concepts from `ikVXGI` for Global Illumination.

**Original Engine:**
Financially supported on [Patreon](http://www.patreon.com/cubizer):

#### Platinum supporters:
* Penti_mmd
* Robert Shawarden

##### Gold supporters:
* Sarashina - 更科
* Birdway

##### Bronze supporters:
* urara在処
* rin kari
* MMD-Seraph
* Rich El

`Thanks! and other supporters! (If you would like to be added or remove from this list Please contact me)`

Contact:
------------
　　If you are a developer using this as part of your love and considering contacting me, you can submit code by `Pull requests` or Feel free to contact me via `twitter` and `issues`, i'll add your profile to team members, Thanks.

* Reach me via Twitter: [@Rui](https://twitter.com/Rui_cg).

[License (MIT)](https://raw.githubusercontent.com/ray-cast/ray-mmd/master/LICENSE.txt)
-------------------------------------------------------------------------------
	Copyright (C) 2016-2018 Ray-MMD Developers. All rights reserved.
    (Reforge modifications are part of the extended project scope).

	https://github.com/ray-cast/ray-mmd

	Permission is hereby granted, free of charge, to any person obtaining a
	copy of this software and associated documentation files (the "Software"),
	to deal in the Software without restriction, including without limitation
	the rights to use, copy, modify, merge, publish, distribute, sublicense,
	and/or sell copies of the Software, and to permit persons to whom the
	Software is furnished to do so, subject to the following conditions:

	The above copyright notice and this permission notice shall be included
	in all copies or substantial portions of the Software.

	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
	OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	BRIAN PAUL BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
	AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
	CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Credits :
--------
* PBR Screen-Space Reflections based on Morgan McGuire & Michael Mara (2014) 2D DDA ray traversal.
* Ultrafast Screen-Space Refractions via Newton's Method based on Chase Mayer, Ulf Assarsson & Erik Sintorn (JCGT 2026).
* HBSSDO rendering concepts referenced from [dendewa](https://dendewa.vercel.app/).
* AgX Tone Mapping per the official Blender 4.0 implementation ([link](https://github.com/EaryChow/AgX)).
* Karis anti-firefly downsampling and luma weighting from Brian Karis' "Next Generation Post Processing in Call of Duty: Advanced Warfare".

References :
--------
* Ultrafast Screen-Space Refractions and Caustics via Newton's Method \[[link](https://jcgt.org/published/0015/01/03/)\].
* Moving to the Next Generation - The Rendering Technology of Ryse \[[link](http://www.crytek.com/download/2014_03_25_CRYENGINE_GDC_Schultz.pdf)\].
* ACES Filmic Tone Mapping Curve \[[link](https://knarkowicz.wordpress.com/2016/08/31/hdr-display-first-steps/)\].
* Compact Normal Storage for small G-Buffers \[[link](http://aras-p.info/texts/CompactNormalStorage.html)\].
* Convert Blinn-Phong to Beckmann distribution \[[link](http://simonstechblog.blogspot.de/2011/12/microfacet-brdf.html)\].
* Spherical Gaussian approximation for Blinn-Phong, Phong and Fresnel \[[link](https://seblagarde.wordpress.com/2012/06/03/spherical-gaussien-approximation-for-blinn-phong-phong-and-fresnel/)\].
* Horizon Occlusion for IBL \[[link](http://marmosetco.tumblr.com/post/81245981087)\].
* Screen space glossy reflections \[[link](http://roar11.com/2015/07/screen-space-glossy-reflections/)\].
* Parallax Occlusion Map \[[link](http://sunandblackcat.com/tipFullView.php?topicid=28)\].
* Special-Case Materials Wetness \[[link](http://advances.realtimerendering.com/other/2016/naughty_dog/NaughtyDog_TechArt_Final.pdf)\]
* (And many more from the original development team...)

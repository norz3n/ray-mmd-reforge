Ray-MMD Reforge
========
### Heavyweight graphics. Reforged for perfection. ###
<div align="center">
  <img src="./Shader/screenshots/logo.png" alt="logo" width="600">
</div>

　　**Ray-MMD Reforge** is a modified fork of the original Ray-MMD library for [mikumikudance](http://www.geocities.jp/higuchuu4/index_e.htm). 
　　Focused on bridging realistic lighting with stylized rendering, Reforge updates the original engine pipeline by integrating modern graphics techniques. We've added **Temporal Anti-Aliasing (TAA)**, **Screen Space Voxel Global Illumination (SSVGI)**, **Hybrid HBAO/SSDO**, and **Percentage-Closer Soft Shadows (PCSS)** to improve shadow accuracy, reduce visual noise, and provide a more stable image out of the box.

Screenshots:
------------
> *Note: These are legacy screenshots from the original Ray-MMD. New showcases demonstrating SSVGI, PCSS, and TAA are coming soon!*

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

Reforge Exclusive Features :
------------
* **Temporal Anti-Aliasing (TAA)**: Improved edge anti-aliasing and temporal stability to reduce subpixel flickering.
* **Hybrid HBAO / SSDO**: Horizon-based ambient occlusion and directional occlusion for more accurate contact shading.
* **Percentage-Closer Soft Shadows (PCSS)**: Distance-based soft shadows that blur based on the distance from the caster.
* **Screen Space Voxel Global Illumination (SSVGI)**: Hybrid GI architecture combining voxel grids with screen-space cone tracing for real-time indirect lighting (includes fixes for NaN/INF artifacts).
* **Contact Shadows**: Screen-space raymarched shadows to reduce light leaking and grounding issues.
* **Optimized SSR**: Upgraded Screen Space Reflections using Binary Search and depth-aware blur for better performance and accuracy.
* **Depth-Aware AA Pipeline**: Fixed black screen issues and updated edge detection for SMAA/TAA passes.

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
* Hierarchical Z-Buffer Screen Space Reflections and HBSSDO implementations by [dendewa](https://dendewa.vercel.app/).

References :
--------
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

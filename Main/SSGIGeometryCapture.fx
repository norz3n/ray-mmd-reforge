////////////////////////////////////////////////////////////////////////////////////////////////
//
//  Ray-MMD SSGI Geometry Capture — Object Shader
//
//  Renders all scene objects into a 2D-sliced depth atlas (SSGICaptureRT) for use
//  by the Screen-Space Voxel GI cone tracer (PostProcessVXGI.fxsub).
//
//  Reference Architecture & Algorithms:
//  - D3D9 2D Sliced Atlas Volume Math: ikeno (ikVXGI)
//
////////////////////////////////////////////////////////////////////////////////////////////////

#include "../ray.conf"
#include "../ray_advanced.conf"
#include "../Shader/common.fxsub"

#define SSGI_CAPTURE_OBJECT_SHADER
#include "../Shader/SSGIGeometryCapture.fxsub"

technique MainTec0 < string MMDPass = "object"; > {
	pass DrawCapture {
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = true; ZWriteEnable = true;
		CullMode = NONE;
		VertexShader = compile vs_3_0 SSGICaptureVS();
		PixelShader  = compile ps_3_0 SSGICapturePS();
	}
}
technique MainTecBS0 < string MMDPass = "object_ss"; > {
	pass DrawCapture {
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = true; ZWriteEnable = true;
		CullMode = NONE;
		VertexShader = compile vs_3_0 SSGICaptureVS();
		PixelShader  = compile ps_3_0 SSGICapturePS();
	}
}

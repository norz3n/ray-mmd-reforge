#include "../ray.conf"
#include "../ray_advanced.conf"
#include "../Shader/common.fxsub"
#include "../Shader/Voxelize.fxsub"

// Voxelization shader pass for MMD Offscreen Render Target
technique MainTec0 < string MMDPass = "object"; > {
	pass DrawVoxel {
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = true; ZWriteEnable = true;
		CullMode = NONE;
		VertexShader = compile vs_3_0 VoxelVS();
		PixelShader  = compile ps_3_0 VoxelPS();
	}
}
technique MainTecBS0 < string MMDPass = "object_ss"; > {
	pass DrawVoxel {
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = true; ZWriteEnable = true;
		CullMode = NONE;
		VertexShader = compile vs_3_0 VoxelVS();
		PixelShader  = compile ps_3_0 VoxelPS();
	}
}

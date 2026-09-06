#include "ray.conf"
#include "ray_advanced.conf"

const float4 BackColor = 0.0;
const float4 WhiteColor = 1.0;
const float ClearDepth = 1.0;
const int ClearStencil = 0;

float mSunLightP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SunLight+";>;
float mSunLightM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SunLight-";>;
float mSunShadowRP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SunShadowR+";>;
float mSunShadowGP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SunShadowG+";>;
float mSunShadowBP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SunShadowB+";>;
float mSunShadowVM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SunShadowV-";>;
float mSSAOP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSAO+";>;
float mSSAOM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSAO-";>;
float mSSAORadiusP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSAORadius+";>;
float mSSAORadiusM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSAORadius-";>;
float mSSDOP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSDO+";>;
float mSSDOM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSDO-";>;
float mSSSSP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSSS+";>;
float mSSSSM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSSS-";>;
float mExposureP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Exposure+";>;
float mExposureM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Exposure-";>;
float mFstopP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Fstop+";>;
float mFstopM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Fstop-";>;
float mFocalLengthP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "FocalLength+";>;
float mFocalLengthM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "FocalLength-";>;
float mFocalDistanceP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "FocalDistance+";>;
float mFocalDistanceM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "FocalDistance-";>;
float mFocalRegionP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "FocalRegion+";>;
float mFocalRegionM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "FocalRegion-";>;
float mBladeCountM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BladeCount-";>;
float mMeasureMode : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "MeasureMode";>;
float mTestMode : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "TestMode";>;
float mVignette : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Vignette";>;
float mDispersion : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Dispersion";>;
float mDispersionRadius : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "DispersionRadius";>;
float mBloomThresholdP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomThreshold";>;
float mBloomRadiusP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomRadius+";>;
float mBloomRadiusM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomRadius-";>;
float mBloomColorAllHP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomColorAllH+";>;
float mBloomColorAllSP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomColorAllS+";>;
float mBloomColorAllVP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomColorAllV+";>;
float mBloomColorAllVM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomColorAllV-";>;
float mBloomStarFade : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BloomStarFade";>;
float mContrastP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Contrast+";>;
float mContrastM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Contrast-";>;
float mSaturationP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Saturation+";>;
float mSaturationM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Saturation-";>;
float mGammaP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Gamma+";>;
float mGammaM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Gamma-";>;
float mColBalanceRP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BalanceR+";>;
float mColBalanceGP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BalanceG+";>;
float mColBalanceBP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BalanceB+";>;
float mColBalanceRM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BalanceR-";>;
float mColBalanceGM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BalanceG-";>;
float mColBalanceBM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "BalanceB-";>;
float mTemperatureP : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Temperature+";>;
float mTemperatureM : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "Temperature-";>;

#if WATER_CAUSTICS_ENABLE
// CausticsController.pmx (Dedicated separate controller)
float mCstIntensityP1   : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Intensity+";>;
float mCstIntensityM1   : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Intensity-";>;
float mCstWaterHeightP1 : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "WaterHeight+";>;
float mCstWaterHeightM1 : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "WaterHeight-";>;
float mCstScaleP1       : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Scale+";>;
float mCstScaleM1       : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Scale-";>;
float mCstSpeedP1       : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Speed+";>;
float mCstSpeedM1       : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Speed-";>;
float mCstDispersionP1  : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Dispersion+";>;
float mCstDispersionM1  : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Dispersion-";>;
float3 mCstWaterPos1    : CONTROLOBJECT<string name="CausticsController.pmx"; string item = "Position";>;

float mCstIntensityP2   : CONTROLOBJECT<string name="CausticsController"; string item = "Intensity+";>;
float mCstIntensityM2   : CONTROLOBJECT<string name="CausticsController"; string item = "Intensity-";>;
float mCstWaterHeightP2 : CONTROLOBJECT<string name="CausticsController"; string item = "WaterHeight+";>;
float mCstWaterHeightM2 : CONTROLOBJECT<string name="CausticsController"; string item = "WaterHeight-";>;
float mCstScaleP2       : CONTROLOBJECT<string name="CausticsController"; string item = "Scale+";>;
float mCstScaleM2       : CONTROLOBJECT<string name="CausticsController"; string item = "Scale-";>;
float mCstSpeedP2       : CONTROLOBJECT<string name="CausticsController"; string item = "Speed+";>;
float mCstSpeedM2       : CONTROLOBJECT<string name="CausticsController"; string item = "Speed-";>;
float mCstDispersionP2  : CONTROLOBJECT<string name="CausticsController"; string item = "Dispersion+";>;
float mCstDispersionM2  : CONTROLOBJECT<string name="CausticsController"; string item = "Dispersion-";>;
float3 mCstWaterPos2    : CONTROLOBJECT<string name="CausticsController"; string item = "Position";>;
#endif

#if SKY_VISIBILITY_ENABLE
#define SKY_VIS_CONTROLLER_AVAILABLE 1
// SkyVisibilityController.pmx (Dedicated separate controller - placed before SSGI)
float mSkyVisIntensityP1 : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Intensity+";>;
float mSkyVisIntensityM1 : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Intensity-";>;
float mSkyVisP1          : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "SkyVis+";>;
float mSkyVisM1          : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "SkyVis-";>;
float mSkyVisRadiusP1    : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Radius+";>;
float mSkyVisRadiusM1    : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Radius-";>;
float mSkyVisFloorP1     : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Floor+";>;
float mSkyVisFloorM1     : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Floor-";>;
float mSkyVisContrastP1  : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Contrast+";>;
float mSkyVisContrastM1  : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Contrast-";>;
float3 mSkyVisPos1       : CONTROLOBJECT<string name="SkyVisibilityController.pmx"; string item = "Position";>;

float mSkyVisIntensityP2 : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Intensity+";>;
float mSkyVisIntensityM2 : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Intensity-";>;
float mSkyVisP2          : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "SkyVis+";>;
float mSkyVisM2          : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "SkyVis-";>;
float mSkyVisRadiusP2    : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Radius+";>;
float mSkyVisRadiusM2    : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Radius-";>;
float mSkyVisFloorP2     : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Floor+";>;
float mSkyVisFloorM2     : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Floor-";>;
float mSkyVisContrastP2  : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Contrast+";>;
float mSkyVisContrastM2  : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Contrast-";>;
float3 mSkyVisPos2       : CONTROLOBJECT<string name="SkyVisibilityController"; string item = "Position";>;

static float mSkyVisIntensityP = max(max(mSkyVisIntensityP1, mSkyVisIntensityP2), max(mSkyVisP1, mSkyVisP2));
static float mSkyVisIntensityM = max(max(mSkyVisIntensityM1, mSkyVisIntensityM2), max(mSkyVisM1, mSkyVisM2));
static float mSkyVisRadiusP    = max(mSkyVisRadiusP1, mSkyVisRadiusP2);
static float mSkyVisRadiusM    = max(mSkyVisRadiusM1, mSkyVisRadiusM2);
static float mSkyVisFloorP     = max(mSkyVisFloorP1, mSkyVisFloorP2);
static float mSkyVisFloorM     = max(mSkyVisFloorM1, mSkyVisFloorM2);
static float mSkyVisContrastP  = max(mSkyVisContrastP1, mSkyVisContrastP2);
static float mSkyVisContrastM  = max(mSkyVisContrastM1, mSkyVisContrastM2);
static float3 mSkyVisPos       = any(mSkyVisPos1) ? mSkyVisPos1 : mSkyVisPos2;

static float mSkyVisIntensityCtrl = lerp(lerp(mSkyVisibilityIntensity, mSkyVisibilityIntensity * 3.0f, mSkyVisIntensityP), 0.0f, mSkyVisIntensityM);
static float mSkyVisRadiusScale   = lerp(lerp(1.0f, 2.5f, mSkyVisRadiusP), 0.25f, mSkyVisRadiusM);
static float mSkyVisFloorCtrl     = lerp(lerp(mSkyVisibilityMinFloor, 0.6f, mSkyVisFloorP), 0.0f, mSkyVisFloorM);
static float mSkyVisContrastCtrl  = lerp(lerp(1.0f, 2.5f, mSkyVisContrastP), 0.4f, mSkyVisContrastM);
#endif

float mDbgSSGIIntensity1 : CONTROLOBJECT<string name="DebugController.pmx"; string item = "SSGIIntensity";>;
float mDbgSSGIConeAngle1 : CONTROLOBJECT<string name="DebugController.pmx"; string item = "SSGIConeAngle";>;
float mDbgSSGIBias1      : CONTROLOBJECT<string name="DebugController.pmx"; string item = "SSGIBias";>;

float mDbgSSGIIntensity2 : CONTROLOBJECT<string name="DebugController"; string item = "SSGIIntensity";>;
float mDbgSSGIConeAngle2 : CONTROLOBJECT<string name="DebugController"; string item = "SSGIConeAngle";>;
float mDbgSSGIBias2      : CONTROLOBJECT<string name="DebugController"; string item = "SSGIBias";>;

float mCtrlSSGIIntensity : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSGIIntensity";>;
float mCtrlSSGIConeAngle : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSGIConeAngle";>;
float mCtrlSSGIBias      : CONTROLOBJECT<string name="ray_controller.pmx"; string item = "SSGIBias";>;

static float mDbgSSGIIntensity = max(mDbgSSGIIntensity1, max(mDbgSSGIIntensity2, mCtrlSSGIIntensity));
static float mDbgSSGIConeAngle = max(mDbgSSGIConeAngle1, max(mDbgSSGIConeAngle2, mCtrlSSGIConeAngle));
static float mDbgSSGIBias      = max(mDbgSSGIBias1,      max(mDbgSSGIBias2,      mCtrlSSGIBias));

static float mSSAOScale = lerp(lerp(mSSDOIntensityMin, mSSDOIntensityMax, mSSAOP), 0, mSSAOM);
static float mSSAORadius = lerp(lerp(1.0, 2.0, mSSAORadiusP), 0.5, mSSAORadiusM);
static float mSSDOScale = lerp(lerp(mSSDOIntensityMin, mSSDOIntensityMax, mSSDOP), 0, mSSDOM);
static float mSSSSScale = lerp(lerp(mSSSSIntensityMin, mSSSSIntensityMax, mSSSSP), 0.25, mSSSSM);
static float mSSGIIntensity = lerp(mSSGIIntensityDefault, 5.0, mDbgSSGIIntensity);
static float mSSGIConeAngle = lerp(mSSGIConeAngleDefault, 1.8, mDbgSSGIConeAngle);
static float mSSGIBias = lerp(mSSGIBiasDefault, 2.0, mDbgSSGIBias);
static float mSSRBlur = mSSRBlurDefault;
static float mSSRThickness = mSSRThicknessDefault;
static float mSSROffset = mSSROffsetDefault;
static float mSSRSmoothness = mSSRSmoothnessDefault;
static float mSSRBrightness = mSSRBrightnessDefault;
static float mSSRFresnel = mSSRFresnelDefault;
static float mSunIntensity = lerp(lerp(mLightIntensityMin, mLightIntensityMax, mSunLightP), 0, mSunLightM);
static float mExposure = lerp(lerp(mExposureMin, mExposureMax, mExposureP), 0, mExposureM);
static float mBloomRadius = lerp(lerp(2.2, 10, mBloomRadiusP), 0.1, mBloomRadiusM);
static float mBloomThreshold = (1.0 - mBloomThresholdP) / (mBloomThresholdP + 1e-5);
static float mColorContrast = lerp(lerp(1, 2, mContrastP), 0.5, mContrastM);
static float mColorSaturation = lerp(lerp(1, 2, mSaturationP), 0.0, mSaturationM);
static float mColorGamma = lerp(lerp(1.0, 0.45, mGammaP), 2.2, mGammaM);
static float mColorTemperature = lerp(lerp(mTemperature, 1000, mTemperatureP), 40000, mTemperatureM);
static float mFstop = lerp(lerp(5.6, 32.0, mFstopP), 1.0, mFstopM);
static float mFocalDistance = lerp(lerp(1, 10.0, mFocalDistanceP), -10.0, mFocalDistanceM);
static float mFocalRegion = lerp(0.0, 10.0, mFocalRegionP);
static float mBladeCount = lerp(10, 5, mBladeCountM);
static float3 mColorShadowSunP = pow(float3(mSunShadowRP, mSunShadowGP, mSunShadowBP), 2);
static float3 mColorBalanceP = float3(mColBalanceRP, mColBalanceGP, mColBalanceBP);
static float3 mColorBalanceM = float3(mColBalanceRM, mColBalanceGM, mColBalanceBM);
#if WATER_CAUSTICS_ENABLE
static float mCstIntensityP   = max(mCstIntensityP1, mCstIntensityP2);
static float mCstIntensityM   = max(mCstIntensityM1, mCstIntensityM2);
static float mCstWaterHeightP = max(mCstWaterHeightP1, mCstWaterHeightP2);
static float mCstWaterHeightM = max(mCstWaterHeightM1, mCstWaterHeightM2);
static float mCstScaleP       = max(mCstScaleP1, mCstScaleP2);
static float mCstScaleM       = max(mCstScaleM1, mCstScaleM2);
static float mCstSpeedP       = max(mCstSpeedP1, mCstSpeedP2);
static float mCstSpeedM       = max(mCstSpeedM1, mCstSpeedM2);
static float mCstDispersionP  = max(mCstDispersionP1, mCstDispersionP2);
static float mCstDispersionM  = max(mCstDispersionM1, mCstDispersionM2);
static float3 mCstWaterPos    = any(mCstWaterPos1) ? mCstWaterPos1 : mCstWaterPos2;

static float mCausticsIntensity  = lerp(lerp(WATER_CAUSTICS_INTENSITY, WATER_CAUSTICS_INTENSITY * 3.0f, mCstIntensityP), 0.0f, mCstIntensityM);
static float mWaterHeight        = WATER_CAUSTICS_SURFACE_Y + mCstWaterPos.y + lerp(lerp(0.0f, 30.0f, mCstWaterHeightP), -30.0f, mCstWaterHeightM);
static float mCausticsFreqScale  = lerp(lerp(1.15f, 3.0f, mCstScaleP), 0.3f, mCstScaleM);
static float mCausticsSpeedScale = lerp(lerp(1.0f, 2.5f, mCstSpeedP), 0.1f, mCstSpeedM);
static float mCausticsDispScale  = lerp(lerp(1.0f, 3.0f, mCstDispersionP), 0.0f, mCstDispersionM);
#endif

#include "shader/math.fxsub"
#include "shader/common.fxsub"
#include "shader/textures.fxsub"
#if (AA_QUALITY == 6) || POST_MOTION_BLUR_ENABLE || AO_TEMPORAL_DENOISE
#	include "shader/PostProcessMatrix.fxsub"
#endif
#include "shader/gbuffer.fxsub"
#include "shader/ibl.fxsub"
#include "shader/BRDF.fxsub"
#include "shader/ColorGrading.fxsub"
#include "shader/ShadingMaterials.fxsub"



#if SUN_SHADOW_QUALITY && SUN_LIGHT_ENABLE
#	include "shader/ShadowCommon.fxsub"
#	include "shader/ShadowMapCascaded.fxsub"
#	include "shader/ShadowMap.fxsub"
#endif

#if SSDO_QUALITY && (IBL_QUALITY || SUN_LIGHT_ENABLE)
#if SSAO_TYPE == 0
#	include "shader/PostProcessOcclusion.fxsub"
#elif SSAO_TYPE == 1
#	include "shader/PostProcessOcclusionHBAO.fxsub"
#elif SSAO_TYPE == 2
#	include "shader/PostProcessOcclusionGTAO.fxsub"
#endif
#endif

#if SSSS_QUALITY
#	include "shader/PostProcessScattering.fxsub"
#endif

#if OUTLINE_QUALITY == 2
#	include "shader/EdgeLineAA.fxsub"
#endif

#if TOON_ENABLE == 2
#	include "shader/PostProcessDiffusion.fxsub"
#endif

#if SSR_QUALITY
#	include "shader/SSR/SSR_Main.fxsub"
#endif

#if GI_ENABLE
#	include "shader/PostProcessSSGI.fxsub"
#endif

#ifndef BOKEH_MODE
#	define BOKEH_MODE BOKEH_QUALITY
#endif

#if BOKEH_MODE == 1
#	include "shader/PostProcessBokeh.fxsub"
#elif BOKEH_MODE == 2
#	include "shader/PostProcessHexagonalBokeh.fxsub"
#endif

#if HDR_EYE_ADAPTATION
#	include "shader/PostProcessEyeAdaptation.fxsub"
#endif

#if HDR_STAR_MODE
#	include "shader/PostProcessLensflare.fxsub"
#endif

#if HDR_FLARE_MODE
#	include "shader/PostProcessGhost.fxsub"
#endif

#if HDR_BLOOM_MODE
#	include "shader/PostProcessBloom.fxsub"
#endif

#include "shader/PostProcessHDR.fxsub"

#if AA_QUALITY == 1
#	include "shader/FXAA3.fxsub"
#endif

#if AA_QUALITY >= 2 && AA_QUALITY <= 5
#	include "shader/SMAA.fxsub"
#endif

#if AA_QUALITY == 6
#	include "shader/TAA.fxsub"
#endif

#if AA_QUALITY == 7
#	include "shader/DLAA.fxsub"
#endif

#if POST_MOTION_BLUR_ENABLE
#	include "shader/PostProcessMotionBlur.fxsub"
#endif

#if POST_SHARPEN_ENABLE
#	include "shader/PostProcessSharpen.fxsub"
#endif


float4 ScreenSpaceQuadVS(
	in float4 Position : POSITION,
	in float4 Texcoord : TEXCOORD,
	out float4 oTexcoord0 : TEXCOORD0,
	out float3 oTexcoord1 : TEXCOORD1) : POSITION
{
	oTexcoord0 = Texcoord;
	oTexcoord0.xy += ViewportOffset;
	oTexcoord0.zw = oTexcoord0.xy * ViewportSize;
	oTexcoord1 = -mul(Position, matProjectInverse).xyz;
	return Position;
}

float4 ScreenSpaceQuadOffsetVS(
	in float4 Position : POSITION,
	in float2 Texcoord : TEXCOORD,
	out float2 oTexcoord : TEXCOORD0,
	uniform float2 offset) : POSITION
{
	oTexcoord = Texcoord + offset * 0.5;
	return Position;
}

float Script : STANDARDSGLOBAL<
	string ScriptOutput = "color";
	string ScriptClass  = "scene";
	string ScriptOrder  = "postprocess";
> = 0.8;

technique DeferredLighting<
	string Script =
	"RenderColorTarget=ScnMap;"
	"RenderDepthStencilTarget=DepthBuffer;"
	"ClearSetColor=BackColor;"
	"ClearSetDepth=ClearDepth;"
	"Clear=Color;"
	"Clear=Depth;"
	"ScriptExternal=Color;"


#if SUN_SHADOW_QUALITY && SUN_LIGHT_ENABLE
	"RenderColorTarget=ShadowMap;"
	"ClearSetColor=WhiteColor;"
	"Clear=Color;"
	"Pass=ShadowMapGen;"
	"ClearSetColor=BackColor;"
#if SHADOW_BLUR_COUNT
	"RenderColorTarget=ShadowMapTemp; Pass=ShadowBlurX;"
	"RenderColorTarget=ShadowMap;	  Pass=ShadowBlurY;"
#endif
#endif

#if SSDO_QUALITY && (IBL_QUALITY || SUN_LIGHT_ENABLE)
	"RenderColorTarget=SSDOMap; Pass=SSDO;"
#if SSDO_BLUR_RADIUS
	"RenderColorTarget=SSDOMapTemp; Pass=SSDOBlurX;"
	"RenderColorTarget=SSDOMap;	    Pass=SSDOBlurY;"
#endif
#if AO_TEMPORAL_DENOISE
	"RenderColorTarget0=SSDOMapTemp; RenderColorTarget1=SSDOMapHistory; Pass=SSDOTemporalDenoise;"
	"RenderColorTarget1=;"
	"RenderColorTarget=SSDOMap; Pass=SSDOCopyTemporal;"
#endif
#endif

#if OUTLINE_QUALITY == 2
	"RenderColorTarget=EdgeEdgeMap;  Clear=Color; Pass=EdgeEdgeDetection;"
	"RenderColorTarget=EdgeBlendMap; Clear=Color; Pass=EdgeBlendingWeightCalculation;"
	"RenderColorTarget=OutlineTempMap; Pass=EdgeNeighborhoodBlending;"
#endif

#if SSSS_QUALITY
	"RenderColorTarget0=ShadingMapTemp;"
	"RenderColorTarget1=ShadingMapTempSpecular;"
	"Pass=ShadingOpacity;"
	"RenderColorTarget1=;"

	"RenderDepthStencilTarget=DepthBuffer;"
	"RenderColorTarget=;"
	"Clear=Depth;"
	"Pass=SSSSStencilTest;"
	"RenderColorTarget=ShadingMap; Clear=Color; Pass=SSSSBlurX;"
	"RenderColorTarget=ShadingMapTemp;	Pass=SSSSBlurY;"
	"RenderColorTarget=ShadingMapTemp;	Pass=ShadingOpacityAlbedo;"
	"RenderColorTarget=ShadingMapTemp;	Pass=ShadingOpacitySpecular;"
	"RenderColorTarget=ShadingMap;		Pass=ShadingTransparent;"
#else
	"RenderColorTarget=ShadingMapTemp;	Pass=ShadingOpacity;"
	"RenderColorTarget=ShadingMap;		Pass=ShadingTransparent;"
#endif

#if TOON_ENABLE == 2
	"RenderColorTarget=ShadingMapTemp; 	Pass=DiffusionBlurX;"
	"RenderColorTarget=ShadingMap; 		Pass=DiffusionBlurY;"
#endif

#if GI_ENABLE
	// Reject trace outliers first; blur and resolve reconstruct a low-frequency GI field.
	"RenderColorTarget=SSGIMap;     Clear=Color; Pass=SSGI;"
	"RenderColorTarget=SSGIMapTemp; Pass=SSGIPrefilter;"
	"RenderColorTarget=SSGIMap;     Pass=SSGIBlurX;"
	"RenderColorTarget=SSGIMapTemp; Pass=SSGIBlurY;"
	"RenderColorTarget=SSGIMap;     Pass=SSGIBlurX2;"
	"RenderColorTarget=SSGIMapTemp; Pass=SSGIBlurY2;"
	"RenderColorTarget=ShadingMap;  Pass=SSGIFinalCombine;"
#endif

#if SSR_QUALITY
	"RenderColorTarget=SSRLightX1Map;"
	"Clear=Color;"
	"Pass=SSR_Trace;"

	"RenderColorTarget=SSRLightX2MapTemp; Pass=SSR_BlurX2;"
	"RenderColorTarget=SSRLightX2Map;	  Pass=SSR_BlurY2;"
	"RenderColorTarget=SSRLightX3MapTemp; Pass=SSR_BlurX3;"
	"RenderColorTarget=SSRLightX3Map;	  Pass=SSR_BlurY3;"
	"RenderColorTarget=SSRLightX4MapTemp; Pass=SSR_BlurX4;"
	"RenderColorTarget=SSRLightX4Map;	  Pass=SSR_BlurY4;"

	"RenderColorTarget=ShadingMap;		  Pass=SSR_Resolve;"
#endif

#if BOKEH_MODE == 1
	"RenderColorTarget=AutoFocalMap;          Clear=Color; Pass=ComputeFocalDistance;"
	"RenderColorTarget=FocalBokehKernelMap;   Clear=Color; Pass=ComputeBokehKernel;"
	"RenderColorTarget=FocalCoCMap;           Clear=Color; Pass=ComputeBokehWeight;"
	"RenderColorTarget=FocalBokehMap;         Clear=Color; Pass=ComputeBokehFarPrefilter;"
	"RenderColorTarget=FocalBokehFarMap;      Clear=Color; Pass=ComputeBokehFar;"
	"RenderColorTarget=FocalBokehMap;         Clear=Color; Pass=ComputeBokehNearPrefilter;"
	"RenderColorTarget=FocalBokehNearMap;     Clear=Color; Pass=ComputeBokehNear;"
	"RenderColorTarget=FocalBokehNearBlurMap; Clear=Color; Pass=ComputeBilinearBlur;"

	"RenderColorTarget=ShadingMap; Pass=ComputeBokehFinal;"
#endif

#if BOKEH_MODE == 2
	"RenderColorTarget=AutoFocalMap;          Clear=Color; Pass=ComputeFocalDistance;"
	"RenderColorTarget=FocalCoCMap;           Clear=Color; Pass=ComputeBokehWeight;"

	"RenderColorTarget=FocalBokehMap;         Clear=Color; Pass=ComputeBokehFarPrefilter;"
	"RenderColorTarget0=FocalBlur1Map;"
	"RenderColorTarget1=FocalBlur2Map;"
	"Clear=Color;"
	"Pass=ComputeHexagonalBlurX;"
	"RenderColorTarget1=;"

	"RenderColorTarget=FocalHexBokehMap;"
	"Clear=Color;"
	"Pass=ComputeHexagonalBlurY;"

	"RenderColorTarget=FocalBokehMap;         Clear=Color; Pass=ComputeBokehNearPrefilter;"
	"RenderColorTarget=FocalBlur2Map;         Clear=Color; Pass=ComputeBokehBlur;"
	"RenderColorTarget=FocalBlur1Map;         Clear=Color; Pass=ComputeBilinearBlur;"

	"RenderColorTarget=ShadingMap; Pass=ComputeBokehFinal;"
#endif

#if HDR_EYE_ADAPTATION
	"RenderColorTarget=EyeLumMap; 	 Pass=EyeLum;"
	"RenderColorTarget=EyeLumAveMap; Pass=EyeAdapation;"
#endif

#if HDR_BLOOM_MODE
	"RenderColorTarget=DownsampleMap1st; Pass=GlareDetection;"
#if HDR_STAR_MODE || HDR_FLARE_MODE
	"RenderColorTarget=DownsampleMap2nd; Pass=HDRDownsample2nd;"
#endif
	"RenderColorTarget=BloomMap1stTemp;  Pass=BloomBlurX1;"
	"RenderColorTarget=BloomMap1st;		 Pass=BloomBlurY1;"
	"RenderColorTarget=BloomMap2nd;		 Pass=BloomDownsampleX2;"
	"RenderColorTarget=BloomMap2ndTemp;  Pass=BloomBlurX2;"
	"RenderColorTarget=BloomMap2nd;		 Pass=BloomBlurY2;"
	"RenderColorTarget=BloomMap3rd;		 Pass=BloomDownsampleX3;"
	"RenderColorTarget=BloomMap3rdTemp;  Pass=BloomBlurX3;"
	"RenderColorTarget=BloomMap3rd;		 Pass=BloomBlurY3;"
	"RenderColorTarget=BloomMap4th;		 Pass=BloomDownsampleX4;"
	"RenderColorTarget=BloomMap4thTemp;  Pass=BloomBlurX4;"
	"RenderColorTarget=BloomMap4th;		 Pass=BloomBlurY4;"
	"RenderColorTarget=BloomMap5th;		 Pass=BloomDownsampleX5;"
	"RenderColorTarget=BloomMap5thTemp;  Pass=BloomBlurX5;"
	"RenderColorTarget=BloomMap5th;		 Pass=BloomBlurY5;"
#if HDR_STAR_MODE == 1 || HDR_STAR_MODE == 2
	"RenderColorTarget=StreakMap1stTemp; Pass=Star1stStreak1st;"
	"RenderColorTarget=StreakMap1st;	 Pass=Star1stStreak2nd;"
	"RenderColorTarget=StreakMap1stTemp; Pass=Star1stStreak3rd;"
	"RenderColorTarget=StreakMap1st;	 Pass=Star1stStreak4th;"
	"RenderColorTarget=StreakMap2ndTemp; Pass=Star2ndStreak1st;"
	"RenderColorTarget=StreakMap2nd;	 Pass=Star2ndStreak2nd;"
	"RenderColorTarget=StreakMap2ndTemp; Pass=Star2ndStreak3rd;"
	"RenderColorTarget=StreakMap2nd;	 Pass=Star2ndStreak4th;"
#endif
#if HDR_STAR_MODE == 3 || HDR_STAR_MODE == 4
	"RenderColorTarget=StreakMap1st;	 Pass=Star1stStreak1st;"
	"RenderColorTarget=StreakMap1stTemp; Pass=Star1stStreak2nd;"
	"RenderColorTarget=StreakMap1st;	 Pass=Star1stStreak3rd;"
	"RenderColorTarget=StreakMap2nd;	 Pass=Star2ndStreak1st;"
	"RenderColorTarget=StreakMap2ndTemp; Pass=Star2ndStreak2nd;"
	"RenderColorTarget=StreakMap2nd;	 Pass=Star2ndStreak3rd;"
	"RenderColorTarget=StreakMap3rd;	 Pass=Star3rdStreak1st;"
	"RenderColorTarget=StreakMap3rdTemp; Pass=Star3rdStreak2nd;"
	"RenderColorTarget=StreakMap3rd;	 Pass=Star3rdStreak3rd;"
	"RenderColorTarget=StreakMap4th;	 Pass=Star4thStreak1st;"
	"RenderColorTarget=StreakMap4thTemp; Pass=Star4thStreak2nd;"
	"RenderColorTarget=StreakMap4th;	 Pass=Star4thStreak3rd;"
#endif
	"RenderColorTarget=BloomMap1st;		Pass=GlareLightComp;"
#if HDR_FLARE_MODE
	"RenderColorTarget=BloomMap1stTemp; Pass=GhostImage1st;"
	"RenderColorTarget=BloomMap1st;		Pass=GhostImage2nd;"
#endif
#endif

#if AA_QUALITY == 0
#if POST_MOTION_BLUR_ENABLE && POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp2; Pass=HDRTonemapping;"
#elif POST_MOTION_BLUR_ENABLE || POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp; Pass=HDRTonemapping;"
#else
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=HDRTonemapping;"
#endif
#else
	"RenderColorTarget=ShadingMapTemp; Pass=HDRTonemapping;"
#endif

#if AA_QUALITY == 1
#if POST_MOTION_BLUR_ENABLE && POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp2; Pass=FXAA;"
#elif POST_MOTION_BLUR_ENABLE || POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp; Pass=FXAA;"
#else
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=FXAA;"
#endif
#endif

#if AA_QUALITY == 2 || AA_QUALITY == 3
	"RenderColorTarget=SMAAEdgeMap;  Clear=Color; Pass=SMAAEdgeDetection;"
	"RenderColorTarget=SMAABlendMap; Clear=Color; Pass=SMAABlendingWeightCalculation;"
#if POST_MOTION_BLUR_ENABLE && POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp2; Pass=SMAANeighborhoodBlending;"
#elif POST_MOTION_BLUR_ENABLE || POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp; Pass=SMAANeighborhoodBlending;"
#else
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=SMAANeighborhoodBlending;"
#endif
#endif

#if AA_QUALITY == 4 || AA_QUALITY == 5
	"RenderColorTarget=SMAAEdgeMap;  Clear=Color; Pass=SMAAEdgeDetection1x;"
	"RenderColorTarget=SMAABlendMap; Clear=Color; Pass=SMAABlendingWeightCalculation1x;"
	"RenderColorTarget=ShadingMap; Pass=SMAANeighborhoodBlending;"

	"RenderColorTarget=SMAAEdgeMap;  Clear=Color; Pass=SMAAEdgeDetection2x;"
	"RenderColorTarget=SMAABlendMap; Clear=Color; Pass=SMAABlendingWeightCalculation2x;"
#if POST_MOTION_BLUR_ENABLE && POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp2; Pass=SMAANeighborhoodBlendingFinal;"
#elif POST_MOTION_BLUR_ENABLE || POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp; Pass=SMAANeighborhoodBlendingFinal;"
#else
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=SMAANeighborhoodBlendingFinal;"
#endif
#endif

#if AA_QUALITY == 6
	"RenderColorTarget0=TAAHistoryMap; RenderColorTarget1=TAADepthMap; Pass=TAAPass;"
	"RenderColorTarget1=;"
#if POST_MOTION_BLUR_ENABLE && POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp2; Pass=TAAFinal;"
#elif POST_MOTION_BLUR_ENABLE || POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp; Pass=TAAFinal;"
#else
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=TAAFinal;"
#endif
#endif

#if AA_QUALITY == 7
#if POST_MOTION_BLUR_ENABLE && POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp2; Pass=GDLAAPass;"
#elif POST_MOTION_BLUR_ENABLE || POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp; Pass=GDLAAPass;"
#else
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=GDLAAPass;"
#endif
#endif

#if POST_MOTION_BLUR_ENABLE
#if POST_SHARPEN_ENABLE
	"RenderColorTarget=ShadingMapTemp; Pass=PostProcessMotionBlur;"
	"RenderColorTarget=TAAMatrixMap; Pass=TAAMatrixUpdatePass;"
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=PostProcessSharpen;"
#else
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=PostProcessMotionBlur;"
	"RenderColorTarget=TAAMatrixMap; Pass=TAAMatrixUpdatePass;"
#endif
#else
#if AA_QUALITY == 6 || AO_TEMPORAL_DENOISE
	"RenderColorTarget=TAAMatrixMap; Pass=TAAMatrixUpdatePass;"
#endif
#if POST_SHARPEN_ENABLE
	"RenderColorTarget=; RenderDepthStencilTarget=; Pass=PostProcessSharpen;"
#endif
#endif
;>
{
#if SUN_LIGHT_ENABLE && SUN_SHADOW_QUALITY
	pass ShadowMapGen<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ShadowMapGenPS();
	}
#if SHADOW_BLUR_COUNT
	pass ShadowBlurX<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ShadowMapBlurPS(ShadowMapSamp, float2(ViewportOffset2.x, 0.0f));
	}
	pass ShadowBlurY<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ShadowMapBlurPS(ShadowMapSampTemp, float2(0.0f, ViewportOffset2.y));
	}
#endif
#endif
#if SSDO_QUALITY && (IBL_QUALITY || SUN_LIGHT_ENABLE)
	pass SSDO<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceDirOccPassVS();
		PixelShader  = compile ps_3_0 ScreenSpaceDirOccPassPS();
	}
	pass SSDOBlurX<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ScreenSpaceDirOccBlurPS(SSDOMapSamp, float2(ViewportOffset2.x, 0.0f));
	}
	pass SSDOBlurY<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ScreenSpaceDirOccBlurPS(SSDOMapSampTemp, float2(0.0f, ViewportOffset2.y));
	}
#if AO_TEMPORAL_DENOISE
	pass SSDOTemporalDenoise<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSDOTemporalDenoisePS();
	}
	pass SSDOCopyTemporal<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSDOCopyTemporalPS();
	}
#endif
#endif
	pass ShadingOpacity<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ShadingOpacityPS();
	}
	pass ShadingTransparent<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ShadingTransparentPS();
	}
#if SSSS_QUALITY
	pass SSSSStencilTest<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		ColorWriteEnable = false;
		StencilEnable = true;
		StencilFunc = ALWAYS;
		StencilRef = 1;
		StencilPass = REPLACE;
		StencilFail = KEEP;
		StencilZFail = KEEP;
		StencilWriteMask = 1;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSSSStencilTestPS();
	}
	pass SSSSBlurX<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		StencilEnable = true; StencilFunc = EQUAL; StencilRef = 1; StencilWriteMask = 0;
		VertexShader = compile vs_3_0 SSSGaussBlurVS();
		PixelShader  = compile ps_3_0 SSSGaussBlurPS(ShadingMapTempSamp, ShadingMapTempSamp, float2(1.0, 0.0));
	}
	pass SSSSBlurY<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		StencilEnable = true; StencilFunc = EQUAL; StencilRef = 1; StencilWriteMask = 0;
		VertexShader = compile vs_3_0 SSSGaussBlurVS();
		PixelShader  = compile ps_3_0 SSSGaussBlurPS(ShadingMapSamp, ShadingMapTempSamp, float2(0.0, 1.0));
	}
	pass ShadingOpacityAlbedo<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		SrcBlend = DESTCOLOR; DestBlend = ZERO;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ShadingOpacityAlbedoPS();
	}
	pass ShadingOpacitySpecular<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		SrcBlend = ONE; DestBlend = ONE;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ShadingOpacitySpecularPS();
	}
#endif
#if OUTLINE_QUALITY == 2
	pass EdgeEdgeDetection<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 EdgeEdgeDetectionVS();
		PixelShader  = compile ps_3_0 EdgeLumaEdgeDetectionPS(OutlineMapSamp);
	}
	pass EdgeBlendingWeightCalculation<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 EdgeBlendingWeightCalculationVS();
		PixelShader  = compile ps_3_0 EdgeBlendingWeightCalculationPS(0.0);
	}
	pass EdgeNeighborhoodBlending<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 EdgeNeighborhoodBlendingVS();
		PixelShader  = compile ps_3_0 EdgeNeighborhoodBlendingPS(OutlineMapSamp, ViewportOffset2);
	}
#endif
#if TOON_ENABLE == 2
	pass DiffusionBlurX<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ScreenSpaceBilateralFilterPS(ShadingMapSamp, mDiffusionOffsetX);
	}
	pass DiffusionBlurY<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		SrcBlend = SRCALPHA; DestBlend = INVSRCALPHA;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ScreenSpaceBilateralFilterPS(ShadingMapTempSamp, mDiffusionOffsetY);
	}
#endif

#if SSR_QUALITY
	pass SSR_Trace<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSR_TracePS();
	}
	pass SSR_BlurX1<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX1Samp, SSRBlurStepX1);
	}
	pass SSR_BlurY1<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX1SampTemp, SSRBlurStepY1);
	}
	pass SSR_BlurX2<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX1Samp, SSRBlurStepX2);
	}
	pass SSR_BlurY2<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2.x * 2);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX2SampTemp, SSRBlurStepY2);
	}
	pass SSR_BlurX3<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2.x * 2);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX2Samp, SSRBlurStepX3);
	}
	pass SSR_BlurY3<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2.x * 4);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX3SampTemp, SSRBlurStepY3);
	}
	pass SSR_BlurX4<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2.x * 4);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX3Samp, SSRBlurStepX4);
	}
	pass SSR_BlurY4<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(ViewportOffset2.x * 8);
		PixelShader  = compile ps_3_0 SSR_FilterBlurPS(SSRLightX4SampTemp, SSRBlurStepY4);
	}
	pass SSR_Resolve<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		SrcBlend = ONE; DestBlend = INVSRCALPHA;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSR_ResolvePS();
	}
#endif
#if GI_ENABLE
	pass SSGI<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSGIPassPS();
	}
	pass SSGIPrefilter<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSGIOutlierRejectPS(SSGIMapSamp);
	}
	pass SSGIBlurX<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSGIBlurPS(SSGIMapSampTemp, 1.0f, 1.0f);
	}
	pass SSGIBlurY<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSGIBlurPS(SSGIMapSamp, 0.0f, 1.0f);
	}
	pass SSGIBlurX2<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSGIBlurPS(SSGIMapSampTemp, 1.0f, 2.0f);
	}
	pass SSGIBlurY2<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSGIBlurPS(SSGIMapSamp, 0.0f, 2.0f);
	}
	pass SSGIFinalCombine<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		SrcBlend = ONE; DestBlend = ONE;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 SSGIFinalCombinePS();
	}
#endif
#if BOKEH_MODE == 1
	pass ComputeFocalDistance<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ComputeFocalDistancePS();
	}
	pass ComputeBokehKernel<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ComputeBokehKernelPS();
	}
	pass ComputeBokehWeight<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ComputeBokehWeightVS();
		PixelShader  = compile ps_3_0 ComputeBokehWeightPS();
	}
	pass ComputeBokehNearPrefilter<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ComputeBokehPrefilterPS(ShadingMapPointSamp, _FocalCoCMap_TexelSize, -1.0f);
	}
	pass ComputeBokehFarPrefilter<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ComputeBokehPrefilterPS(ShadingMapPointSamp, _FocalCoCMap_TexelSize, 1.0f);
	}
	pass ComputeBokehFar<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(_FocalBokehMap_TexelSize.xy);
		PixelShader  = compile ps_3_0 ComputeBokehFarPS(FocalBokehMapPointSamp, FocalBokehMapLinearSamp, _FocalBokehMap_TexelSize);
	}
	pass ComputeBokehNear<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(_FocalBokehMap_TexelSize.xy);
		PixelShader  = compile ps_3_0 ComputeBokehNearPS(FocalBokehMapPointSamp, FocalBokehMapLinearSamp, _FocalBokehMap_TexelSize);
	}
	pass ComputeBilinearBlur<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(_FocalBokehMap_TexelSize.xy);
		PixelShader  = compile ps_3_0 ComputeBilinearBlurPS(FocalBokehNearMapLinearSamp, _FocalBokehMap_TexelSize);
	}
	pass ComputeBokehFinal<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		DestBlend = INVSRCALPHA; SrcBlend = SRCALPHA;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(float2(0.0f, 0.0f));
		PixelShader  = compile ps_3_0 ComputeBokehFinalPS();
	}
#elif BOKEH_MODE == 2
	pass ComputeFocalDistance<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ComputeFocalDistancePS();
	}
	pass ComputeBokehWeight<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ComputeBokehWeightVS();
		PixelShader  = compile ps_3_0 ComputeBokehWeightPS();
	}
	pass ComputeBokehNearPrefilter<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ComputeBokehPrefilterPS(ShadingMapPointSamp, _FocalCoCMap_TexelSize, -1.0f);
	}
	pass ComputeBokehFarPrefilter<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 ComputeBokehPrefilterPS(ShadingMapPointSamp, _FocalCoCMap_TexelSize, 1.0f);
	}
	pass ComputeHexagonalBlurX<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ComputeHexagonalBlurXVS(_FocalBokehMap_TexelSize);
		PixelShader  = compile ps_3_0 ComputeHexagonalBlurXPS(FocalBokehMapLinearSamp, _FocalBokehMap_TexelSize);
	}
	pass ComputeHexagonalBlurY<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ComputeHexagonalBlurYVS(_FocalBokehMap_TexelSize);
		PixelShader  = compile ps_3_0 ComputeHexagonalBlurYPS(FocalBlur1MapLinearSamp, FocalBlur2MapLinearSamp, _FocalBokehMap_TexelSize);
	}
	pass ComputeBokehBlur<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(_FocalBokehMap_TexelSize.xy);
		PixelShader  = compile ps_3_0 ComputeBokehBlurPS(FocalBokehMapPointSamp, FocalBokehMapLinearSamp, _FocalBokehMap_TexelSize);
	}
	pass ComputeBilinearBlur<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(_FocalBokehMap_TexelSize.xy);
		PixelShader  = compile ps_3_0 ComputeBilinearBlurPS(FocalBlur2MapLinearSamp, _FocalBokehMap_TexelSize);
	}
	pass ComputeBokehFinal<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		DestBlend = INVSRCALPHA; SrcBlend = SRCALPHA;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(float2(0.0f, 0.0f));
		PixelShader  = compile ps_3_0 ComputeBokehFinalPS();
	}
#endif
#if HDR_EYE_ADAPTATION
	pass EyeLum<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 EyeDownsampleVS(ViewportOffset2);
		PixelShader  = compile ps_3_0 EyeDownsamplePS(ShadingMapPointSamp);
	}
	pass EyeAdapation<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 EyeAdapationPS();
	}
#endif
#if HDR_BLOOM_MODE
	pass GlareDetection<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 GlareDetectionVS();
		PixelShader  = compile ps_3_0 GlareDetectionPS(ShadingMapPointSamp);
	}
#if HDR_STAR_MODE || HDR_FLARE_MODE
	pass HDRDownsample2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 HDRDownsampleVS(ViewportOffset2 * 2);
		PixelShader  = compile ps_3_0 HDRDownsample4XPS(DownsampleSamp1st);
	}
#endif
	pass BloomBlurX1<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset1);
		PixelShader  = compile ps_3_0 BloomBlurPS(DownsampleSamp1st, BloomOffsetX1);
	}
	pass BloomBlurY1<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset1);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp1stTemp, BloomOffsetY1);
	}
	pass BloomDownsampleX2<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 HDRDownsampleVS(BloomOffset1);
		PixelShader  = compile ps_3_0 HDRDownsamplePS(BloomSamp1st, BloomOffset1);
	}
	pass BloomBlurX2<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset2);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp2nd, BloomOffsetX2);
	}
	pass BloomBlurY2<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset2);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp2ndTemp, BloomOffsetY2);
	}
	pass BloomDownsampleX3<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 HDRDownsampleVS(BloomOffset2);
		PixelShader  = compile ps_3_0 HDRDownsamplePS(BloomSamp2nd, BloomOffset2);
	}
	pass BloomBlurX3<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset3);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp3rd, BloomOffsetX3);
	}
	pass BloomBlurY3<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset3);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp3rdTemp, BloomOffsetY3);
	}
	pass BloomDownsampleX4<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 HDRDownsampleVS(BloomOffset3);
		PixelShader  = compile ps_3_0 HDRDownsamplePS(BloomSamp3rd, BloomOffset3);
	}
	pass BloomBlurX4<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset4);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp4th, BloomOffsetX4);
	}
	pass BloomBlurY4<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset4);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp4thTemp, BloomOffsetY4);
	}
	pass BloomDownsampleX5<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 HDRDownsampleVS(BloomOffset4);
		PixelShader  = compile ps_3_0 HDRDownsamplePS(BloomSamp4th, BloomOffset4);
	}
	pass BloomBlurX5<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset5);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp5th, BloomOffsetX5);
	}
	pass BloomBlurY5<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadOffsetVS(BloomOffset5);
		PixelShader  = compile ps_3_0 BloomBlurPS(BloomSamp5thTemp, BloomOffsetY5);
	}
#if HDR_STAR_MODE == 1 || HDR_STAR_MODE == 2
	pass Star1stStreak1st<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, 0), 1);
		PixelShader  = compile ps_3_0 StarStreakPS(DownsampleSamp2nd, star_colorCoeff1st, mBloomStarFade);
	}
	pass Star1stStreak2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, 0), 4);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp1stTemp, star_colorCoeff2nd, 0);
	}
	pass Star1stStreak3rd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, 0), 16);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp1st, star_colorCoeff3rd, 0);
	}
	pass Star1stStreak4th<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, 0), 64);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp1stTemp, star_colorCoeff4th, 0);
	}
	pass Star2ndStreak1st<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, 0), 1);
		PixelShader  = compile ps_3_0 StarStreakPS(DownsampleSamp2nd, star_colorCoeff1st, mBloomStarFade);
	}
	pass Star2ndStreak2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, 0), 4);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp2ndTemp, star_colorCoeff2nd, 0);
	}
	pass Star2ndStreak3rd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, 0), 16);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp2nd, star_colorCoeff3rd, 0);
	}
	pass Star2ndStreak4th<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, 0), 64);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp2ndTemp, star_colorCoeff4th, 0);
	}
#endif
#if HDR_STAR_MODE == 3 || HDR_STAR_MODE == 4
	pass Star1stStreak1st<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, 0.9), 1);
		PixelShader  = compile ps_3_0 StarStreakPS(DownsampleSamp2nd, star_colorCoeff1st, mBloomStarFade);
	}
	pass Star1stStreak2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, 0.9), 4);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp1st, star_colorCoeff2nd, 0);
	}
	pass Star1stStreak3rd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, 0.9), 16);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp1stTemp, star_colorCoeff3rd, 0);
	}
	pass Star2ndStreak1st<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, 0.9), 1);
		PixelShader  = compile ps_3_0 StarStreakPS(DownsampleSamp2nd, star_colorCoeff1st, mBloomStarFade);
	}
	pass Star2ndStreak2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, 0.9), 4);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp2nd, star_colorCoeff2nd, 0);
	}
	pass Star2ndStreak3rd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, 0.9), 16);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp2ndTemp, star_colorCoeff3rd, 0);
	}
	pass Star3rdStreak1st<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, -0.9), 1);
		PixelShader  = compile ps_3_0 StarStreakPS(DownsampleSamp2nd, star_colorCoeff1st, mBloomStarFade);
	}
	pass Star3rdStreak2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, -0.9), 4);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp3rd, star_colorCoeff2nd, 0);
	}
	pass Star3rdStreak3rd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(0.9, -0.9), 16);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp3rdTemp, star_colorCoeff3rd, 0);
	}
	pass Star4thStreak1st<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, -0.9), 1);
		PixelShader  = compile ps_3_0 StarStreakPS(DownsampleSamp2nd, star_colorCoeff1st, mBloomStarFade);
	}
	pass Star4thStreak2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, -0.9), 4);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp4th, star_colorCoeff2nd, 0);
	}
	pass Star4thStreak3rd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 StarStreakVS(float2(-0.9, -0.9), 16);
		PixelShader  = compile ps_3_0 StarStreakPS(StreakSamp4thTemp, star_colorCoeff3rd, 0);
	}
#endif
#if HDR_FLARE_MODE
	pass GhostImage1st<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 GhostImageVS(ghost_scalar1st);
		PixelShader  = compile ps_3_0 GhostImagePS(DownsampleSamp2nd, BloomSamp2nd, BloomSamp2nd, ghost_modulation1st, mBloomStarFade);
	}
	pass GhostImage2nd<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		SrcBlend = ONE; DestBlend = ONE;
		VertexShader = compile vs_3_0 GhostImageVS(ghost_scalar2nd);
		PixelShader  = compile ps_3_0 GhostImagePS(BloomSamp1stTemp, BloomSamp1stTemp, BloomSamp2nd, ghost_modulation2nd, 0);
	}
#endif
	pass GlareLightComp<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = true; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		SrcBlend = ONE; DestBlend = SRCALPHA;
		VertexShader = compile vs_3_0 GlareLightCompVS();
		PixelShader  = compile ps_3_0 GlareLightCompPS();
	}
#endif
	pass HDRTonemapping<string Script= "Draw=Buffer;";>{
		 AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 HDRTonemappingVS();
		PixelShader  = compile ps_3_0 HDRTonemappingPS(ShadingMapPointSamp);
	}
#if AA_QUALITY == 1
	pass FXAA<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 FXAA3(ShadingMapTempSamp, ViewportOffset2);
	}
#endif
#if AA_QUALITY == 2 || AA_QUALITY == 3
	pass SMAAEdgeDetection<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAAEdgeDetectionVS();
		PixelShader  = compile ps_3_0 SMAALumaEdgeDetectionPS(ShadingMapTempSamp);
	}
	pass SMAABlendingWeightCalculation<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAABlendingWeightCalculationVS();
		PixelShader  = compile ps_3_0 SMAABlendingWeightCalculationPS(0.0);
	}
	pass SMAANeighborhoodBlending<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAANeighborhoodBlendingVS();
		PixelShader  = compile ps_3_0 SMAANeighborhoodBlendingPS(ShadingMapTempSamp, true);
	}
#endif
#if AA_QUALITY == 4 || AA_QUALITY == 5
	pass SMAAEdgeDetection1x<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAAEdgeDetectionVS();
		PixelShader  = compile ps_3_0 SMAALumaEdgeDetectionPS(ShadingMapTempSamp);
	}
	pass SMAABlendingWeightCalculation1x<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAABlendingWeightCalculationVS();
		PixelShader  = compile ps_3_0 SMAABlendingWeightCalculationPS(float4(1, 1, 1, 0));
	}
	pass SMAANeighborhoodBlending<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAANeighborhoodBlendingVS();
		PixelShader  = compile ps_3_0 SMAANeighborhoodBlendingPS(ShadingMapTempSamp, false);
	}
	pass SMAAEdgeDetection2x<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAAEdgeDetectionVS();
		PixelShader  = compile ps_3_0 SMAALumaEdgeDetectionPS(ShadingMapSamp);
	}
	pass SMAABlendingWeightCalculation2x<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAABlendingWeightCalculationVS();
		PixelShader  = compile ps_3_0 SMAABlendingWeightCalculationPS(float4(2, 2, 2, 0));
	}
	pass SMAANeighborhoodBlendingFinal<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 SMAANeighborhoodBlendingVS();
		PixelShader  = compile ps_3_0 SMAANeighborhoodBlendingPS(ShadingMapSamp, true);
	}
#endif
#if AA_QUALITY == 6
	pass TAAPass<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 TAAPS(ShadingMapTempSamp);
	}
#endif
#if AA_QUALITY == 6 || POST_MOTION_BLUR_ENABLE || AO_TEMPORAL_DENOISE
	pass TAAMatrixUpdatePass<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 TAAMatrixUpdatePS();
	}
#endif
#if POST_MOTION_BLUR_ENABLE
#if POST_SHARPEN_ENABLE
	pass PostProcessMotionBlur<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 PostProcessMotionBlurPS(ShadingMapTemp2Samp);
	}
#else
	pass PostProcessMotionBlur<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 PostProcessMotionBlurPS(ShadingMapTempSamp);
	}
#endif
#endif
#if AA_QUALITY == 6
	pass TAAFinal<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 TAAFinalPS();
	}
#endif
#if AA_QUALITY == 7
	pass GDLAAPass<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 GDLAA_PS(ShadingMapTempSamp, ViewportOffset2);
	}
#endif
#if POST_SHARPEN_ENABLE
	pass PostProcessSharpen<string Script= "Draw=Buffer;";>{
		AlphaBlendEnable = false; AlphaTestEnable = false;
		ZEnable = false; ZWriteEnable = false;
		VertexShader = compile vs_3_0 ScreenSpaceQuadVS();
		PixelShader  = compile ps_3_0 PostProcessSharpenPS(ShadingMapTempSamp, ViewportOffset2);
	}
#endif
}


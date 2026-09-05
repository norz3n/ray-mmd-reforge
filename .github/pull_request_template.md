## Description

Briefly describe the changes introduced in this pull request and the motivation behind them.

Fixes #(issue number, if applicable)

---

## Type of Change

- [ ] 🐛 **Bug fix** (non-breaking change fixing a shader or controller defect)
- [ ] ✨ **New feature** (non-breaking change adding a new shader effect, BRDF model, or preset)
- [ ] ⚡ **Performance optimization** (improving framerate, reducing VRAM / render target overhead)
- [ ] 🎨 **Material / Preset addition** (new material preset or toon shader variant)
- [ ] ♻️ **Refactoring** (code simplification without changing visual output)
- [ ] 📝 **Documentation** (updates to README, Wiki, or inline HLSL docstrings)

---

## Technical & Graphics Impact

- **Shader Profile**: Direct3D 9 Shader Model 3.0 (`ps_3_0`)
- **Render Targets**: Does this PR add, remove, or change any RenderTarget textures? (If yes, specify formats and VRAM impact)
- **Controller Morphs**: Are any morphs in `ray_controller.pmx`, `CausticsController.pmx`, or `DebugController.pmx` added or changed?
- **Macro Switches**: Are new macros added to `ray.conf` or `ray_advanced.conf`?

---

## Testing & Hardware Verification

- **GPU & Driver**: (e.g., NVIDIA RTX 4070 / GeForce Driver 572.16)
- **Environment**: MikuMikuDance 9.26/9.32 x64 + MikuMikuEffect 0.37 x64
- **Tested Scenarios**: (e.g., clean scene, heavy scene, MSAA disabled, window resize test)

### Visual Evidence (Before vs After)

| Before | After |
| :---: | :---: |
| *(Image or N/A)* | *(Image or N/A)* |

---

## Checklist

- [ ] My code strictly adheres to Direct3D 9 Shader Model 3.0 (`ps_3_0`) limits.
- [ ] All modified `.fx` and `.fxsub` files compile without errors or warnings in MME x64.
- [ ] No regression introduced to existing materials, lighting passes, or post-processes.
- [ ] Default values in `ray.conf` / `ray_advanced.conf` maintain sensible defaults out-of-the-box.
- [ ] Inline comments explain "why" specific math/tricks were chosen.

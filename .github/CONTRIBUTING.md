# Contributing to Ray-MMD Reforge

Thank you for your interest in contributing to **Ray-MMD Reforge**! This guide outlines how to submit bug reports, feature requests, and code contributions.

---

## 1. Reporting Issues

Before creating an issue, please check:
- The [Wiki Documentation](https://github.com/norz3n/ray-mmd-reforge/wiki) and [Troubleshooting & FAQ](https://github.com/norz3n/ray-mmd-reforge/wiki/Troubleshooting-&-FAQ).
- Existing open and closed [Issues](https://github.com/norz3n/ray-mmd-reforge/issues) to avoid duplicates.

When opening an issue, please use our **Issue Forms**:
- **Bug Report**: Include your GPU model, driver version, MMD version (x64), MME version (x64), and exact MME error logs or screenshots.
- **Feature Request**: Explain the use case and provide shader references/papers if proposing graphics algorithms.
- **Troubleshooting**: Describe your setup and what steps you've already attempted.

---

## 2. Technical Guidelines for Code Contributions

Ray-MMD Reforge is built on top of Direct3D 9 for MikuMikuDance via MikuMikuEffect. All shader contributions must follow these technical constraints:

### Direct3D 9 & Shader Model 3.0 Constraints
- **Profile**: All pixel and vertex shaders target `ps_3_0` / `vs_3_0`.
- **Register Pressure**: SM 3.0 has strict limits on temporary registers (`r#`), constant registers (`c#`), and sampler registers (`s#`). Keep arithmetic dense and vectorized (`float4`).
- **Texture Samplers**: Avoid unbounded sampler allocations; reuse G-Buffer samplers and coordinate lookups where possible.
- **Precision**: Use `float` for coordinates, depth, and raymarching steps; use `half` where precision permits without banding.

### Coding Conventions
- **Clean HLSL**: Use descriptive variable names and consistent indentation (4 spaces or tabs consistent with surrounding file).
- **Self-Documenting & Comments**: Comments should explain *why* an algorithm or mathematical approximation was chosen (e.g., citing papers, formulas, or hardware workarounds).
- **Configuration Switches**: User-facing switches belong in `ray.conf` or `ray_advanced.conf` with clear `#define` descriptions and safe defaults.
- **Backward Compatibility**: Do not arbitrarily break existing PMX controller bindings or material preset definitions unless part of a planned major version bump.

---

## 3. Pull Request Process

1. **Fork the repository** and create your branch from `master` (or current active development branch).
2. **Test locally**:
   - Verify that your modified shaders compile cleanly in **MME x64** with no compile errors or register overflow popups.
   - Verify rendering at standard viewport resolutions and during window resizing.
   - Test with both default and custom `ray.conf` configurations.
3. **Submit your PR** using the provided [Pull Request Template](.github/pull_request_template.md).
4. Provide side-by-side visual comparisons (Before vs After) for visual changes.

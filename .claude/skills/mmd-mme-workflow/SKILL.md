---
name: mmd-mme-workflow
description: Use when Claude needs to coordinate an MMD/MME effect authoring workflow, especially requests involving MikuMikuEffect, MME, MMD shaders, .fx/.fxsub files, ray-mmd, sdPBR, ExcellentShadow, post effects, material shaders, render targets, or runtime debugging.
---

# MMD MME Workflow

## Overview

Coordinate MMD/MME effect creation as a staged workflow. Treat MME authoring as a rendering pipeline problem first, and a shader-writing task second.

## Stage Flow

Use the smallest set of stage skills needed for the request:

1. Use `mme-effect-assessment` to turn the user's visual goal into concrete rendering objectives and acceptance criteria.
2. Use `mme-effect-feasibility` to decide whether the effect should be standalone or built as a ray-mmd/sdPBR/ExcellentShadow-style extension.
3. Use `mme-implementation-planning` to choose files, render targets, pass order, controller inputs, and test points.
4. Use `mme-reference-lookup` when syntax, annotations, semantics, framework rules, or local documentation must be verified.
5. Use `mme-example-mining` before authoring whenever a similar effect likely exists locally.
6. Use `mme-authoring` only after the plan is specific enough to implement.
7. Use `mme-static-review` after writing or changing `.fx` or `.fxsub` files.
8. Use `mme-runtime-debugging` when MMD/MME reports errors, renders black/blank, draws in the wrong order, or behaves differently in ray-mmd/sdPBR/ExcellentShadow.

Do not skip assessment and feasibility for complex visual requests. For tiny edits to an existing shader, start at the relevant later stage but still verify the local reference if the change touches MME script annotations, render targets, or framework integration.

## Routing Rules

- If the user asks for "an effect like X" or describes a look, start with assessment.
- If the user names ray-mmd, sdPBR, ExcellentShadow, GBuffer, PBR, shadow maps, SSDO/SSAO, AutoLuminous, or controller PMX, run feasibility before writing code.
- If the user asks "how does this parameter work", "what is the syntax", or "which pass should I use", route to reference lookup.
- If the user provides an existing effect folder or asks to imitate a known effect, route to example mining before planning.
- If MME shows `some techniques cannot run on this hardware`, "black screen", missing texture/include errors, wrong draw order, or stale settings, route to runtime debugging.

## Shared References

Load only the files needed:

- `references/mme-core-reference.md`: MME semantics, annotations, scripts, encoding, and shader model constraints.
- `references/example-index.md`: local official and modern examples worth mining.
- `references/framework-notes.md`: ray-mmd ReForge (this repo), ExcellentShadow2, and sdPBR480 framework notes.
- `references/debug-checklist.md`: runtime and static debugging checklist.

If this skill is installed with the suite, sibling stage skills may refer back to these same references under the `mmd-mme-workflow/references` folder.

## Deliverable Contract

Keep stage outputs concise and handoff-ready:

- Assessment output: effect brief, target objects, visual success criteria, constraints, unknowns.
- Feasibility output: standalone/framework decision, risks, required shader model/resources/controllers, compatibility notes.
- Planning output: file layout, RT/pass graph, techniques, UI/controller parameters, example sources, test strategy.
- Authoring output: changed files, how they map to the plan, known limitations.
- Review/debug output: findings first, then fixes or next diagnostics.

## Hard Rules

- Prefer local documentation and examples before inventing MME syntax.
- Remember that `Draw=Geometry` and `Draw=Buffer` belong on pass `Script` annotations; technique `Script` strings orchestrate `RenderColorTarget`, `Pass`, `ScriptExternal`, `LoopByCount`, and related commands.
- Treat ray-mmd and sdPBR as frameworks, not ordinary single-file effects.
- Preserve framework file layout and include paths. In MME, a correct shader can still fail if it is attached to the wrong object, loaded in the wrong order, or compiled under the wrong encoding.

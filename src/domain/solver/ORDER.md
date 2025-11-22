# Constraint ordering

## Methodology

Constraints in HarmonyArch are not relaxed jacobians but are deterministically-solved through direct inference of explicit user intent.

This can be seen as constraining but allows for greater performance, and additionally permits explicit exceptions to perfectly sound norms, allowing faster development of basic architectural forms.

## Constraint Philosophy

Architectural constraints like **Plumb**, **Level**, and **Orthogonal** are **opt-out rather than opt-in**. This reflects architectural reality: buildings are typically vertical, horizontal, and rectangular by default. Users only need to explicitly opt out when creating special cases (sloped walls, non-rectangular plans, etc.). This sane default dramatically reduces the cognitive load and verbosity required for common architectural modeling.

The order is as follows:

1. Coincident (Rare)
2. Collinear (Projection)
3. Coplanar (Projection)
4. Boundary 
5. Equilateral
6. Equiangular
7. Plumb (Opt-out)
8. Level (Opt-out)
9. Orthogonal (Opt-out)

New points are valid or invalid. Deltas loop through this system, cascading changes through the same loop. For sane cases in our hierarchical pattern this will not lead to computational overload and will be much simpler than relaxing jacobians.

We have not yet allowed for curves, non-euclidean geometry, etc. but we will work with these in a similarly deterministic fashion once the cases arise.
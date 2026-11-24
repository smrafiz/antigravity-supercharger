---
name: design-review
description: Review UI for accessibility, visual hierarchy, and responsiveness. Read-only audit with measured values against WCAG 2.2 AA.
---

# UI Design & Accessibility Review

Review the target UI for accessibility, visual hierarchy, and responsiveness.

**Rule: numbers, not adjectives.** "Low contrast" is not an actionable finding; "#8a8a8a on #ffffff is 3.4:1, below the 4.5:1 required for body text" is. Every finding must name the element (selector, component, or `file:line`) and the exact measured value.

## 1. Scope & Rendered UI
- Target routes, pages, or components from arguments or recent UI diffs.
- Inspect viewports: **1440px**, **768px**, and **375px**.
- Keyboard-only navigation check: verify all interactive controls can be focused and triggered without a pointer.

## 2. Accessibility (WCAG 2.2 AA)
- **Contrast**: 4.5:1 for normal text, 3:1 for large text and UI components/focus indicators.
- **Keyboard Navigation**: Sensible tab order, visible focus rings, no keyboard traps.
- **Focus Visibility**: Focus indicators must not be obscured by sticky headers or overlays (2.4.11).
- **Target Size**: Minimum 24×24 CSS px target area or sufficient spacing (2.5.8).
- **Labels & Semantics**: Explicit labels for inputs, accessible names for icon buttons, meaningful alt text, valid heading hierarchy.
- **Motion**: Respect `prefers-reduced-motion` media queries.

## 3. Hierarchy & Usability
- Clear primary call-to-action per view.
- Consistent typography and spacing scale.
- Well-defined states: loading, empty, hover, active, disabled, error.
- Destructive actions require explicit confirmation.

## 4. Output Report Structure
```markdown
DESIGN REVIEW: [scope] · rendered: [yes — 1440/768/375 | no — code only]

BLOCKERS (fails WCAG A/AA on primary flow):
- [element — file:line] — [measured value vs requirement] — [impact]

HIGH:
- ...

MEDIUM:
- ...

NITS:
- ...

CONSISTENCY: [off-token hardcoded values — file:line]
```

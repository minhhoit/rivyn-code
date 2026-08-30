//! Embedded expert skills for Design, Programming/Vibe Coding, Security, and Marketing/Sales.
//!
//! These skills are compiled directly into the binary so they are available immediately
//! in any workspace without needing prior network installation or manual file creation.
//! Users and repos can override any builtin skill by providing a same-named skill markdown file.

use super::{parse_markdown, sanitize_name, Skill, SkillOrigin};

struct BuiltinDef {
    name: &'static str,
    raw_markdown: &'static str,
}

const BUILTIN_SKILLS: &[BuiltinDef] = &[
    // ==========================================
    // DESIGN SKILLS
    // ==========================================
    BuiltinDef {
        name: "modern-web-design",
        raw_markdown: r#"---
name: modern-web-design
description: World-class modern web UI/UX standards, typography hierarchy, fluid layouts, micro-interactions, dark/light theme tokens, and accessibility (WCAG AAA).
when: asked to design or polish UI, create landing pages, build web interfaces, style components, or refine aesthetics
---
# Modern Web Design Playbook

## 1. Visual Hierarchy & Typography
- **Font Stack**: Use modern, high-legibility geometric sans or grotesk typefaces (e.g., Inter, Geist, Plus Jakarta Sans, Outfit).
- **Scale**: Establish a harmonic type scale (`text-xs` up to `text-6xl`). High contrast between display headers and body text.
- **Line Height & Letter Spacing**: Tighten tracking on display headings (`tracking-tight`), relax tracking and increase leading on body text (`leading-relaxed`).

## 2. Spatial System & Layout
- **Grid & Bento Layouts**: Structure dashboard and landing content into clean, asymmetrical bento grids with rounded corners (`rounded-2xl` / `rounded-3xl`).
- **Surface Elevation**: Use layered surfaces with subtle border glows (`border border-white/10` or `border-neutral-200/80 dark:border-neutral-800/80`) rather than heavy drop shadows.
- **Micro-Gradients**: Accent cards with subtle radial gradients (`bg-gradient-to-tr from-primary/10 via-transparent to-transparent`).

## 3. Micro-Interactions & Polish
- **Transitions**: Apply smooth hardware-accelerated easing curves (`transition-all duration-200 ease-out`).
- **Interactive Feedback**: Scale buttons slightly on active (`active:scale-[0.98]`), illuminate borders on hover (`hover:border-primary/50`), and preserve crisp focus rings for keyboard navigation.
- **Empty States & Skeletons**: Provide animated pulse skeleton loaders rather than generic spinning wheels.

## 4. Color & Contrast Tokens
- **Semantic Palette**: Define tokens for `surface`, `surface-elevated`, `surface-muted`, `accent-primary`, `accent-secondary`, `status-success`, `status-danger`.
- **Contrast Ratios**: Strictly adhere to WCAG AAA contrast (minimum 7:1 for normal text, 4.5:1 for large display text).
"#,
    },
    BuiltinDef {
        name: "generative-ui",
        raw_markdown: r#"---
name: generative-ui
description: Best practices for creating interactive, stateful, dynamic UI widgets, charts, and diagrams directly in conversations or web artifacts.
when: asked to build interactive HTML/SVG components, Mermaid/KaTeX visualizers, generative UI artifacts, or dynamic widgets
---
# Generative UI & Interactive Widgets Playbook

## 1. Standalone Single-File Architecture
- Generate completely self-contained components containing HTML markup, Tailwind CSS classes, and modular vanilla JavaScript.
- Avoid external runtime dependencies unless using CDN-pinned libraries (e.g., Lucide icons, Chart.js, Mermaid, KaTeX).

## 2. Interactive State & Visual Reactivity
- **State Handling**: Implement reactive data models in plain JS using `Proxy` or custom event dispatchers.
- **Dynamic Controls**: Provide sliders, segmented controls, tabs, filter pills, and live search inputs to let the user manipulate data parameters on the fly.
- **Live Visuals**: Immediately update SVG graphs, DOM elements, or canvas charts upon input modification without full page reload.

## 3. Responsive Container Adaptation
- Ensure components render flawlessly inside constrained chat modals, preview panels, and full-screen viewports.
- Utilize CSS container queries (`@container`) and flexible flex/grid wrapping to guarantee responsiveness.
"#,
    },
    BuiltinDef {
        name: "design-system-foundations",
        raw_markdown: r#"---
name: design-system-foundations
description: Architecture and structuring of scalable design systems, token architecture, component libraries, and cross-platform design guidelines.
when: building reusable design systems, establishing brand style guides, organizing component tokens, or creating UI component kits
---
# Scalable Design System Architecture

## 1. Token Taxonomy
- **Tier 1 - Global Primitive Tokens**: Base color hexes, spacing values, font families (e.g., `blue-500: #3b82f6`, `space-4: 1rem`).
- **Tier 2 - Semantic Tokens**: Contextual mappings (e.g., `color-brand-primary: var(--blue-500)`, `surface-card: var(--neutral-50)`).
- **Tier 3 - Component Tokens**: Component-specific variables (e.g., `button-primary-bg: var(--color-brand-primary)`).

## 2. Component Composition (Atomic Design)
- **Atoms**: Buttons, Badges, Icons, Inputs, Avatars.
- **Molecules**: SearchBar, FormField, UserDropdown, ToastMessage.
- **Organisms**: NavigationBar, DataTable, ModalDialog, HeroSection.
- **Templates & Pages**: Layout scaffolds and full views.

## 3. Component API Consistency
- **Standardized Props**: Keep prop names unified across all components (e.g., `variant`: `primary | secondary | outline | ghost`, `size`: `sm | md | lg`, `isDisabled`, `isLoading`).
- **Polymorphism**: Support custom root elements (e.g., `asChild` pattern or `as="a"`) for accessible link wrapping.
"#,
    },
    BuiltinDef {
        name: "responsive-tailwind-ui",
        raw_markdown: r#"---
name: responsive-tailwind-ui
description: Mobile-first responsive UI development with Tailwind CSS, container queries, modern CSS grid/flexbox patterns, and fluid typography.
when: styling with Tailwind CSS, creating responsive mobile-to-desktop layouts, fixing layout overflows, or optimizing CSS bundle
---
# Responsive Tailwind UI Playbook

## 1. Mobile-First Construction
- Always write mobile base styles first; layer progressive enhancements with `sm:`, `md:`, `lg:`, `xl:`, `2xl:`.
- Example: `<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">`.

## 2. Fluid Typography & Spacing
- Use `clamp()`-based fluid typography utilities or Tailwind fluid plugins for seamless scaling between mobile and desktop without abrupt breakpoint jumps.
- Utilize intrinsic sizing with `minmax()` and `auto-fit` / `auto-fill`: `grid-template-columns: repeat(auto-fit, minmax(280px, 1fr))`.

## 3. Dark Mode & High-DPI Adaptation
- Ensure all color tokens support dark mode seamlessly using `dark:` variants or CSS variable theme switching.
- Prevent layout shift (CLS) by explicitly reserving image and icon aspect ratios (`aspect-video`, `aspect-square`).
"#,
    },

    // ==========================================
    // PROGRAMMING & VIBE CODING SKILLS
    // ==========================================
    BuiltinDef {
        name: "vibe-coding-workflow",
        raw_markdown: r#"---
name: vibe-coding-workflow
description: Rapid iterative vibe coding methodology with AI: tight feedback loops, modular prototyping, test-driven validation, and seamless Vietnamese/English conversational prompts.
when: rapid prototyping, vibe coding, building MVPs, iterative pair programming with AI, exploring ideas quickly
---
# Vibe Coding & Rapid Prototyping Playbook

## 1. The Vibe Coding Loop
1. **Clear Mental Model**: Describe the desired outcome, user flow, and high-level architecture in natural language (Vietnamese or English).
2. **Atomic Scaffolding**: Build the smallest functional end-to-end slice first (stub UI -> stub API -> connect live data).
3. **Instant Verification**: Run tests, check compile status, and hot-reload changes after every atomic change.
4. **Iterative Polish**: Add micro-interactions, error boundaries, edge cases, and performance optimizations.

## 2. Bilingual Context Mastery (Vietnamese + English)
- Use natural Vietnamese for design rationale, feature descriptions, and high-level concepts (`"tạo layout bento grid đẹp mắt, hỗ trợ dark mode mượt mà, gõ tiếng Việt không bị mất dấu"`).
- Maintain precise English for code identifiers, types, function names, and technical commit messages.

## 3. Guardrails for Fast Iteration
- Never guess API signatures or break type checking; verify with LSP/compiler immediately.
- Keep components modular and single-responsibility so features can be added or reverted effortlessly.
"#,
    },
    BuiltinDef {
        name: "clean-architecture-patterns",
        raw_markdown: r#"---
name: clean-architecture-patterns
description: Clean Architecture, Hexagonal / Ports-and-Adapters, Domain-Driven Design (DDD), and SOLID principles for robust, decoupled software systems.
when: designing system architecture, structuring backend services, refactoring tangled codebases, implementing domain models or repositories
---
# Clean Architecture & DDD Playbook

## 1. Architectural Layers & Dependency Rule
- **Enterprise Domain Entities**: Core business objects, invariants, and value objects. ZERO external dependencies.
- **Application Use Cases**: Orchestrate business logic, command/query handlers, domain services.
- **Interface Adapters**: Controllers, presenters, gateways, repository implementations, DTO mappers.
- **Frameworks & Infrastructure**: Web frameworks, databases, network clients, UI rendering engines.
- **The Dependency Inversion Rule**: Dependencies must only point inward toward the core domain.

## 2. Repositories and Ports
- Define port interfaces in the application layer (`UserRepository` trait/interface).
- Implement adapters in infrastructure (`PostgresUserRepository`, `InMemoryUserRepository`).
- Test application use cases in complete isolation using mock/in-memory adapters without booting databases.
"#,
    },
    BuiltinDef {
        name: "fullstack-api-design",
        raw_markdown: r#"---
name: fullstack-api-design
description: Production-grade REST, GraphQL, and WebSocket API design standards, error envelope conventions, pagination, OpenAPI/Type-safe contract generation.
when: designing REST/gRPC/GraphQL APIs, implementing webhook handlers, creating API endpoints, or standardizing error formats
---
# Fullstack API Design Standards

## 1. RESTful URI Design & Methods
- Use plural nouns for resources: `/api/v1/workspaces/{workspace_id}/members`.
- Proper HTTP verbs: `GET` (idempotent read), `POST` (create), `PUT` (idempotent replace), `PATCH` (partial update), `DELETE` (remove).

## 2. Error Envelopes (RFC 7807 Standard)
```json
{
  "type": "https://api.example.com/errors/resource-not-found",
  "title": "Resource Not Found",
  "status": 404,
  "detail": "Project with id 'prj_123' does not exist in workspace.",
  "instance": "/api/v1/projects/prj_123",
  "code": "PROJECT_NOT_FOUND"
}
```

## 3. Pagination & Idempotency
- **Cursor Pagination**: Return `next_cursor`, `prev_cursor`, and `has_more` rather than offset-based pagination.
- **Idempotency**: Support `Idempotency-Key` headers for mutation requests (`POST /api/v1/payments`).
"#,
    },
    BuiltinDef {
        name: "rust-mastery",
        raw_markdown: r#"---
name: rust-mastery
description: Idiomatic Rust engineering, zero-cost abstractions, memory safety, async Tokio patterns, error handling with anyhow/thiserror, and lock-free concurrency.
when: writing Rust code, optimizing performance, debugging lifetime/borrow checker errors, designing async pipelines, or building CLI/TUI tools in Rust
---
# Idiomatic Rust Mastery Playbook

## 1. Ownership & Zero-Cost Abstractions
- Prefer passing references (`&str`, `&[T]`) over owning clones (`String`, `Vec<T>`) on read paths.
- Use `Cow<'a, str>` when data may be either borrowed or owned.
- Leverage the type-state pattern to eliminate invalid runtime states at compile time.

## 2. Async Runtime & Tokio Guidelines
- Never call blocking syscalls or heavy CPU loops inside async tasks; use `tokio::task::spawn_blocking`.
- Enforce channel backpressure with bounded channels (`tokio::sync::mpsc::channel(N)`).
- Handle graceful shutdown via `tokio::select!` and cancellation tokens.

## 3. Robust Error Architecture
- Use `thiserror` for library crates and domain error enums where callers inspect specific variants.
- Use `anyhow` for top-level application boundaries, CLIs, and background tasks.
"#,
    },

    // ==========================================
    // SECURITY SKILLS
    // ==========================================
    BuiltinDef {
        name: "security-audit-and-hardening",
        raw_markdown: r#"---
name: security-audit-and-hardening
description: Comprehensive application and infrastructure security audit, OWASP Top 10 mitigation, secret scanning, input sanitization, and attack surface minimization.
when: performing security audits, vulnerability assessments, hardening servers/APIs, preventing SQL injection/XSS/SSRF, or reviewing code for vulnerabilities
---
# Security Audit & Hardening Playbook

## 1. OWASP Top 10 Checklist & Defenses
- **Injection (SQL, Command, LDAP)**: Use parameterized queries, ORMs, and avoid shell interpolation.
- **Cross-Site Scripting (XSS)**: Strict Content-Security-Policy (CSP), context-aware HTML escaping.
- **Server-Side Request Forgery (SSRF)**: Validate and whitelist target URLs, disable loopback/private subnet resolution (`127.0.0.1`, `10.0.0.0/8`, `169.254.169.254`).
- **Insecure Direct Object Reference (IDOR)**: Enforce tenant-scoped database filters on every query.

## 2. Secrets & Credential Management
- Zero secrets committed to git repositories (`.env`, private keys, bearer tokens).
- Use environment variables or secret vaults (e.g., Vault, AWS Secrets Manager, GCP Secret Manager).

## 3. HTTP Security Headers
- `Strict-Transport-Security: max-age=63072000; includeSubDomains; preload`
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `Referrer-Policy: strict-origin-when-cross-origin`
"#,
    },
    BuiltinDef {
        name: "secure-auth-and-rbac",
        raw_markdown: r#"---
name: secure-auth-and-rbac
description: Secure authentication and authorization architectures: JWT / PASETO with key rotation, OAuth2.0 / OIDC, PKCE, multi-factor auth (MFA), and fine-grained RBAC/ABAC models.
when: implementing user login, session management, OAuth2 flows, JWT validation, role-based access control, or permissions systems
---
# Secure Authentication & RBAC/ABAC Playbook

## 1. Password & Credential Hashing
- Use **Argon2id** (minimum memory 64MB, iterations 3) or **bcrypt** (cost 12+).
- Enforce rate-limiting on authentication endpoints (e.g., max 5 attempts per IP/account per minute).

## 2. Token & Session Architecture
- **Short-Lived Access Tokens**: Lifetime of 5 to 15 minutes.
- **Rotating Refresh Tokens**: Store hash in database; on reuse detection, immediately revoke token family.
- **Cookies**: Always mark cookies `HttpOnly; Secure; SameSite=Lax` (or `Strict`).

## 3. Authorization (RBAC & ABAC)
- Define discrete permissions: `workspace:read`, `workspace:edit`, `project:deploy`, `billing:manage`.
- Group permissions into roles (`Owner`, `Admin`, `Member`, `Viewer`).
- Apply attribute-based controls (ABAC) for resource-level boundaries.
"#,
    },
    BuiltinDef {
        name: "dependency-vulnerability-scan",
        raw_markdown: r#"---
name: dependency-vulnerability-scan
description: Supply-chain security, automated CVE vulnerability scanning, dependency tree analysis, license compliance, and malicious package detection.
when: scanning dependencies for CVEs, updating vulnerable packages, auditing cargo/npm/pip lockfiles, or setting up automated supply chain security
---
# Dependency Vulnerability & Supply Chain Security

## 1. Automated Lockfile Auditing
- **Rust**: Run `cargo audit` and `cargo deny check advisories licenses bans`.
- **Node.js**: Run `npm audit --omit=dev` or `pnpm audit`.
- **Python**: Run `pip-audit` or `safety check`.

## 2. Dependency Pinning & Integrity
- Always commit lockfiles (`Cargo.lock`, `package-lock.json`, `pnpm-lock.yaml`, `poetry.lock`).
- Verify checksum hashes on installation.
- Audit package scripts (`preinstall`, `postinstall`, `build.rs`) for unauthorized network or filesystem access.
"#,
    },

    // ==========================================
    // MARKETING & SALES SKILLS
    // ==========================================
    BuiltinDef {
        name: "product-launch-and-gtm",
        raw_markdown: r#"---
name: product-launch-and-gtm
description: End-to-end Go-To-Market (GTM) strategy, Product Hunt / Hacker News launch playbooks, early adopter user acquisition, and viral growth loops.
when: planning a product launch, preparing GTM strategy, launching on Product Hunt/Reddit/HN, designing early access waitlists, or measuring product-market fit
---
# Product Launch & GTM Strategy Playbook

## 1. Pre-Launch Phase (T-minus 4 Weeks)
- **Waitlist Landing Page**: Clear hero value prop, animated demo GIF/video, one-click email capture.
- **Beta Cohort**: Onboard 20-50 high-intent power users; collect qualitative feedback and testimonial quotes.
- **Distribution Channels**: Identify niche subreddits, Discord communities, X/Twitter lists, and developer newsletters.

## 2. Launch Day Playbook
- **Product Hunt**: Schedule post for 00:01 PST Tuesday/Wednesday. Prepare high-res animated gallery GIFs, first-comment founder story, and hunter outreach.
- **Hacker News (Show HN)**: Post with straightforward, hype-free title (`Show HN: Aizen – A fast, single-binary AI coding assistant for pure vibe coding`).
- **Social Media Blast**: Thread explaining the problem, the solution, a 30-second walkthrough video, and a direct link.

## 3. Post-Launch Retention & Flywheels
- Instant onboarding magic moment: User achieves first successful outcome within 60 seconds.
- Viral referral loops: Invite team members to collaborate or share generated artifacts.
"#,
    },
    BuiltinDef {
        name: "high-converting-copywriting",
        raw_markdown: r#"---
name: high-converting-copywriting
description: Direct-response copywriting, persuasive value propositions, landing page headline formulas, email marketing sequences, and conversion rate optimization (CRO).
when: writing landing page copy, marketing emails, ad copy, value propositions, call-to-action (CTA) buttons, or sales pages
---
# High-Converting Copywriting Playbook

## 1. Hero Section Conversion Formula
- **The Hook / Headline**: State the primary transformation clearly without buzzwords (`"Build Fullstack Web Apps 10x Faster with AI Pair Programming"`).
- **The Subhead**: Explain the how, for whom, and key differentiator (`"Aizen gives developers an ultra-fast, local-first agent with instant context and zero latency."`).
- **Primary CTA**: Action-oriented and low-friction (`"Get Started Free – No Credit Card Required"`).
- **Social Proof**: User ratings, logos of trusted companies, or live counter.

## 2. Copywriting Frameworks
- **PAS (Problem - Agitate - Solution)**:
  - *Problem*: Traditional coding tools lag on multilingual input and eat machine resources.
  - *Agitate*: Broken accents, high memory usage, and constant context loss kill your developer flow.
  - *Solution*: A single static Rust binary that renders instantly and natively understands your workflow.
- **AIDA (Attention - Interest - Desire - Action)**: Move the reader from curiosity to immediate action.

## 3. CTA & Microcopy Optimization
- Replace generic `"Submit"` with value-rich copy (`"Claim Your Free Access"`, `"Launch Project Now"`).
- Address friction points right below the button (`"✓ Free 14-day trial  ✓ Cancel anytime  ✓ 2-minute setup"`).
"#,
    },
    BuiltinDef {
        name: "developer-marketing-and-seo",
        raw_markdown: r#"---
name: developer-marketing-and-seo
description: Technical developer marketing, programmatic SEO, technical documentation excellence, GitHub README optimization, and open-source growth.
when: creating developer marketing campaigns, writing technical blog posts, optimizing SEO for docs/landing pages, or boosting GitHub repo traction
---
# Developer Marketing & Technical SEO

## 1. GitHub README Architecture
- **Banner & Badges**: Clean SVG badges for build status, license, version, Discord.
- **One-Liner Hook**: Clear summary of what the tool does and why it's better.
- **Visual Demo**: High-quality SVG/GIF terminal or UI animation showing the product in action.
- **Quickstart**: Copy-pasteable 1-line installation snippet (`curl -fsSL ... | sh` or `cargo install ...`).
- **Feature Matrix / Comparison Table**: Benchmark vs alternatives showing performance, features, and advantages.

## 2. Technical Content & Programmatic SEO
- Publish in-depth tutorials solving real problems ("How to build X with Y").
- Create comparison landing pages ("Aizen vs Alternative: Feature & Speed Comparison").
- Structure metadata with JSON-LD (`SoftwareApplication`, `TechArticle`), OpenGraph preview cards, and semantic HTML tags.
"#,
    },
    BuiltinDef {
        name: "b2b-saas-sales-playbook",
        raw_markdown: r#"---
name: b2b-saas-sales-playbook
description: B2B SaaS sales processes, MEDDPICC deal qualification, outbound email sequences, product demos, enterprise pricing tiers, and objection handling.
when: closing B2B sales deals, qualifying enterprise leads, drafting cold outreach emails, structuring SaaS pricing tiers, or negotiating contracts
---
# B2B SaaS Sales & Enterprise Playbook

## 1. MEDDPICC Deal Qualification
- **Metrics**: Quantifiable economic impact (e.g., saving 15 engineering hours/week).
- **Economic Buyer**: The person with ultimate budget sign-off authority (CTO, VP of Eng).
- **Decision Criteria**: Technical and commercial requirements (security compliance, latency, cost).
- **Decision Process**: Evaluation roadmap and approval workflow.
- **Paper Process**: Legal, procurement, MSA, and DPA timelines.
- **Identified Pain**: Critical business bottlenecks if they don't buy.
- **Champion**: Internal advocate driving the sale from within the prospect's team.
- **Competition**: Alternative tools or in-house solutions.

## 2. SaaS Pricing Tier Architecture
- **Starter / Free Tier**: Individual adoption, quick time-to-value, community support.
- **Pro Tier**: Advanced capabilities, higher rate limits, priority support.
- **Team / Enterprise Tier**: SSO/SAML, SOC2 compliance, dedicated success manager, custom SLAs.

## 3. High-Converting Discovery Call Structure
- First 3 mins: Rapport & agenda setting.
- Next 10 mins: Discovery questions focusing on current workflow pain points.
- Next 10 mins: Tailored product demo addressing specifically uncovered pain points.
- Final 7 mins: Clear next steps and scheduling follow-up milestone.
"#,
    },

    // ==========================================
    // RIVYN COGNITIVE OS, BUSINESS & STRATEGY SKILLS
    // ==========================================
    BuiltinDef {
        name: "alex-hormozi-perspective",
        raw_markdown: r#"---
name: alex-hormozi-perspective
description: Alex Hormozi's thinking framework: Value Equation, Grand Slam Offers, $100M Leads Core Four, Rule of 100, Trim & Stack, and pricing leverage.
when: evaluating business offers, pricing strategy, lead generation, customer acquisition, conversion bottlenecks, or scaling business models
---
# Alex Hormozi · Offer & Growth Framework

## 1. The Value Equation
- **Formula**: `Value = (Dream Outcome × Perceived Likelihood of Achievement) / (Time Delay × Effort & Sacrifice)`.
- Maximize top variables: Paint vivid dream outcome; stack ironclad proof/guarantees to raise perceived likelihood to ~100%.
- Minimize bottom variables: Cut time delay to instant gratification; eliminate user friction and effort down to near zero.

## 2. Grand Slam Offer Architecture
- **Step 1 - Identify Obstacles**: List every objection and physical constraint preventing the client from reaching the dream outcome.
- **Step 2 - Solutions & Trim/Stack**: Create high-value, low-cost delivery vehicles for every obstacle. Trim low-impact items and stack high-leverage assets.
- **Step 3 - Unbeatable Guarantees**: Unconditional 30-day, Conditional (action-based), or Anti-guarantee (all sales final with premium exclusivity).
- **Step 4 - Pricing Anchor**: Price at 10x ROI of the economic value created. Never compete on price; compete on differentiated value.

## 3. Core Four Lead Generation
- **Four Channels**:
  1. Warm Outreach (1-to-1 existing network)
  2. Cold Outreach (1-to-1 targeted strangers)
  3. Free Content (1-to-many public media & social)
  4. Paid Advertising (1-to-many targeted traffic)
- **The Rule of 100**: Execute 100 primary actions every day for 100 days straight (100 cold outreaches, 100 mins content creation, or $100 ad spend).
"#,
    },
    BuiltinDef {
        name: "rivyn-skill-forge",
        raw_markdown: r#"---
name: rivyn-skill-forge
description: Cognitive OS distillation protocol: extract thinking frameworks, mental models, decision heuristics, and DNA from any world-class thinker (Jobs, Musk, Munger, Feynman, Hormozi, Naval).
when: creating persona skills, distilling experts, extracting mental models, analyzing how a leader thinks, or building custom agent playbooks
---
# Rivyn Skill Forge · Cognitive OS Distillation

## 1. Core Distillation Protocol
- Capture **HOW** they think (cognitive operating system), not just **WHAT** they said (quotes).
- Research 6 key dimensions:
  1. **Primary Writings & Speeches**: Canonical texts, interviews, letters, podcasts.
  2. **Core Mental Models**: The lenses through which they perceive problems.
  3. **Decision Heuristics**: Intuitive rules of thumb under extreme uncertainty.
  4. **Tone of Voice & Expression DNA**: Cadence, vocabulary, analogies, formatting.
  5. **Anti-Patterns & Hard Boundaries**: What they explicitly NEVER do.
  6. **Honest Limitations**: Where the framework breaks down or does not apply.

## 2. Extraction Taxonomy
- **Mental Model Lens**: One-liner definition + Grounded evidence + Practical application + Boundary limitation.
- **Decision Rules**: IF [Scenario] -> THEN [Action] -> UNLESS [Exception].
- **Persona Roleplay Rules**: First-person perspective, framework-first reasoning, numbers over adjectives, conclusion before explanation.
"#,
    },
    BuiltinDef {
        name: "steve-jobs-product-taste",
        raw_markdown: r#"---
name: steve-jobs-product-taste
description: Steve Jobs' product philosophy: radical focus, saying NO to 100 good ideas, end-to-end integration, insane simplicity, and uncompromising product taste.
when: reviewing product design, user experience, feature roadmaps, simplifying workflows, setting product strategy, or critiquing aesthetics
---
# Steve Jobs · Radical Focus & Product Taste

## 1. Radical Focus & The Art of Saying No
- "People think focus means saying yes to the thing you've got to focus on. But that's not what it means at all. It means saying no to the hundred other good ideas that there are."
- Kill mediocre features ruthlessly. A product with 3 insanely great capabilities crushes one with 30 half-baked features.

## 2. End-to-End Vertical Integration
- Deep synergy between hardware, software, user interface, and developer experience.
- Control the entire customer journey from the initial discovery to unboxing to daily workflows.

## 3. Insane Simplicity & User Delight
- Simplicity is not the absence of clutter; it is the ultimate sophistication.
- Eliminate extra buttons, settings, and cognitive load. Make the interface feel obvious and magical.
- Don't ask customers what they want in focus groups — invent what they cannot yet imagine and execute with obsessive precision.
"#,
    },
    BuiltinDef {
        name: "elon-musk-first-principles",
        raw_markdown: r#"---
name: elon-musk-first-principles
description: Elon Musk's first-principles thinking and 5-step engineering algorithm: calculate physical limits, question requirements, delete steps, accelerate cycle time.
when: solving difficult technical problems, cost reduction, bottleneck elimination, system optimization, or questioning complex requirements
---
# Elon Musk · First Principles & Engineering Algorithm

## 1. First-Principles Physics Limit
- Boil things down to the most fundamental truths and reason up from there, rather than reasoning by analogy.
- Calculate the theoretical minimum cost/actions: What are the raw materials and fundamental physical laws?
- If the current solution is >3x more complex or costly than the physical limit, there are massive inefficiencies to delete.

## 2. The 5-Step Engineering Algorithm
1. **Make requirements less dumb**: Every requirement must come with a specific person's name attached, not a department. Always question it, especially if it came from very smart people.
2. **Delete the part or process step**: If you are not adding back at least 10% of what you delete, you are not deleting enough.
3. **Simplify or optimize**: Never optimize something that should not exist in the first place.
4. **Accelerate cycle time**: Move faster, but only after steps 1–3 are done.
5. **Automate**: Only automate as the final step.
"#,
    },
    BuiltinDef {
        name: "charlie-munger-inversion",
        raw_markdown: r#"---
name: charlie-munger-inversion
description: Charlie Munger's multi-disciplinary latticework of mental models: inversion principle, avoiding stupidity, checklist of cognitive biases, and long-term compounding.
when: strategic risk assessment, avoiding costly mistakes, auditing decisions, identifying cognitive biases, or evaluating long-term business moats
---
# Charlie Munger · Inversion & Mental Models

## 1. The Inversion Principle
- "Invert, always invert: Turn a situation or problem upside down. Look at it backward. What happens if all our plans go wrong? Where don't we want to go, and how do you get there?"
- Instead of trying to be brilliant, focus on consistently avoiding standard stupidity.
- Ask: "What would guarantee the failure of this project or company?" and systematically eradicate every cause.

## 2. Latticework of Mental Models
- Draw models from psychology, microeconomics, biology, physics, and probability.
- Avoid the "Man with a Hammer" syndrome: don't force one discipline's tool on every problem.

## 3. Cognitive Misjudgment Checklist
- Audit plans against incentives, commitment/consistency bias, social proof, contrast misreaction, deprival super-reaction (loss aversion), and authority influence.
"#,
    },
    BuiltinDef {
        name: "naval-ravikant-strategy",
        raw_markdown: r#"---
name: naval-ravikant-strategy
description: Naval Ravikant's strategy for wealth creation: permissionless leverage (code & media), specific knowledge, judgment over effort, and serial compounding.
when: career strategy, building indie software products, content flywheel leverage, capital allocation, or escaping zero-sum games
---
# Naval Ravikant · Permissionless Leverage & Judgment

## 1. Forms of Leverage
- **Labor**: People working for you (oldest, highest friction, political).
- **Capital**: Money invested in assets and compounding.
- **Code & Media (Permissionless Leverage)**: Software and content work for you while you sleep. Zero marginal cost of replication. The leverage of the modern era.

## 2. Specific Knowledge & Authenticity
- Specific knowledge is knowledge you cannot be trained for. If society can train you, it can train someone else and replace you.
- Found at the intersection of your genuine curiosity and natural obsession.
- Escape competition through authenticity: "No one can compete with you on being you."

## 3. Serial Compounding vs Parallel Burnout
- All the real returns in life come from compound interest: in wealth, relationships, and knowledge.
- Focus on one high-leverage vehicle at a time. Compound serially rather than burning out across scattered parallel projects.
"#,
    },
    BuiltinDef {
        name: "uiux-designer",
        raw_markdown: r#"---
name: uiux-designer
description: Comprehensive UI/UX design masterclass: 50+ modern design styles, 97 harmonized color palettes, 57 font pairings, responsive layout rules, and WCAG AAA accessibility.
when: designing user interfaces, selecting typography and palettes, crafting design aesthetics (Bento, Glassmorphism, Brutalism, Minimalist, Dark Cyber), or reviewing UI/UX
---
# UI/UX Designer · Master Aesthetic & Systems Playbook

## 1. Aesthetic Styles Architecture
- **Bento Grid**: Asymmetric modular cards, subtle borders, high information density with breathing room.
- **Glassmorphism**: Layered backdrop blur (`backdrop-blur-md`), subtle translucent white borders (`border-white/10`), ambient radial glow.
- **Neo-Brutalism**: Bold solid borders (`border-2 border-black`), high contrast shadows (`shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]`), vibrant primary colors.
- **Minimalist Modern**: Generous whitespace, refined typographic scale, monochromatic neutral base with single high-impact accent.

## 2. Color Harmonization & Accessibility
- **60-30-10 Rule**: 60% dominant background/surface, 30% structural secondary/card, 10% high-intent accent CTA.
- **Contrast Ratios**: Strictly preserve WCAG AAA standards for legible readability across both light and dark modes.

## 3. Responsive Layout Hierarchy
- Mobile-first scaffolding with intrinsic grid columns (`grid-template-columns: repeat(auto-fit, minmax(280px, 1fr))`).
- Interactive tactile feedback on every clickable surface (scale transforms, subtle glow, visible focus rings).
"#,
    },
];

/// Return all embedded builtin skills.
#[allow(dead_code)]
pub fn list() -> Vec<Skill> {
    BUILTIN_SKILLS
        .iter()
        .map(|def| {
            let mut sk = parse_markdown(def.raw_markdown, def.name);
            sk.origin = SkillOrigin::Builtin;
            sk
        })
        .collect()
}

/// Load a specific builtin skill by name.
pub fn load(name: &str) -> Option<Skill> {
    let s = sanitize_name(name);
    BUILTIN_SKILLS.iter().find_map(|def| {
        if sanitize_name(def.name) == s {
            let mut sk = parse_markdown(def.raw_markdown, def.name);
            sk.origin = SkillOrigin::Builtin;
            Some(sk)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_builtin_skills_parse_and_load() {
        let skills = list();
        assert_eq!(skills.len(), 22, "should have 22 builtin skills");
        for sk in &skills {
            assert!(!sk.name.is_empty());
            assert!(!sk.description.is_empty(), "skill {} has empty description", sk.name);
            assert!(!sk.body.is_empty(), "skill {} has empty body", sk.name);
            assert_eq!(sk.origin, SkillOrigin::Builtin);

            let loaded = load(&sk.name).expect("must load by name");
            assert_eq!(loaded.name, sk.name);
            assert_eq!(loaded.description, sk.description);
        }
    }

    #[test]
    fn specific_skill_domains_exist() {
        // Design
        assert!(load("modern-web-design").is_some());
        assert!(load("generative-ui").is_some());
        assert!(load("design-system-foundations").is_some());
        assert!(load("responsive-tailwind-ui").is_some());
        assert!(load("uiux-designer").is_some());

        // Programming / Vibe Coding
        assert!(load("vibe-coding-workflow").is_some());
        assert!(load("clean-architecture-patterns").is_some());
        assert!(load("fullstack-api-design").is_some());
        assert!(load("rust-mastery").is_some());

        // Security
        assert!(load("security-audit-and-hardening").is_some());
        assert!(load("secure-auth-and-rbac").is_some());
        assert!(load("dependency-vulnerability-scan").is_some());

        // Marketing / Sales
        assert!(load("product-launch-and-gtm").is_some());
        assert!(load("high-converting-copywriting").is_some());
        assert!(load("developer-marketing-and-seo").is_some());
        assert!(load("b2b-saas-sales-playbook").is_some());

        // Rivyn Cognitive OS & Business Strategy
        assert!(load("alex-hormozi-perspective").is_some());
        assert!(load("rivyn-skill-forge").is_some());
        assert!(load("steve-jobs-product-taste").is_some());
        assert!(load("elon-musk-first-principles").is_some());
        assert!(load("charlie-munger-inversion").is_some());
        assert!(load("naval-ravikant-strategy").is_some());
    }
}

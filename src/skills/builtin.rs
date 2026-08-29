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
        assert_eq!(skills.len(), 15, "should have 15 builtin skills");
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
    }
}

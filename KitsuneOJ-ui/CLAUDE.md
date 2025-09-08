# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- `pnpm dev` - Start development server
- `pnpm build` - Create production build  
- `pnpm preview` - Preview production build
- `pnpm check` - Run Svelte type checking
- `pnpm check:watch` - Run Svelte type checking in watch mode
- `pnpm fmt` - Format code with Prettier
- `pnpm lint` - Check code formatting with Prettier

## Project Architecture

This is a SvelteKit application built with TypeScript and TailwindCSS, configured for Cloudflare deployment.

### Key Technologies
- **SvelteKit**: Full-stack framework with file-based routing
- **TypeScript**: Type safety throughout the application
- **TailwindCSS v4**: Utility-first CSS with Vite plugin integration
- **shadcn-svelte**: Component library based on shadcn/ui
- **bits-ui**: Accessible component primitives
- **ky**: HTTP client for API requests

### Project Structure
- `src/lib/` - Shared library code
  - `components/` - Reusable UI components (with shadcn-svelte structure)
  - `api/` - API client and configuration
  - `hooks/` - SvelteKit hooks
  - `utils.ts` - Utility functions
- `src/routes/` - SvelteKit file-based routing
- `static/` - Static assets
- `components.json` - shadcn-svelte configuration

### API Configuration
API calls use ky HTTP client with configuration in `src/lib/api/config.ts`. The API base URL is configured via `PUBLIC_API_URL` environment variable.

### Component System
Uses shadcn-svelte with component aliases configured in `components.json`:
- `$lib/components` - General components
- `$lib/components/ui` - shadcn-svelte UI components
- `$lib/utils` - Utility functions
- `$lib/hooks` - Custom hooks

### Deployment
Configured for Cloudflare Pages deployment using `@sveltejs/adapter-cloudflare`.
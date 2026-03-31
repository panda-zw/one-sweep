# ClearSpace - Product Brief

## The Problem

macOS users, especially developers, lose hundreds of gigabytes to invisible clutter: simulator runtimes, Docker images, dependency caches, old IDE data, and build artifacts. Apple's built-in storage breakdown is vague ("System Data: 231 GB") and offers no way to act on it.

Existing tools like CleanMyMac and DaisyDisk are either too broad (cleaning browser cookies instead of real space hogs) or too technical (showing raw folder trees). None of them understand developer workflows - they don't know what node_modules is, that Xcode simulators can be safely deleted, or that Docker images can be pruned.

ClearSpace fills this gap: a macOS desktop app that finds the real space hogs, explains them in plain language, and lets anyone - technical or not - safely reclaim storage without breaking anything.

---

## The User

**Primary persona**: A Mac user whose disk is filling up. They may or may not be a developer. They don't want to open a terminal. They want to understand what's eating their storage and confidently free it up.

**Secondary persona**: A developer who knows what node_modules is but doesn't want to spend 30 minutes running du commands and manually pruning Docker. They want one-click cleanup.

---

## Core Principles

1. **No jargon** - Every category and description is written in everyday language. "Coding project dependencies (safe to remove, re-downloaded when needed)" not "node_modules".

2. **Safety first** - The app must never delete something that causes data loss or breaks an application. Every action is reversible or regenerable. When in doubt, don't offer to delete it.

3. **Transparency** - The user always sees exactly what will be deleted, how much space it will free, and why it's safe before any action is taken. No silent cleanup.

4. **Instant value** - The very first scan should surface actionable findings. The user should be able to free significant space within 60 seconds of opening the app.

---

## What ClearSpace Scans

### Category 1: Safe to Remove (Green)

Items that are always regenerable. The app can confidently recommend these.

- **Dependency caches** - node_modules across all projects, pnpm store, npm cache, yarn cache, pip cache, .gradle, .expo, CocoaPods cache
- **Build artifacts** - Xcode derived data, Android build cache, Webpack/Vite output folders
- **IDE/Editor leftovers** - Old VS Code workspace storage, old JetBrains version data (keeping current), extension caches
- **Simulator/emulator runtimes** - Xcode iOS simulators and runtimes, Android AVD images
- **Container cleanup** - Unused Docker images, stopped containers, dangling volumes, build cache
- **System caches** - Homebrew cache, browser caches, SiriTTS cache, general ~/Library/Caches

### Category 2: Review Before Removing (Yellow)

Items that are probably not needed but the user should confirm.

- **Downloads folder** - Old files the user may have forgotten about
- **Large hidden folders** - .android, .cache, .local and other dotfiles that have grown large
- **Old application data** - Data left behind by apps that are no longer installed
- **Database dumps** - .sql and .dump files in the home directory or project folders
- **Duplicate projects** - Projects that appear to be copies or forks of each other

### Category 3: Be Careful (Red)

Items the app surfaces for awareness but does not offer one-click deletion.

- **Docker volumes with databases** - May contain development data
- **Project source code** - Never auto-suggested for deletion
- **Application support data** - Conversation history, saved credentials, user preferences
- **Anything the app cannot confidently classify**

---

## User Experience Flow

### 1. First Launch

- Welcome screen: "Let's find out what's taking up your space."
- Request necessary permissions (full disk access if needed).
- Begin automatic scan.

### 2. Scan Results

- Summary at the top: "We found X GB that can be safely freed."
- Visual breakdown (pie chart or bar) showing space by category.
- Categories listed below, each expandable to show individual items.
- Each item shows: friendly name, size, safety badge (green/yellow/red), one-line explanation.

### 3. Cleanup

- User checks/unchecks categories or individual items.
- "Free X GB" button at the bottom, always showing the live total.
- Confirmation dialog before any deletion: "This will remove [summary]. These items can be re-downloaded if needed. Continue?"
- Progress indicator during cleanup.
- Completion screen: "Done! You freed X GB."

### 4. Ongoing

- Optional scheduled scans (weekly/monthly) with notification: "You have X GB of clutter that can be cleaned."
- Menu bar icon showing current free space.
- Quick-clean from menu bar for green-tier items only.

---

## Technical Architecture

### Platform

macOS only (initially). Native desktop app.

### Recommended Stack

- **Tauri** (Rust + web frontend) - Lightweight, native-feeling, no Electron bloat. Fitting for a utility app about saving space.
- **Svelte or React** - For the frontend UI.
- **Swift helpers** - For operations requiring elevated privileges (deleting system-level simulator runtimes, accessing restricted Library paths).
- **SQLite** - Local database for scan history, tracking what was cleaned and when.

### Why Tauri over Electron

- App bundle size: ~5-10 MB vs ~150+ MB for Electron
- RAM usage: ~30 MB vs ~200+ MB for Electron
- An app about freeing space should not itself be bloated

### Core Modules

**Scanner Engine**
- Walks the filesystem using efficient methods (no full recursive walks where avoidable)
- Uses known paths for each cleanup category rather than scanning the entire disk
- Fingerprints items (e.g., detects node_modules by the presence of package.json in the parent)
- Calculates sizes asynchronously, updating the UI as results come in
- Caches scan results so returning to the app is instant

**Safety Classifier**
- Every detected item is classified into green/yellow/red based on rules
- Rules are defined declaratively (type, path pattern, conditions for safety)
- Items that don't match any rule default to red (never offer to delete unknowns)
- Special handling: if a project has uncommitted git changes, its node_modules drops from green to yellow with a warning

**Cleanup Engine**
- Executes deletions in a queue, one category at a time
- Logs every deletion to a local audit log (what, when, size, path)
- For Docker operations: calls Docker CLI or API
- For Xcode simulators: calls xcrun simctl
- For everything else: standard file deletion
- Reports progress back to the UI in real time

**Privilege Manager**
- Most cleanup operations work without elevated privileges
- For system-level paths (/Library/Developer), uses a Swift helper with SMJobBless or AppleScript authorization
- Never stores credentials; requests permission per session

---

## Cleanup Recipes

These are the specific operations the app performs, derived from real-world cleanup sessions.

### Node.js / JavaScript
- Find all node_modules directories → offer deletion (green)
- Find and clear npm cache (~/.npm/_cacache) → green
- Find and clear yarn cache (~/.yarn/cache, ~/Library/Caches/Yarn) → green
- Find and clear pnpm store (~/Library/pnpm/store) → green
- Run pnpm store prune if pnpm is installed → green
- Find and clear .expo cache → green

### Docker / Containers
- Check if Docker/OrbStack is running
- Show disk usage breakdown (images, containers, volumes, build cache)
- Offer "docker system prune -a" for unused images → green
- Offer "docker volume prune" for unused volumes → yellow (may contain data)
- Offer "docker builder prune" for build cache → green
- Note: OrbStack VM disk may not shrink automatically - advise compact

### Xcode / iOS Development
- Find simulator devices → offer "xcrun simctl delete all" → green
- Find simulator runtimes → offer "xcrun simctl runtime delete all" → green
- Find derived data (~/Library/Developer/Xcode/DerivedData) → green
- Find old Xcode archives → yellow

### Android Development
- Find AVD images (~/.android/avd) → yellow
- Find Gradle cache (~/.gradle/caches) → green
- Find Android SDK components that are outdated → yellow

### IDE / Editors
- Find old JetBrains version data (keep current, remove old) → green
- Find VS Code workspace storage that's grown large → yellow
- Find Cursor/other editor caches → green

### System / General
- Find and clear ~/Library/Caches contents → green
- Find Homebrew cache → green (brew cleanup --prune=all)
- Find old iOS backups (~/Library/Application Support/MobileSync) → yellow
- Find large files in Downloads older than 90 days → yellow

---

## What ClearSpace Does NOT Do

- **Never deletes source code or project files** - Only caches, dependencies, and build artifacts
- **Never runs without user confirmation** - Every deletion requires explicit approval
- **Never phones home** - No analytics, no telemetry, no cloud. Fully offline
- **Never stores or accesses sensitive data** - No reading file contents, only paths and sizes
- **Never claims more savings than real** - Sizes shown are actual disk usage, not estimated

---

## Revenue Model

- **Free tier** - Scan and see what's taking space. Clean green-tier items up to 10 GB per month.
- **One-time purchase ($9.99)** - Unlimited cleanup, scheduled scans, menu bar widget.
- No subscriptions. Users who pay once own it forever.

---

## Name Options

- ClearSpace (current working title)
- DiskSweep
- Declutter
- SpaceMaker
- FreeUp

---

## Success Metrics

- First scan completes in under 30 seconds
- Average user frees 20+ GB on first cleanup
- Zero reports of broken applications or data loss
- App bundle size under 15 MB
- RAM usage under 50 MB during scan

---

## Development Phases

### Phase 1 - MVP (Scanner + Green Cleanup)
- Build scanner engine for all green-tier categories
- Basic UI: scan results list with sizes and descriptions
- One-click cleanup for selected green items
- Audit log of all deletions
- macOS only, no App Store - direct download

### Phase 2 - Full Product
- Add yellow and red tier scanning
- Visual storage breakdown (charts)
- Scheduled scans with notifications
- Menu bar widget
- Smarter detection (git status awareness, last-accessed dates)
- Polished onboarding flow

### Phase 3 - Distribution
- Apple notarization and App Store submission
- Landing page and marketing site
- One-time purchase via App Store or Gumroad
- Auto-update mechanism

### Phase 4 - Expansion (Future)
- Windows support (AppData, npm, Docker equivalents)
- Linux support
- Team/enterprise version (IT departments managing dev machines)

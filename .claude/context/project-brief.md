# Project Brief: Hotkey Prompt Refiner

## Project Name
Hotkey Prompt Refiner

## Project Type
Desktop Application - Background Service

## Quick Summary
A lightweight, cross-platform Rust desktop application that enables AI-powered text processing via global hotkeys. Users copy text, press a configurable hotkey, and the clipboard content is sent to Claude API with a predetermined prompt. The AI response is automatically pasted at the cursor location.
It can handle tasks such as:
- prompt refinement
- decreasing prompt size while maintaining all relevant details to respect context
- adding cliche phrases: "Deep dive into social opinions. Avoid promotional posts." 
etc. 

## Scope

### In Scope
- Global hotkey registration and handling (cross-platform)
- Clipboard text capture on hotkey activation
- Direct HTTP integration with Anthropic Claude API
- Configurable prompt templates
- Automatic pasting of AI responses at cursor location
- Configuration via environment variables and config files
- Minimal resource footprint (<5MB binary, <30MB RAM)
- Support for macOS (primary), Windows, and Linux (secondary)

### Out of Scope
- GUI for configuration (CLI/file-based configuration only)
- Response history or logging
- Image processing or non-text clipboard content
- Custom hotkey configuration UI (requires code modification)

## Goals

### Primary Goals
1. **Seamless User Experience**: Enable instant AI-powered text processing without leaving current workflow
2. **Minimal Footprint**: Achieve <5MB binary size and <30MB RAM usage
3. **Fast Response Time**: <200ms latency from hotkey to API call initiation
4. **Cross-platform Support**: Work reliably on macOS, Windows, and Linux

### Secondary Goals
5. **Easy Configuration**: Simple setup via environment variables and text files
6. **Secure by Default**: No persistent storage of sensitive data
7. **Developer-friendly**: Clean architecture for future enhancements

## Key Objectives

### Technical Objectives
- ✅ Implement global hotkey system with macOS main thread compatibility
- ✅ Direct HTTP API integration using `reqwest` (no SDK dependencies)
- ✅ Clipboard integration using `arboard`
- ✅ Auto-paste using keyboard simulation (`enigo`)
- ✅ Configuration system with validation
- ✅ Complete workflow integration
- ✅ Comprehensive documentation
- ⏳ Performance optimization to meet targets

### User Experience Objectives
- Clear error messages and troubleshooting guidance
- Minimal setup requirements
- No learning curve for basic usage

### Performance Objectives
- Binary size: <5MB (release build)
- Idle RAM: <30MB
- Idle CPU: <1%
- Hotkey-to-API latency: <200ms

## Target Users

### Primary Audience
- Developers who frequently refine prompts for AI tools
- Technical writers who need regular AI assistance while writing
- Power users who want seamless AI integration in their workflow

### User Characteristics
- Comfortable with terminal/CLI tools
- Have access to Anthropic API key
- Work primarily in text-based applications
- Value efficiency and minimal context switching

## Success Criteria

### Must Have (MVP)
- ✅ Global hotkey successfully registers on all platforms
- ✅ Clipboard text captured accurately
- ✅ Claude API communication works reliably
- ✅ Responses auto-paste at cursor location
- ✅ Clear error handling and user feedback
- ✅ Documentation for setup and usage

### Should Have
- ⏳ Binary size <5MB
- ⏳ Memory usage <30MB idle
- ⏳ Tested on macOS, Windows, Linux
- Graceful handling of edge cases (empty clipboard, network errors)

### Could Have
- Configurable hotkeys via config file
- Multiple prompt templates
- Streaming responses for faster UX
- System tray integration

## Constraints

### Technical Constraints
- Must use Rust for performance and cross-platform support
- Direct HTTP API calls only (no SDK dependencies)
- macOS hotkey event loop must run on main thread
- Auto-paste requires Accessibility permissions on macOS

### Resource Constraints
- Solo developer project
- No dedicated testing infrastructure
- Limited access to all platform variants

### Security Constraints
- API keys must never be in source code
- No persistent storage of clipboard content
- All API communication over HTTPS
- No telemetry or user data collection

## Timeline

### Completed (Weeks 1-2)
- ✅ Project setup and dependencies
- ✅ Global hotkey system implementation
- ✅ Clipboard integration
- ✅ Configuration system
- ✅ Direct HTTP API integration
- ✅ Auto-paste system
- ✅ Complete workflow integration
- ✅ Documentation

### Current Phase (Week 2-3)
- ⏳ Cross-platform testing and refinement
- ⏳ Performance optimization

### Future Phases
- Multi-platform testing
- Community feedback integration
- Feature enhancements (configurable hotkeys, multiple templates)

## Key Stakeholders

### Project Owner
- Developer: Primary maintainer and contributor

### End Users
- AI prompt engineers
- Technical writers
- Software developers
- Power users seeking productivity tools

### External Dependencies
- Anthropic (API provider)
- Rust ecosystem (crate maintainers)

## Risks & Mitigation

### Technical Risks
- **Risk**: macOS main thread requirement complicates architecture
  - **Mitigation**: ✅ Implemented with `tao` event loop and worker thread for async

- **Risk**: Auto-paste may not work reliably across applications
  - **Mitigation**: ✅ Fallback to clipboard if paste fails (to be implemented)

- **Risk**: Hotkey conflicts with other applications
  - **Mitigation**: Clear error message, future: configurable hotkeys

### User Risks
- **Risk**: Users may not have API key
  - **Mitigation**: ✅ Clear documentation and validation with helpful error messages

- **Risk**: macOS Accessibility permissions confusing
  - **Mitigation**: ✅ Documentation with step-by-step instructions

## Metrics for Success

### Adoption Metrics
- GitHub stars and forks
- Issue reports and engagement

### Performance Metrics
- Binary size: Target <5MB
- Memory usage: Target <30MB idle
- CPU usage: Target <1% idle
- Response time: Target <200ms to API call

### Quality Metrics
- No crashes during normal operation
- Error rate <1% for valid operations
- Documentation completeness (all features covered)

## Communication Plan

### Documentation
- ✅ README.md with comprehensive setup and usage
- ✅ Inline code comments for complex logic
- CLAUDE.md for AI assistant guidance

### Community
- GitHub Issues for bug reports
- GitHub Discussions for feature requests
- Regular updates via commits and releases
